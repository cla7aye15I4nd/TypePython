# Binary search algorithm (for powers of 2)
def binary_search_power(target: int, max_exp: int) -> int:
    low: int = 0
    high: int = max_exp
    result: int = -1

    while low <= high:
        mid: int = (low + high) // 2
        value: int = 1
        i: int = 0

        # Compute 2^mid
        while i < mid:
            value = value * 2
            i = i + 1

        if value == target:
            result = mid
            low = high + 1
        else:
            if value < target:
                low = mid + 1
            else:
                high = mid - 1

    return result

def test_binary_search() -> int:
    pow1: int = binary_search_power(16, 10)
    pow2: int = binary_search_power(64, 10)
    pow3: int = binary_search_power(256, 10)
    pow4: int = binary_search_power(1024, 10)

    return pow1 + pow2 + pow3 + pow4

result: int = test_binary_search()
print("Binary search result:", result)
print("Log2(16):", binary_search_power(16, 10))
print("Log2(256):", binary_search_power(256, 10))
