# Module R - imports S and T
import module_s
import module_t

def test() -> int:
    # Only call forward (s, t) to avoid cycles
    return module_s.test() + module_t.test() + 18
