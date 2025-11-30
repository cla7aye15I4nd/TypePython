# Test iteration patterns in search algorithms

# Linear search
def linear_search(arr: list[int], target: int) -> int:
    for i in range(len(arr)):
        if arr[i] == target:
            return i
    return -1

data: list[int] = [64, 34, 25, 12, 22, 11, 90]
print(b"Linear search 22:", linear_search(data, 22))
print(b"Linear search 100:", linear_search(data, 100))

# Binary search (iterative)
def binary_search(arr: list[int], target: int) -> int:
    left: int = 0
    right: int = len(arr) - 1

    while left <= right:
        mid: int = (left + right) // 2
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1

    return -1

sorted_data: list[int] = [11, 12, 22, 25, 34, 64, 90]
print(b"Binary search 22:", binary_search(sorted_data, 22))
print(b"Binary search 100:", binary_search(sorted_data, 100))

# Find all occurrences
def find_all(arr: list[int], target: int) -> list[int]:
    indices: list[int] = []
    for i in range(len(arr)):
        if arr[i] == target:
            indices.append(i)
    return indices

repeated: list[int] = [1, 2, 3, 2, 4, 2, 5]
print(b"All occurrences of 2:", find_all(repeated, 2))

# Find first occurrence
def find_first(arr: list[int], target: int) -> int:
    for i in range(len(arr)):
        if arr[i] == target:
            return i
    return -1

print(b"First 2:", find_first(repeated, 2))

# Find last occurrence
def find_last(arr: list[int], target: int) -> int:
    result: int = -1
    for i in range(len(arr)):
        if arr[i] == target:
            result = i
    return result

print(b"Last 2:", find_last(repeated, 2))

# Find min
def find_min(arr: list[int]) -> tuple[int, int]:
    if len(arr) == 0:
        return (-1, -1)
    min_val: int = arr[0]
    min_idx: int = 0
    for i in range(1, len(arr)):
        if arr[i] < min_val:
            min_val = arr[i]
            min_idx = i
    return (min_idx, min_val)

print(b"Min:", find_min(data))

# Find max
def find_max(arr: list[int]) -> tuple[int, int]:
    if len(arr) == 0:
        return (-1, -1)
    max_val: int = arr[0]
    max_idx: int = 0
    for i in range(1, len(arr)):
        if arr[i] > max_val:
            max_val = arr[i]
            max_idx = i
    return (max_idx, max_val)

print(b"Max:", find_max(data))

# Find min and max together
def find_min_max(arr: list[int]) -> tuple[int, int]:
    if len(arr) == 0:
        return (0, 0)
    min_v: int = arr[0]
    max_v: int = arr[0]
    for x in arr:
        if x < min_v:
            min_v = x
        if x > max_v:
            max_v = x
    return (min_v, max_v)

print(b"Min-Max:", find_min_max(data))

# Find kth smallest
def find_kth_smallest(arr: list[int], k: int) -> int:
    sorted_arr: list[int] = sorted(arr)
    return sorted_arr[k - 1]

print(b"3rd smallest:", find_kth_smallest(data, 3))

# Find kth largest
def find_kth_largest(arr: list[int], k: int) -> int:
    sorted_arr: list[int] = sorted(arr, reverse=True)
    return sorted_arr[k - 1]

print(b"2nd largest:", find_kth_largest(data, 2))

# Count occurrences
def count_occurrences(arr: list[int], target: int) -> int:
    count: int = 0
    for x in arr:
        if x == target:
            count = count + 1
    return count

print(b"Count of 2:", count_occurrences(repeated, 2))

# Find missing number in 1 to n
def find_missing(arr: list[int], n: int) -> int:
    expected_sum: int = n * (n + 1) // 2
    actual_sum: int = 0
    for x in arr:
        actual_sum = actual_sum + x
    return expected_sum - actual_sum

incomplete: list[int] = [1, 2, 4, 5, 6]  # Missing 3
print(b"Missing number:", find_missing(incomplete, 6))

# Find duplicate
def find_duplicate(arr: list[int]) -> int:
    seen: set[int] = set()
    for x in arr:
        if x in seen:
            return x
        seen.add(x)
    return -1

with_dup: list[int] = [1, 2, 3, 4, 2, 5]
print(b"Duplicate:", find_duplicate(with_dup))

# Find all duplicates
def find_all_duplicates(arr: list[int]) -> list[int]:
    freq: dict[int, int] = {}
    for x in arr:
        freq[x] = freq.get(x, 0) + 1

    dups: list[int] = []
    for k, v in freq.items():
        if v > 1:
            dups.append(k)
    return dups

with_dups: list[int] = [1, 2, 3, 2, 4, 3, 5]
print(b"All duplicates:", find_all_duplicates(with_dups))

# Find majority element
def find_majority(arr: list[int]) -> int:
    freq: dict[int, int] = {}
    for x in arr:
        freq[x] = freq.get(x, 0) + 1

    threshold: int = len(arr) // 2
    for k, v in freq.items():
        if v > threshold:
            return k
    return -1

majority: list[int] = [3, 3, 4, 2, 4, 4, 2, 4, 4]
print(b"Majority element:", find_majority(majority))

# Two sum
def two_sum(arr: list[int], target: int) -> tuple[int, int]:
    seen: dict[int, int] = {}
    for i in range(len(arr)):
        complement: int = target - arr[i]
        if complement in seen:
            return (seen[complement], i)
        seen[arr[i]] = i
    return (-1, -1)

nums: list[int] = [2, 7, 11, 15]
print(b"Two sum for 9:", two_sum(nums, 9))

# Subarray sum
def subarray_sum(arr: list[int], target: int) -> tuple[int, int]:
    for i in range(len(arr)):
        current_sum: int = 0
        for j in range(i, len(arr)):
            current_sum = current_sum + arr[j]
            if current_sum == target:
                return (i, j)
    return (-1, -1)

arr: list[int] = [1, 4, 20, 3, 10, 5]
print(b"Subarray sum 33:", subarray_sum(arr, 33))
