use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Represents a builtin function
#[derive(Debug, Clone)]
struct BuiltinFunction {
    name: String,   // print_int (used in codegen and C source)
    symbol: String, // __builtin_tpy_print_int (actual symbol in .o)
    module: String,
    return_type: String,
    params: Vec<String>, // Just types, no names needed
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let runtime_dir = manifest_dir.join("src/runtime");
    let builtins_dir = runtime_dir.join("builtins");
    let build_dir = runtime_dir.join("build");

    fs::create_dir_all(&build_dir).expect("Failed to create build directory");

    let llvm_prefix =
        env::var("LLVM_SYS_211_PREFIX").unwrap_or_else(|_| "/usr/lib/llvm-21".to_string());
    let clang = format!("{}/bin/clang", llvm_prefix);
    let llvm_nm = format!("{}/bin/llvm-nm", llvm_prefix);
    let llvm_dis = format!("{}/bin/llvm-dis", llvm_prefix);

    // Discover modules by scanning src/runtime/builtins/*.c
    let mut modules: Vec<String> = Vec::new();
    if let Ok(entries) = fs::read_dir(&builtins_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "c" {
                        let module_name = path.file_stem().unwrap().to_string_lossy().to_string();
                        modules.push(module_name);
                        println!("cargo:rerun-if-changed={}", path.display());
                    }
                }
            }
        }
    }

    // First pass: compile to temp to discover symbols
    let temp_dir = build_dir.join("temp");
    fs::create_dir_all(&temp_dir).expect("Failed to create temp directory");

    let mut all_symbols: Vec<String> = Vec::new();

    for module in &modules {
        let source_path = builtins_dir.join(format!("{}.c", module));
        let temp_object = temp_dir.join(format!("{}.o", module));

        let status = Command::new(&clang)
            .args(["-c", "-emit-llvm", "-flto", "-O2"])
            .arg("-o")
            .arg(&temp_object)
            .arg(&source_path)
            .status()
            .expect("Failed to run clang");

        if !status.success() {
            panic!("Failed to compile {} to bitcode", source_path.display());
        }

        // Extract symbols (exclude standard library functions)
        let output = Command::new(&llvm_nm)
            .arg(&temp_object)
            .output()
            .expect("Failed to run llvm-nm");

        if output.status.success() {
            let nm_output = String::from_utf8_lossy(&output.stdout);
            for line in nm_output.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let symbol_type = parts[parts.len() - 2];
                    let symbol = parts[parts.len() - 1];
                    // Only T (defined text) symbols, exclude common C library functions
                    if symbol_type == "T" && !is_stdlib_symbol(symbol) {
                        all_symbols.push(symbol.to_string());
                    }
                }
            }
        }
    }

    // Build -D flags to add __builtin_tpy_ prefix
    let rename_flags: Vec<String> = all_symbols
        .iter()
        .map(|s| format!("-D{}=__builtin_tpy_{}", s, s))
        .collect();

    // Second pass: compile with symbol renaming and extract signatures
    let mut all_functions: HashMap<String, Vec<BuiltinFunction>> = HashMap::new();

    for module in &modules {
        let source_path = builtins_dir.join(format!("{}.c", module));
        let object_path = build_dir.join(format!("{}.o", module));

        let mut cmd = Command::new(&clang);
        cmd.args(["-c", "-emit-llvm", "-flto", "-O2"]);
        for flag in &rename_flags {
            cmd.arg(flag);
        }
        cmd.arg("-o").arg(&object_path).arg(&source_path);

        let status = cmd.status().expect("Failed to run clang");
        if !status.success() {
            panic!("Failed to compile {} to bitcode", source_path.display());
        }

        // Use llvm-dis to get LLVM IR with function signatures
        let output = Command::new(&llvm_dis)
            .arg(&object_path)
            .arg("-o")
            .arg("-")
            .output()
            .expect("Failed to run llvm-dis");

        if !output.status.success() {
            panic!("llvm-dis failed on {}", object_path.display());
        }

        let ir_output = String::from_utf8_lossy(&output.stdout);
        let functions = parse_llvm_ir(&ir_output, module);
        all_functions.insert(module.clone(), functions);
    }

    let _ = fs::remove_dir_all(&temp_dir);

    // Generate Rust code
    let generated_code = generate_rust_code(&all_functions);
    let generated_path = out_dir.join("builtins_generated.rs");
    fs::write(&generated_path, generated_code).expect("Failed to write generated Rust code");

    println!(
        "cargo:rustc-env=TYPEPYTHON_BUILTIN_BUILD_DIR={}",
        build_dir.display()
    );
}

