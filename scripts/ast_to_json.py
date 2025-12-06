#!/usr/bin/env python3
"""
Serialize Python AST to JSON for TypePython compiler.
Output matches Python's ast module structure exactly.
"""

import ast
import json
import sys
from typing import Any


def node_to_dict(node: ast.AST) -> dict[str, Any]:
    """Convert an AST node to a dictionary."""
    result = {"_type": node.__class__.__name__}

    for field, value in ast.iter_fields(node):
        result[field] = serialize_value(value)

    # Include location info
    if hasattr(node, "lineno"):
        result["lineno"] = node.lineno
    if hasattr(node, "col_offset"):
        result["col_offset"] = node.col_offset
    if hasattr(node, "end_lineno"):
        result["end_lineno"] = node.end_lineno
    if hasattr(node, "end_col_offset"):
        result["end_col_offset"] = node.end_col_offset

    return result


def serialize_value(value: Any) -> Any:
    """Serialize a field value."""
    if isinstance(value, ast.AST):
        return node_to_dict(value)
    elif isinstance(value, list):
        return [serialize_value(v) for v in value]
    elif isinstance(value, bytes):
        # Bytes literals need special handling
        return {"_bytes": list(value)}
    elif value is None or isinstance(value, (str, int, float, bool)):
        return value
    elif isinstance(value, complex):
        return {"_complex": {"real": value.real, "imag": value.imag}}
    elif value is ...:
        return {"_ellipsis": True}
    else:
        # Fallback for unknown types
        return repr(value)


def parse_file(filepath: str) -> dict[str, Any]:
    """Parse a Python file and return JSON-serializable AST."""
    with open(filepath, "r") as f:
        source = f.read()

    tree = ast.parse(source, filename=filepath)
    return node_to_dict(tree)


def parse_source(source: str) -> dict[str, Any]:
    """Parse Python source code and return JSON-serializable AST."""
    tree = ast.parse(source)
    return node_to_dict(tree)


def main():
    if len(sys.argv) < 2:
        print("Usage: ast_to_json.py <file.py>", file=sys.stderr)
        print("       ast_to_json.py --source <code>", file=sys.stderr)
        sys.exit(1)

    try:
        if sys.argv[1] == "--source":
            if len(sys.argv) < 3:
                print("Error: --source requires code argument", file=sys.stderr)
                sys.exit(1)
            result = parse_source(sys.argv[2])
        else:
            filepath = sys.argv[1]
            result = parse_file(filepath)

        print(json.dumps(result))

    except SyntaxError as e:
        error = {
            "_error": "SyntaxError",
            "msg": str(e.msg) if e.msg else str(e),
            "lineno": e.lineno,
            "offset": e.offset,
            "text": e.text,
        }
        print(json.dumps(error))
        sys.exit(1)
    except FileNotFoundError as e:
        print(json.dumps({"_error": f"FileNotFoundError: {e}"}))
        sys.exit(1)
    except Exception as e:
        print(json.dumps({"_error": str(e)}))
        sys.exit(1)


if __name__ == "__main__":
    main()
