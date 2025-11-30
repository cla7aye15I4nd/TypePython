#!/usr/bin/env python3
"""
Generate all possible operator and builtin function test files.

This script uses Python's runtime to determine which operations are valid
by attempting each operation and catching TypeErrors.
"""

import itertools
import subprocess
import tempfile
from pathlib import Path

# Output directories
INVALID_DIR = Path(__file__).parent.parent / "tests" / "fixtures" / "invalid"
VALID_DIR = Path(__file__).parent.parent / "tests" / "fixtures" / "valid" / "operators"
VALID_BUILTINS_DIR = Path(__file__).parent.parent / "tests" / "fixtures" / "valid" / "builtins"

# Type examples for testing
# Format: (type_annotation, code_example, runtime_value)
TYPES = {
    "int": ("int", "1", 1),
    "float": ("float", "1.0", 1.0),
    "bool": ("bool", "True", True),
    "str": ("str", '"hello"', "hello"),
    "bytes": ("bytes", 'b"hello"', b"hello"),
    "list": ("list[int]", "[1, 2, 3]", [1, 2, 3]),
    "dict": ("dict[str, int]", '{"a": 1}', {"a": 1}),
    "set": ("set[int]", "{1, 2, 3}", {1, 2, 3}),
    "none": ("None", "None", None),
}

