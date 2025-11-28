# Module M - imports N and O, creates cycle with K and L
import module_n
import module_o
import module_k

def test() -> int:
    return module_n.test() + module_o.test() + 13
