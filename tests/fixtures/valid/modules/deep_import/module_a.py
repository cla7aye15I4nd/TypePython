# Module A - starts the import chain
import module_b
import module_c

def test() -> int:
    return module_b.test() + module_c.test() + 1
