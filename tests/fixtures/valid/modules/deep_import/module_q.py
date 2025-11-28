# Module Q - imports R and S, creates cycle with O and P
import module_r
import module_s
import module_o

def test() -> int:
    # Only call forward (r, s) to avoid cycles with o
    return module_r.test() + module_s.test() + 17
