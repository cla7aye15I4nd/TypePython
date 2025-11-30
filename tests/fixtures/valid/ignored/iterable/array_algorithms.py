# Test iteration patterns in array algorithms

# Rotate array left
def rotate_left(arr: list[int], k: int) -> list[int]:
    n: int = len(arr)
    k = k % n
    return arr[k:] + arr[:k]

data: list[int] = [1, 2, 3, 4, 5]
print(b"Rotate left 2:", rotate_left(data, 2))

# Rotate array right
def rotate_right(arr: list[int], k: int) -> list[int]:
    n: int = len(arr)
    k = k % n
    return arr[-k:] + arr[:-k]

print(b"Rotate right 2:", rotate_right(data, 2))

# Remove duplicates (preserve order)
def remove_duplicates(arr: list[int]) -> list[int]:
    seen: set[int] = set()
    result: list[int] = []
    for x in arr:
        if x not in seen:
            seen.add(x)
            result.append(x)
    return result

dups: list[int] = [1, 2, 2, 3, 3, 3, 4, 4, 4, 4]
print(b"Remove dups:", remove_duplicates(dups))

# Move zeros to end
def move_zeros(arr: list[int]) -> list[int]:
    non_zero: list[int] = []
    zero_count: int = 0
    for x in arr:
        if x != 0:
            non_zero.append(x)
        else:
            zero_count = zero_count + 1

    for _ in range(zero_count):
        non_zero.append(0)
    return non_zero

zeros: list[int] = [0, 1, 0, 3, 12]
print(b"Move zeros:", move_zeros(zeros))

# Partition around pivot
def partition(arr: list[int], pivot: int) -> list[int]:
    less: list[int] = []
    equal: list[int] = []
    greater: list[int] = []

    for x in arr:
        if x < pivot:
            less.append(x)
        elif x == pivot:
            equal.append(x)
        else:
            greater.append(x)

    return less + equal + greater

p_data: list[int] = [3, 1, 4, 1, 5, 9, 2, 6]
print(b"Partition around 4:", partition(p_data, 4))

# Find equilibrium index
def equilibrium(arr: list[int]) -> int:
    total: int = 0
    for x in arr:
        total = total + x

    left_sum: int = 0
    for i in range(len(arr)):
        if left_sum == total - left_sum - arr[i]:
            return i
        left_sum = left_sum + arr[i]

    return -1

eq_data: list[int] = [-7, 1, 5, 2, -4, 3, 0]
print(b"Equilibrium index:", equilibrium(eq_data))

# Maximum subarray sum (Kadane's)
def max_subarray(arr: list[int]) -> int:
    max_ending: int = arr[0]
    max_so_far: int = arr[0]

    for i in range(1, len(arr)):
        max_ending = max(arr[i], max_ending + arr[i])
        max_so_far = max(max_so_far, max_ending)

    return max_so_far

sub_data: list[int] = [-2, 1, -3, 4, -1, 2, 1, -5, 4]
print(b"Max subarray:", max_subarray(sub_data))

# Longest increasing subsequence length
def lis_length(arr: list[int]) -> int:
    if len(arr) == 0:
        return 0

    dp: list[int] = [1] * len(arr)

    for i in range(1, len(arr)):
        for j in range(i):
            if arr[j] < arr[i]:
                dp[i] = max(dp[i], dp[j] + 1)

    max_len: int = dp[0]
    for x in dp:
        if x > max_len:
            max_len = x
    return max_len

lis_data: list[int] = [10, 9, 2, 5, 3, 7, 101, 18]
print(b"LIS length:", lis_length(lis_data))

# Merge two sorted arrays
def merge_sorted(a: list[int], b: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    j: int = 0

    while i < len(a) and j < len(b):
        if a[i] <= b[j]:
            result.append(a[i])
            i = i + 1
        else:
            result.append(b[j])
            j = j + 1

    while i < len(a):
        result.append(a[i])
        i = i + 1

    while j < len(b):
        result.append(b[j])
        j = j + 1

    return result

arr1: list[int] = [1, 3, 5, 7]
arr2: list[int] = [2, 4, 6, 8]
print(b"Merged sorted:", merge_sorted(arr1, arr2))

# Find intersection
def intersection(a: list[int], b: list[int]) -> list[int]:
    set_a: set[int] = set(a)
    result: list[int] = []
    for x in b:
        if x in set_a:
            result.append(x)
            set_a.remove(x)
    return result

int1: list[int] = [1, 2, 2, 1]
int2: list[int] = [2, 2]
print(b"Intersection:", intersection(int1, int2))

# Find union
def union(a: list[int], b: list[int]) -> list[int]:
    result: set[int] = set()
    for x in a:
        result.add(x)
    for x in b:
        result.add(x)
    return list(result)

print(b"Union:", union([1, 2, 3], [3, 4, 5]))

# Frequency count
def frequency(arr: list[int]) -> dict[int, int]:
    freq: dict[int, int] = {}
    for x in arr:
        freq[x] = freq.get(x, 0) + 1
    return freq

print(b"Frequency:", frequency([1, 2, 2, 3, 3, 3]))

# Leaders in array
def leaders(arr: list[int]) -> list[int]:
    result: list[int] = []
    max_right: int = arr[-1]
    result.append(max_right)

    for i in range(len(arr) - 2, -1, -1):
        if arr[i] >= max_right:
            result.append(arr[i])
            max_right = arr[i]

    result.reverse()
    return result

lead_data: list[int] = [16, 17, 4, 3, 5, 2]
print(b"Leaders:", leaders(lead_data))

# Stock buy sell (max profit)
def max_profit(prices: list[int]) -> int:
    if len(prices) < 2:
        return 0

    min_price: int = prices[0]
    profit: int = 0

    for price in prices:
        if price < min_price:
            min_price = price
        elif price - min_price > profit:
            profit = price - min_price

    return profit

stock: list[int] = [7, 1, 5, 3, 6, 4]
print(b"Max profit:", max_profit(stock))

# Trapping rain water
def trap_water(heights: list[int]) -> int:
    if len(heights) == 0:
        return 0

    n: int = len(heights)
    left_max: list[int] = [0] * n
    right_max: list[int] = [0] * n

    left_max[0] = heights[0]
    for i in range(1, n):
        left_max[i] = max(left_max[i - 1], heights[i])

    right_max[n - 1] = heights[n - 1]
    for i in range(n - 2, -1, -1):
        right_max[i] = max(right_max[i + 1], heights[i])

    water: int = 0
    for i in range(n):
        water = water + min(left_max[i], right_max[i]) - heights[i]

    return water

heights: list[int] = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]
print(b"Trapped water:", trap_water(heights))
