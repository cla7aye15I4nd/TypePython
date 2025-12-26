# Linear Congruential Generator (LCG) for random number generation
# Uses the MINSTD parameters: a=48271, m=2147483647

class RNG:
    seed: int

    def __init__(self, s: int) -> None:
        self.seed = s

    def next(self) -> int:
        # LCG formula: next = (a * seed) % m
        # Using MINSTD: a=48271, m=2^31-1=2147483647
        self.seed = (self.seed * 48271) % 2147483647
        return self.seed

    def rand_range(self, min_val: int, max_val: int) -> int:
        # Returns a random int in [min_val, max_val]
        r: int = self.next()
        range_size: int = max_val - min_val + 1
        return min_val + (r % range_size)


def make_rand_list(rng: RNG, size: int, min_val: int, max_val: int) -> list[int]:
    # Generate a list of random integers
    result: list[int] = [0]
    result_size: int = 0
    i: int = 0
    while i < size:
        if result_size == 0:
            result[0] = rng.rand_range(min_val, max_val)
        else:
            result.append(rng.rand_range(min_val, max_val))
        result_size = result_size + 1
        i = i + 1
    return result
