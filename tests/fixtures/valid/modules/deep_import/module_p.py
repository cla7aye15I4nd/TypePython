# Module P - imports Q and R
import module_q
import module_r

def test() -> int:
    return module_q.test() + module_r.test() + 16
