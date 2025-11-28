# Module F - imports G and H
import module_g
import module_h

def test() -> int:
    return module_g.test() + module_h.test() + 6
