# Module I - imports J and K, creates cycle with G and H
import module_j
import module_k
import module_g

def test() -> int:
    return module_j.test() + module_k.test() + 9
