# TypePython Built-in Functions Test Coverage

## Legend

- ✅ Tested
- ⚠️ Implemented (no tests)
- ❌ Not implemented

---

## Built-in Functions

### Tested

| Function | Notes |
|----------|-------|
| `print()` | Supports `int`, `float`, `bool`, `bytes`, `None` |
| `abs()` | Integer and float support |
| `round()` | Basic rounding and `ndigits` parameter |
| `min()` | Two-argument form (int, float, mixed) |
| `max()` | Two-argument form (int, float, mixed) |
| `pow()` | Two and three argument forms (modular) |
| `len()` | For `bytes` type |

### Not Tested

| Function | Status | Notes |
|----------|--------|-------|
| `divmod()` | ⚠️ | Runtime exists but no tests |
| `input()` | ❌ | User input |
| `int()` | ❌ | Type conversion |
| `float()` | ❌ | Type conversion |
| `str()` | ❌ | N/A - TypePython uses `bytes` |
| `bool()` | ❌ | Type conversion |
| `type()` | ❌ | Type introspection |
| `isinstance()` | ❌ | Type checking |
| `range()` | ❌ | No list/iterator support yet |
| `enumerate()` | ❌ | Iteration |
| `zip()` | ❌ | Iteration |
| `map()` | ❌ | Functional |
| `filter()` | ❌ | Functional |
| `sorted()` | ❌ | Requires list support |
| `reversed()` | ❌ | Requires sequence support |
| `sum()` | ❌ | Requires iterable support |
| `any()` | ❌ | Requires iterable support |
| `all()` | ❌ | Requires iterable support |
| `hex()` | ❌ | Number formatting |
| `oct()` | ❌ | Number formatting |
| `bin()` | ❌ | Number formatting |
| `ord()` | ❌ | Character to int |
| `chr()` | ❌ | Int to character |
| `open()` | ❌ | File I/O |
| `id()` | ❌ | Object identity |
| `hash()` | ❌ | Hash value |
| `callable()` | ❌ | Callable check |
| `repr()` | ❌ | String representation |
| `format()` | ❌ | String formatting |

---

## Bytes Operations

### Tested

| Operation | Notes |
|-----------|-------|
| Creation (`b"..."`) | Literal syntax |
| Concatenation (`+`) | Multiple operands |
| Repetition (`*`) | Including zero/one |
| Equality (`==`) | |
| Inequality (`!=`) | |
| Less than (`<`) | Lexicographic |
| Less than or equal (`<=`) | |
| Greater than (`>`) | |
| Greater than or equal (`>=`) | |
| Length (`len()`) | |
| Contains (`in`) | |
| Not contains (`not in`) | |
| Escape sequences | `\n`, `\t`, `\"`, `\\`, `\a`, `\x##` |
| Indexing (`[]`) | Positive indices |
| Negative indexing | |
| Slicing (`[:]`) | Start, end, negative |

### Not Tested (Implemented in Runtime)

| Operation | Runtime Function |
|-----------|------------------|
| `find()` | `bytes_find` |
| `startswith()` | `bytes_startswith` |
| `endswith()` | `bytes_endswith` |
| `upper()` | `bytes_upper` |
| `lower()` | `bytes_lower` |
| `strip()` | `bytes_strip` |
| `lstrip()` | `bytes_lstrip` |
| `rstrip()` | `bytes_rstrip` |
| `replace()` | `bytes_replace` |
| `count()` | `bytes_count` |
| `join()` | `bytes_join` |
| `isalnum()` | `bytes_isalnum` |
| `isalpha()` | `bytes_isalpha` |
| `isdigit()` | `bytes_isdigit` |
| `isspace()` | `bytes_isspace` |
| `islower()` | `bytes_islower` |
| `isupper()` | `bytes_isupper` |
| `reverse()` | `bytes_reverse` |
| `center()` | `bytes_center` |
| `ljust()` | `bytes_ljust` |
| `rjust()` | `bytes_rjust` |
| `zfill()` | `bytes_zfill` |

---

## Math Operators

### Tested

| Operator | Notes |
|----------|-------|
| `+` | Addition |
| `-` | Subtraction |
| `*` | Multiplication |
| `/` | Division (returns float) |
| `//` | Floor division (Python-style) |
| `%` | Modulo (Python-style) |
| `**` | Power |
| `-` (unary) | Negation |
