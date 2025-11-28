# Module O - imports P and Q, creates cycle with M and N
import module_p
import module_q
import module_m

def test() -> int:
    return module_p.test() + module_q.test() + 15