fn is_stdlib_symbol(symbol: &str) -> bool {
    // Common C library symbols to exclude
    matches!(
        symbol,
        "main"
            | "printf"
            | "puts"
            | "putchar"
            | "malloc"
            | "free"
            | "strlen"
            | "strcpy"
            | "strcat"
            | "strcmp"
            | "memcpy"
            | "memset"
            | "floor"
            | "pow"
            | "fmod"
            | "fabs"
            | "round"
    )
}

/// Parse LLVM IR to extract function signatures for __builtin_tpy_ functions
fn parse_llvm_ir(ir: &str, module: &str) -> Vec<BuiltinFunction> {
    let mut functions = Vec::new();

    for line in ir.lines() {
        // Look for function definitions: define ... @__builtin_tpy_xxx(...)
        if !line.starts_with("define ") {
            continue;
        }

        // Extract the function name
        let at_pos = match line.find('@') {
            Some(p) => p,
            None => continue,
        };

        let after_at = &line[at_pos + 1..];
        let paren_pos = match after_at.find('(') {
            Some(p) => p,
            None => continue,
        };

        let symbol = &after_at[..paren_pos];
        if !symbol.starts_with("__builtin_tpy_") {
            continue;
        }

        let name = symbol.strip_prefix("__builtin_tpy_").unwrap().to_string();

        // Extract return type - it's between "define" and "@"
        let before_at = &line[7..at_pos]; // Skip "define "
        let return_type = parse_llvm_return_type(before_at);

        // Extract parameter types from the function signature
        // Need to find matching closing paren, accounting for nested parens like captures(none)
        let params_start = at_pos + 1 + paren_pos + 1; // After the '('
        let remaining = &line[params_start..];

        // Handle empty parameter list ()
        let param_types = if remaining.starts_with(')') {
            Vec::new()
        } else {
            let params_end = find_matching_paren(remaining);
            if params_end == 0 {
                continue;
            }
            let params_str = &remaining[..params_end];
            parse_llvm_params(params_str)
        };

        functions.push(BuiltinFunction {
            name,
            symbol: symbol.to_string(),
            module: module.to_string(),
            return_type,
            params: param_types,
        });
    }

    functions
}

/// Find the position of the matching closing paren, accounting for nested parens
fn find_matching_paren(s: &str) -> usize {
    let mut depth = 1;
    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return i;
                }
            }
            _ => {}
        }
    }
    0 // Not found
}

/// Parse LLVM return type from the "define" line
fn parse_llvm_return_type(type_str: &str) -> String {
    // type_str might be: "dso_local void " or "dso_local i64 " or "dso_local double "
    let type_str = type_str.trim();

    // Find the actual type (last word before @)
    let parts: Vec<&str> = type_str.split_whitespace().collect();

    for part in parts.iter().rev() {
        match *part {
            "void" => return "void".to_string(),
            "i64" => return "i64".to_string(),
            "i32" => return "i32".to_string(),
            "double" => return "f64".to_string(),
            "float" => return "f32".to_string(),
            "i1" => return "bool".to_string(),
            "ptr" => return "ptr".to_string(),
            _ => continue,
        }
    }

    "void".to_string()
}

/// Parse LLVM parameter types from the parameter list
fn parse_llvm_params(params_str: &str) -> Vec<String> {
    if params_str.trim().is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();

    // Split by comma, but respect nested parentheses like captures(none)
    let params = split_params(params_str);

    for param in params {
        let param = param.trim();
        if param.is_empty() {
            continue;
        }

        // Parameter format: "type [attributes] %name" or just "type"
        // Examples: "i64 noundef %0", "ptr noundef %0", "i1 noundef zeroext %0"
        let parts: Vec<&str> = param.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let llvm_type = parts[0];
        let rust_type = match llvm_type {
            "i64" => "i64",
            "i32" => "i32",
            "double" => "f64",
            "float" => "f32",
            "i1" => "bool",
            "ptr" => "ptr",
            "..." => continue, // Varargs, skip
            _ => continue,
        };

        result.push(rust_type.to_string());
    }

    result
}

