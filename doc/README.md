### 1\. Language Overview

  * **Syntax:** Python-style (indentation-based).
  * **Typing:** Strong, Static. Types are required for variable declarations and function signatures.
  * **Scoping:** Block-scoped (variables defined inside an `if` or `function` do not leak out).
  * **Primitive Types:** `int`, `float`, `bool`, `str`, `void`.

-----

### 2\. The Grammar Specification

Here is the hierarchical grammar. I have separated it into **Declarations**, **Statements**, and **Expressions** to handle operator precedence automatically.

#### A. Lexical Tokens (The Building Blocks)

Before the grammar, assume the Lexer produces these tokens:

  * `ID`: Identifiers (e.g., `myVar`, `calculate_sum`)
  * `INT_LIT`, `FLOAT_LIT`, `STR_LIT`: Literals
  * `INDENT`, `DEDENT`: These are crucial. Your lexer must track whitespace changes and emit these tokens to mimic Python blocks.
  * `NEWLINE`: End of a statement.

#### B. Program Structure

```ebnf
program      ::= (statement | func_decl)* EOF
```

#### C. Statements

Statements are the executable units. Notice that `block` relies on Indentation.

```ebnf
statement    ::= var_decl NEWLINE
               | assignment NEWLINE
               | if_stmt
               | while_stmt
               | return_stmt NEWLINE
               | expr_stmt NEWLINE
               | pass_stmt NEWLINE

block        ::= NEWLINE INDENT statement+ DEDENT

# 1. Variable Declaration: x: int = 5
var_decl     ::= ID ":" type_spec "=" expression

# 2. Assignment: x = 10
assignment   ::= ID "=" expression

# 3. Control Flow
if_stmt      ::= "if" expression ":" block 
                 ("elif" expression ":" block)* ("else" ":" block)?

while_stmt   ::= "while" expression ":" block

# 4. Functions
func_decl    ::= "def" ID "(" param_list? ")" "->" type_spec ":" block

param_list   ::= param ("," param)*
param        ::= ID ":" type_spec

return_stmt  ::= "return" expression?

pass_stmt    ::= "pass"
```

#### D. Types

```ebnf
type_spec    ::= "int" 
               | "float" 
               | "bool" 
               | "str" 
               | "void"
```

#### E. Expressions (With Precedence)

This hierarchy ensures mathematical operations happen in the correct order (PEMDAS).

```ebnf
# Lowest precedence
expression   ::= logic_or

logic_or     ::= logic_and ("or" logic_and)*
logic_and    ::= equality ("and" equality)*

equality     ::= comparison (("==" | "!=") comparison)*
comparison   ::= term (("<" | ">" | "<=" | ">=") term)*

term         ::= factor (("+" | "-") factor)*
factor       ::= unary (("*" | "/" | "%") unary)*

unary        ::= ("-" | "not") unary 
               | call

# Function calls or atom access
call         ::= atom "(" arg_list? ")" 
               | atom

arg_list     ::= expression ("," expression)*

# Highest precedence
atom         ::= INT_LIT
               | FLOAT_LIT
               | STR_LIT
               | "True" | "False"
               | ID
               | "(" expression ")"
```

-----

### 3\. Example Code

Here is what valid code in this grammar looks like:

```python
# Function with explicit typing
def add_numbers(a: int, b: int) -> int:
    result: int = a + b
    return result

# Variable Declaration
radius: float = 5.5
is_valid: bool = True

# Control Flow & Scope
if is_valid:
    # 'area' is scoped to this block
    area: float = 3.14 * (radius * radius)
    
    # Nested scope
    while radius > 0.0:
        radius = radius - 1.0

# Calling a function
x: int = add_numbers(10, 20)
```