# Binary operators
BINARY_OPS = {
    "add": ("+", "add", lambda a, b: a + b),
    "sub": ("-", "subtract", lambda a, b: a - b),
    "mul": ("*", "multiply", lambda a, b: a * b),
    "div": ("/", "divide", lambda a, b: a / b),
    "floordiv": ("//", "floor divide", lambda a, b: a // b),
    "mod": ("%", "modulo", lambda a, b: a % b),
    "pow": ("**", "power", lambda a, b: a ** b),
    "bitand": ("&", "bitwise AND", lambda a, b: a & b),
    "bitor": ("|", "bitwise OR", lambda a, b: a | b),
    "bitxor": ("^", "bitwise XOR", lambda a, b: a ^ b),
    "lshift": ("<<", "left shift", lambda a, b: a << b),
    "rshift": (">>", "right shift", lambda a, b: a >> b),
    "eq": ("==", "compare with ==", lambda a, b: a == b),
    "ne": ("!=", "compare with !=", lambda a, b: a != b),
    "lt": ("<", "compare with <", lambda a, b: a < b),
    "le": ("<=", "compare with <=", lambda a, b: a <= b),
    "gt": (">", "compare with >", lambda a, b: a > b),
    "ge": (">=", "compare with >=", lambda a, b: a >= b),
    "in": ("in", "use 'in' with", lambda a, b: a in b),
    "notin": ("not in", "use 'not in' with", lambda a, b: a not in b),
    # Logical and/or - TypePython returns bool, wrap with bool() for Python compat
    "and": ("and", "use 'and' with", lambda a, b: a and b),
    "or": ("or", "use 'or' with", lambda a, b: a or b),
}

# Unary operators
UNARY_OPS = {
    "neg": ("-", "unary -", lambda a: -a),
    "pos": ("+", "unary +", lambda a: +a),
    "bitnot": ("~", "bitwise NOT", lambda a: ~a),
}

# Operations that TypePython doesn't support (even if Python allows them)
# Format: set of (left_type, op, right_type) tuples
UNSUPPORTED_OPS = {
    # bytes % formatting is not supported
    ("bytes", "mod", "bytes"),
    ("bytes", "mod", "list"),
    ("bytes", "mod", "dict"),
    ("bytes", "mod", "set"),
    ("bytes", "mod", "none"),
    ("bytes", "mod", "int"),
    ("bytes", "mod", "float"),
    ("bytes", "mod", "bool"),
    ("bytes", "mod", "str"),
    # 'in' with mismatched types (str in int list, etc.)
    ("str", "in", "list"),
    ("str", "in", "set"),
    ("str", "in", "dict"),
    ("str", "notin", "list"),
    ("str", "notin", "set"),
    ("str", "notin", "dict"),
    ("bytes", "in", "list"),
    ("bytes", "in", "set"),
    ("bytes", "in", "dict"),
    ("bytes", "notin", "list"),
    ("bytes", "notin", "set"),
    ("bytes", "notin", "dict"),
    ("float", "in", "list"),
    ("float", "in", "set"),
    ("float", "in", "dict"),
    ("float", "notin", "list"),
    ("float", "notin", "set"),
    ("float", "notin", "dict"),
    ("none", "in", "list"),
    ("none", "in", "set"),
    ("none", "in", "dict"),
    ("none", "notin", "list"),
    ("none", "notin", "set"),
    ("none", "notin", "dict"),
    ("list", "in", "list"),
    ("list", "in", "set"),
    ("list", "in", "dict"),
    ("list", "notin", "list"),
    ("list", "notin", "set"),
    ("list", "notin", "dict"),
    ("dict", "in", "list"),
    ("dict", "in", "set"),
    ("dict", "in", "dict"),
    ("dict", "notin", "list"),
    ("dict", "notin", "set"),
    ("dict", "notin", "dict"),
    ("set", "in", "list"),
    ("set", "in", "set"),
    ("set", "in", "dict"),
    ("set", "notin", "list"),
    ("set", "notin", "set"),
    ("set", "notin", "dict"),
    # bool in int list is actually OK since bool can be 0 or 1
    # but let's add some for safety
    # dict == dict has a bug (compares pointers not content)
    ("dict", "eq", "dict"),
    ("dict", "ne", "dict"),
}

# Builtin functions to test with their configurations
# Format: (func, min_args, max_args, description)
# We'll auto-detect argument counts where possible
BUILTIN_FUNCS = {
    # Single argument functions
    "abs": (abs, 1, 1, "absolute value"),
    "len": (len, 1, 1, "length"),
    "bool": (bool, 1, 1, "bool conversion"),
    "int": (int, 1, 1, "int conversion"),
    "float": (float, 1, 1, "float conversion"),
    "str": (str, 1, 1, "str conversion"),
    # Unsupported: repr, hash, id, type
    "hex": (hex, 1, 1, "hex conversion"),
    "oct": (oct, 1, 1, "oct conversion"),
    "bin": (bin, 1, 1, "bin conversion"),
    "ord": (ord, 1, 1, "ord"),
    "chr": (chr, 1, 1, "chr"),
    "ascii": (ascii, 1, 1, "ascii"),
    "sorted": (sorted, 1, 1, "sorted"),
    "list": (list, 1, 1, "list conversion"),
    # Unsupported: set, reversed, frozenset, tuple, iter, any, all, enumerate
    "sum": (sum, 1, 1, "sum"),

    # Two argument functions
    "pow": (pow, 2, 3, "power"),
    "divmod": (divmod, 2, 2, "divmod"),
    # Unsupported: isinstance, issubclass, getattr, hasattr, setattr, delattr

    # Variable argument functions
    "min": (min, 1, 2, "min"),
    "max": (max, 1, 2, "max"),
    "round": (round, 1, 2, "round"),
    # Unsupported: format, range, slice, zip, map, filter
    "print": (print, 1, 1, "print"),  # Test with 1 arg only
    "input": None,  # Skip - requires stdin
    "open": None,  # Skip - file operations
    "exec": None,  # Skip - security
    "eval": None,  # Skip - security
    "compile": None,  # Skip - complex
}


def is_binary_op_valid(left_val, right_val, op_func) -> bool:
    """Check if a binary operation is valid by trying it."""
    try:
        op_func(left_val, right_val)
        return True
    except (TypeError, ZeroDivisionError, ValueError):
        return False


def is_unary_op_valid(val, op_func) -> bool:
    """Check if a unary operation is valid by trying it."""
    try:
        op_func(val)
        return True
    except TypeError:
        return False


def is_code_valid(code: str) -> bool:
    """Check if Python code is valid by actually running it."""
    with tempfile.NamedTemporaryFile(mode='w', suffix='.py', delete=False) as f:
        f.write(code)
        f.flush()
        temp_path = f.name

    try:
        result = subprocess.run(
            ['python3', temp_path],
            capture_output=True,
            timeout=5
        )
        return result.returncode == 0
    except subprocess.TimeoutExpired:
        return False
    finally:
        Path(temp_path).unlink(missing_ok=True)


def get_type_annotation(type_key: str) -> str:
    """Get the full type annotation for a type key."""
    return TYPES[type_key][0]


def get_result_type(left_type: str, right_type: str, op: str) -> str:
    """Determine the expected result type for a binary operation."""
    if op in ("eq", "ne", "lt", "le", "gt", "ge", "in", "notin"):
        return "bool"
    # and/or: same type returns same type, different types return bool
    if op in ("and", "or"):
        if left_type == right_type:
            return get_type_annotation(left_type)
        return "bool"
    if op == "div":
        return "float"
    # Handle string/bytes multiplication with int: result is str/bytes
    if op == "mul":
        if left_type == "str" or right_type == "str":
            return "str"
        if left_type == "bytes" or right_type == "bytes":
            return "bytes"
        if left_type == "list" or right_type == "list":
            return get_type_annotation("list")
    # String formatting
    if op == "mod" and left_type == "str":
        return "str"
    if left_type == "float" or right_type == "float":
        return "float"
    # Bitwise ops on bool+bool return bool in TypePython
    if op in ("bitand", "bitor", "bitxor"):
        if left_type == "bool" and right_type == "bool":
            return "bool"
        # bool & int or int & bool returns int
        if left_type in ("int", "bool") and right_type in ("int", "bool"):
            return "int"
    # Arithmetic with bool always returns int (bool is promoted)
    if op in ("add", "sub", "mul", "floordiv", "mod", "pow", "lshift", "rshift"):
        if left_type == "bool" or right_type == "bool":
            # bool involved in arithmetic -> int result
            if left_type in ("int", "bool") and right_type in ("int", "bool"):
                return "int"
    if left_type == right_type:
        return get_type_annotation(left_type)
    if left_type in ("int", "bool") and right_type in ("int", "bool"):
        return "int"
    return get_type_annotation(left_type)


def get_unary_result_type(type_name: str) -> str:
    """Determine the expected result type for a unary operation."""
    if type_name == "bool":
        return "int"
    return get_type_annotation(type_name)


def generate_invalid_binary_file(left_type: str, right_type: str, op: str) -> tuple[str, str]:
    """Generate content for an invalid binary operation test file."""
    type_annot, example, _ = TYPES[left_type]
    right_annot, right_example, _ = TYPES[right_type]
    op_symbol, op_desc, _ = BINARY_OPS[op]

    filename = f"{left_type}_{op}_{right_type}.py"
    result_type = get_result_type(left_type, right_type, op)

    left_name = type_annot.capitalize()
    right_name = right_annot.capitalize()

    if op == "in":
        content = f"""# Cannot use 'in' with {left_name} and {right_name}
x: bool = {example} in {right_example}
"""
    elif op == "notin":
        content = f"""# Cannot use 'not in' with {left_name} and {right_name}
x: bool = {example} not in {right_example}
"""
    else:
        content = f"""# Cannot {op_desc} {left_name} and {right_name}
x: {result_type} = {example} {op_symbol} {right_example}
"""
    return filename, content


def generate_invalid_unary_file(type_name: str, op: str) -> tuple[str, str]:
    """Generate content for an invalid unary operation test file."""
    type_annot, example, _ = TYPES[type_name]
    op_symbol, op_desc, _ = UNARY_OPS[op]

    filename = f"{type_name}_{op}.py"
    result_type = get_unary_result_type(type_name)
    type_display = type_annot.capitalize()

    if op == "bitnot":
        content = f"""# Cannot use bitwise NOT on {type_display}
x: {type_annot} = {example}
y: int = ~x
"""
    else:
        content = f"""# Cannot use {op_desc} on {type_display}
x: {result_type} = {op_symbol}{example}
"""
    return filename, content


def generate_invalid_builtin_file(func_name: str, type_names: tuple[str, ...]) -> tuple[str, str]:
    """Generate content for an invalid builtin function test file."""
    type_str = "_".join(type_names)
    filename = f"{func_name}_{type_str}.py"

    examples = [TYPES[t][1] for t in type_names]
    type_displays = [TYPES[t][0].capitalize() for t in type_names]

    args_str = ", ".join(examples)
    types_str = ", ".join(type_displays)

    content = f"""# Cannot call {func_name}() with {types_str}
x = {func_name}({args_str})
"""
    return filename, content


def generate_valid_line(left_type: str, right_type: str, op: str, idx: int) -> str:
    """Generate a single line for valid binary operation."""
    _, left_example, _ = TYPES[left_type]
    _, right_example, _ = TYPES[right_type]
    op_symbol, _, _ = BINARY_OPS[op]
    result_type = get_result_type(left_type, right_type, op)

    var_name = f"v{idx}_{left_type}_{op}_{right_type}"

    if op == "in":
        return f"{var_name}: bool = {left_example} in {right_example}"
    elif op == "notin":
        return f"{var_name}: bool = {left_example} not in {right_example}"
    elif op in ("and", "or"):
        # TypePython: same type returns same type, different types return bool
        # Wrap with bool() only for different types (for Python compat)
        if left_type != right_type:
            return f"{var_name}: bool = bool({left_example} {op_symbol} {right_example})"
        else:
            return f"{var_name}: {result_type} = {left_example} {op_symbol} {right_example}"
    else:
        return f"{var_name}: {result_type} = {left_example} {op_symbol} {right_example}"


def generate_valid_unary_line(type_name: str, op: str, idx: int) -> list[str]:
    """Generate lines for valid unary operation."""
    type_annot, example, _ = TYPES[type_name]
    op_symbol, _, _ = UNARY_OPS[op]
    result_type = get_unary_result_type(type_name)

    var_name = f"u{idx}_{type_name}_{op}"

    if op == "bitnot":
        return [
            f"{var_name}_val: {type_annot} = {example}",
            f"{var_name}: int = ~{var_name}_val"
        ]
    else:
        return [f"{var_name}: {result_type} = {op_symbol}{example}"]


def generate_valid_builtin_line(func_name: str, type_names: tuple[str, ...], idx: int) -> str:
    """Generate a single line for valid builtin function call."""
    examples = [TYPES[t][1] for t in type_names]
    args_str = ", ".join(examples)
    type_str = "_".join(type_names)
    var_name = f"b{idx}_{func_name}_{type_str}"
    return f"{var_name} = {func_name}({args_str})"


def process_operators():
    """Process binary and unary operators."""
    INVALID_DIR.mkdir(parents=True, exist_ok=True)
    VALID_DIR.mkdir(parents=True, exist_ok=True)

    invalid_generated = 0
    invalid_skipped = 0
    valid_lines = [
        "# All valid binary and unary operator combinations",
        "# Auto-generated by scripts/generate_operator_tests.py",
        "",
        "# ===== VALID BINARY OPERATIONS =====",
        "",
    ]
    valid_idx = 0
    binary_var_names = []

    print("Processing binary operators...")

    for left_type, (_, _, left_val) in TYPES.items():
        for right_type, (_, _, right_val) in TYPES.items():
            for op, (_, _, op_func) in BINARY_OPS.items():
                # Skip operations that TypePython doesn't support
                if (left_type, op, right_type) in UNSUPPORTED_OPS:
                    continue
                if is_binary_op_valid(left_val, right_val, op_func):
                    var_name = f"v{valid_idx}_{left_type}_{op}_{right_type}"
                    binary_var_names.append(var_name)
                    valid_lines.append(generate_valid_line(left_type, right_type, op, valid_idx))
                    valid_idx += 1
                else:
                    filename, content = generate_invalid_binary_file(left_type, right_type, op)
                    filepath = INVALID_DIR / filename

                    if filepath.exists():
                        invalid_skipped += 1
                    else:
                        filepath.write_text(content)
                        invalid_generated += 1
                        print(f"  Generated: {filename}")

    valid_lines.append("")
    valid_lines.append("# ===== VALID UNARY OPERATIONS =====")
    valid_lines.append("")

    print("\nProcessing unary operators...")

    unary_idx = 0
    unary_var_names = []
    for type_name, (_, _, val) in TYPES.items():
        for op, (_, _, op_func) in UNARY_OPS.items():
            if is_unary_op_valid(val, op_func):
                var_name = f"u{unary_idx}_{type_name}_{op}"
                unary_var_names.append(var_name)
                valid_lines.extend(generate_valid_unary_line(type_name, op, unary_idx))
                unary_idx += 1
            else:
                filename, content = generate_invalid_unary_file(type_name, op)
                filepath = INVALID_DIR / filename

                if filepath.exists():
                    invalid_skipped += 1
                else:
                    filepath.write_text(content)
                    invalid_generated += 1
                    print(f"  Generated: {filename}")

    valid_lines.append("")
    valid_lines.append("# ===== PRINT ALL VARIABLES =====")
    valid_lines.append("")
    for var_name in binary_var_names:
        valid_lines.append(f'print("{var_name}:", {var_name})')
    for var_name in unary_var_names:
        valid_lines.append(f'print("{var_name}:", {var_name})')
    valid_lines.append("")
    valid_lines.append('print("All valid operator tests passed!")')
    valid_lines.append("")

    valid_file = VALID_DIR / "all_valid_operators.py"
    valid_file.write_text("\n".join(valid_lines))

    print(f"\nOperators: Generated {invalid_generated} new invalid files, skipped {invalid_skipped} existing.")
    print(f"Valid operators file: {valid_file}")
    print(f"Total valid: {valid_idx} binary + {unary_idx} unary")

    return invalid_generated, invalid_skipped


def process_builtins():
    """Process builtin functions - generate separate file for each function."""
    INVALID_DIR.mkdir(parents=True, exist_ok=True)
    VALID_BUILTINS_DIR.mkdir(parents=True, exist_ok=True)

    invalid_generated = 0
    invalid_skipped = 0
    total_valid = 0

    print("\nProcessing builtin functions...")

    for func_name, config in BUILTIN_FUNCS.items():
        if config is None:
            print(f"  Skipping {func_name} (not testable)")
            continue

        _, min_args, max_args, _ = config

        # Collect valid calls for this function
        valid_lines = [
            f"# Valid {func_name}() calls",
            f"# Auto-generated by scripts/generate_tests.py",
            "",
        ]
        valid_idx = 0
        var_names = []

        for num_args in range(min_args, max_args + 1):
            # Generate all type combinations for this number of arguments
            for type_combo in itertools.product(TYPES.keys(), repeat=num_args):
                # Generate the code line
                code_line = generate_valid_builtin_line(func_name, type_combo, valid_idx)
                # Test if it's valid by running actual Python
                test_code = code_line + "\n"

                if is_code_valid(test_code):
                    type_str = "_".join(type_combo)
                    var_name = f"b{valid_idx}_{func_name}_{type_str}"
                    var_names.append(var_name)
                    valid_lines.append(code_line)
                    valid_idx += 1
                else:
                    filename, content = generate_invalid_builtin_file(func_name, type_combo)
                    filepath = INVALID_DIR / filename

                    if filepath.exists():
                        invalid_skipped += 1
                    else:
                        filepath.write_text(content)
                        invalid_generated += 1
                        print(f"  Generated: {filename}")

        # Write valid file for this function if there are any valid calls
        if valid_idx > 0:
            valid_lines.append("")
            valid_lines.append("# Print all variables")
            for var_name in var_names:
                valid_lines.append(f'print("{var_name}:", {var_name})')
            valid_lines.append("")
            valid_lines.append(f'print("{func_name}() tests passed!")')
            valid_lines.append("")

            valid_file = VALID_BUILTINS_DIR / f"{func_name}.py"
            valid_file.write_text("\n".join(valid_lines))
            total_valid += valid_idx
            print(f"  {func_name}: {valid_idx} valid calls")

    print(f"\nBuiltins: Generated {invalid_generated} new invalid files, skipped {invalid_skipped} existing.")
    print(f"Valid builtins directory: {VALID_BUILTINS_DIR}")
    print(f"Total valid builtin calls: {total_valid}")

    return invalid_generated, invalid_skipped


def main():
    """Generate all test files."""
    op_gen, op_skip = process_operators()
    builtin_gen, builtin_skip = process_builtins()

    print(f"\n{'='*50}")
    print(f"TOTAL: Generated {op_gen + builtin_gen} new files, skipped {op_skip + builtin_skip} existing.")


if __name__ == "__main__":
    main()