/// Split parameters by comma, respecting nested parentheses
fn split_params(s: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut depth = 0;
    let mut start = 0;

    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                result.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }

    // Don't forget the last parameter
    if start < s.len() {
        result.push(&s[start..]);
    }

    result
}

fn generate_rust_code(modules: &HashMap<String, Vec<BuiltinFunction>>) -> String {
    let mut code = String::new();

    code.push_str("// Auto-generated by build.rs - do not edit\n");
    code.push_str("use std::collections::HashMap;\n\n");

    code.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
    code.push_str("pub enum BuiltinType {\n");
    code.push_str("    Void,\n");
    code.push_str("    I64,\n");
    code.push_str("    I32,\n");
    code.push_str("    F64,\n");
    code.push_str("    F32,\n");
    code.push_str("    Bool,\n");
    code.push_str("    Ptr,\n");
    code.push_str("}\n\n");

    code.push_str("#[derive(Debug, Clone)]\n");
    code.push_str("pub struct BuiltinFunction {\n");
    code.push_str("    pub name: &'static str,\n");
    code.push_str("    pub symbol: &'static str,\n");
    code.push_str("    pub module: &'static str,\n");
    code.push_str("    pub return_type: BuiltinType,\n");
    code.push_str("    pub params: &'static [BuiltinType],\n");
    code.push_str("}\n\n");

    for (module, functions) in modules {
        let array_name = format!("{}_FUNCTIONS", module.to_uppercase());
        code.push_str(&format!(
            "pub static {}: &[BuiltinFunction] = &[\n",
            array_name
        ));

        for func in functions {
            let return_type = type_to_enum(&func.return_type);
            let params_array = generate_params_array(&func.params);

            code.push_str(&format!(
                "    BuiltinFunction {{\n        name: \"{}\",\n        symbol: \"{}\",\n        module: \"{}\",\n        return_type: {},\n        params: {},\n    }},\n",
                func.name, func.symbol, func.module, return_type, params_array
            ));
        }

        code.push_str("];\n\n");
    }

    code.push_str("pub static BUILTIN_MODULES: &[(&str, &[BuiltinFunction])] = &[\n");
    for module in modules.keys() {
        let array_name = format!("{}_FUNCTIONS", module.to_uppercase());
        code.push_str(&format!("    (\"{}\", {}),\n", module, array_name));
    }
    code.push_str("];\n\n");

    code.push_str("lazy_static::lazy_static! {\n");
    code.push_str(
        "    pub static ref BUILTIN_TABLE: HashMap<&'static str, &'static BuiltinFunction> = {\n",
    );
    code.push_str("        let mut map = HashMap::new();\n");
    code.push_str("        for (_, functions) in BUILTIN_MODULES {\n");
    code.push_str("            for func in *functions {\n");
    code.push_str("                map.insert(func.name, func);\n");
    code.push_str("            }\n");
    code.push_str("        }\n");
    code.push_str("        map\n");
    code.push_str("    };\n");
    code.push_str("}\n");

    code
}

fn type_to_enum(type_str: &str) -> &'static str {
    match type_str {
        "void" => "BuiltinType::Void",
        "i64" => "BuiltinType::I64",
        "i32" => "BuiltinType::I32",
        "f64" => "BuiltinType::F64",
        "f32" => "BuiltinType::F32",
        "bool" => "BuiltinType::Bool",
        "ptr" => "BuiltinType::Ptr",
        _ => panic!("Unknown type: {}", type_str),
    }
}

fn generate_params_array(params: &[String]) -> String {
    if params.is_empty() {
        return "&[]".to_string();
    }

    let params_str: Vec<&str> = params.iter().map(|t| type_to_enum(t)).collect();

    format!("&[{}]", params_str.join(", "))
}
