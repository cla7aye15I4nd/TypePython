# Random module tests
# This module provides RNG utilities used by other tests
from rng.random import RNG, make_rand_list

def test() -> int:
    # Basic RNG test
    rng: RNG = RNG(42)
    print(rng.next())  # Should produce deterministic random number

    # Test rand_range
    print(rng.rand_range(1, 10))

    # Test make_rand_list
    rand_list: list[int] = make_rand_list(rng, 5, 1, 100)
    print(len(rand_list))  # 5

    return 0
