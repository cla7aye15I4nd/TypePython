# Module S - imports T and creates cycle with Q and R, also cycles back to A
import module_t
import module_q
import module_a

def test() -> int:
    return module_t.test() + 19
