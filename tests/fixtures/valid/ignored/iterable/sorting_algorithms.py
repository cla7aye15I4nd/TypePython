# Test iteration patterns in sorting algorithms

# Bubble sort
def bubble_sort(arr: list[int]) -> list[int]:
    n: int = len(arr)
    result: list[int] = arr[:]
    for i in range(n):
        for j in range(0, n - i - 1):
            if result[j] > result[j + 1]:
                result[j], result[j + 1] = result[j + 1], result[j]
    return result

data: list[int] = [64, 34, 25, 12, 22, 11, 90]
print(b"Bubble sorted:", bubble_sort(data))

# Selection sort
def selection_sort(arr: list[int]) -> list[int]:
    n: int = len(arr)
    result: list[int] = arr[:]
    for i in range(n):
        min_idx: int = i
        for j in range(i + 1, n):
            if result[j] < result[min_idx]:
                min_idx = j
        result[i], result[min_idx] = result[min_idx], result[i]
    return result

print(b"Selection sorted:", selection_sort(data))

# Insertion sort
def insertion_sort(arr: list[int]) -> list[int]:
    result: list[int] = arr[:]
    for i in range(1, len(result)):
        key: int = result[i]
        j: int = i - 1
        while j >= 0 and result[j] > key:
            result[j + 1] = result[j]
            j = j - 1
        result[j + 1] = key
    return result

print(b"Insertion sorted:", insertion_sort(data))

# Merge sort
def merge(left: list[int], right: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    j: int = 0
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            result.append(left[i])
            i = i + 1
        else:
            result.append(right[j])
            j = j + 1
    while i < len(left):
        result.append(left[i])
        i = i + 1
    while j < len(right):
        result.append(right[j])
        j = j + 1
    return result

def merge_sort(arr: list[int]) -> list[int]:
    if len(arr) <= 1:
        return arr[:]
    mid: int = len(arr) // 2
    left: list[int] = merge_sort(arr[:mid])
    right: list[int] = merge_sort(arr[mid:])
    return merge(left, right)

print(b"Merge sorted:", merge_sort(data))

# Quick sort (iterative simulation with explicit stack)
def quick_sort(arr: list[int]) -> list[int]:
    result: list[int] = arr[:]
    n: int = len(result)
    if n <= 1:
        return result

    stack: list[tuple[int, int]] = [(0, n - 1)]
    while len(stack) > 0:
        low, high = stack.pop()
        if low >= high:
            continue

        pivot: int = result[high]
        i: int = low - 1
        for j in range(low, high):
            if result[j] <= pivot:
                i = i + 1
                result[i], result[j] = result[j], result[i]
        result[i + 1], result[high] = result[high], result[i + 1]
        pi: int = i + 1

        stack.append((low, pi - 1))
        stack.append((pi + 1, high))

    return result

print(b"Quick sorted:", quick_sort(data))

# Counting sort
def counting_sort(arr: list[int]) -> list[int]:
    if len(arr) == 0:
        return []

    max_val: int = arr[0]
    min_val: int = arr[0]
    for x in arr:
        if x > max_val:
            max_val = x
        if x < min_val:
            min_val = x

    count: list[int] = []
    for _ in range(max_val - min_val + 1):
        count.append(0)

    for x in arr:
        count[x - min_val] = count[x - min_val] + 1

    result: list[int] = []
    for i in range(len(count)):
        for _ in range(count[i]):
            result.append(i + min_val)

    return result

print(b"Counting sorted:", counting_sort(data))

# Heap sort (simplified)
def heapify(arr: list[int], n: int, i: int) -> None:
    largest: int = i
    left: int = 2 * i + 1
    right: int = 2 * i + 2

    if left < n and arr[left] > arr[largest]:
        largest = left
    if right < n and arr[right] > arr[largest]:
        largest = right

    if largest != i:
        arr[i], arr[largest] = arr[largest], arr[i]
        heapify(arr, n, largest)

def heap_sort(arr: list[int]) -> list[int]:
    result: list[int] = arr[:]
    n: int = len(result)

    for i in range(n // 2 - 1, -1, -1):
        heapify(result, n, i)

    for i in range(n - 1, 0, -1):
        result[0], result[i] = result[i], result[0]
        heapify(result, i, 0)

    return result

print(b"Heap sorted:", heap_sort(data))

# Shell sort
def shell_sort(arr: list[int]) -> list[int]:
    result: list[int] = arr[:]
    n: int = len(result)
    gap: int = n // 2

    while gap > 0:
        for i in range(gap, n):
            temp: int = result[i]
            j: int = i
            while j >= gap and result[j - gap] > temp:
                result[j] = result[j - gap]
                j = j - gap
            result[j] = temp
        gap = gap // 2

    return result

print(b"Shell sorted:", shell_sort(data))

# Verify all sorts produce same result
original: list[int] = [5, 2, 8, 1, 9, 3, 7, 4, 6]
results: list[list[int]] = [
    bubble_sort(original),
    selection_sort(original),
    insertion_sort(original),
    merge_sort(original),
    quick_sort(original),
    counting_sort(original),
    heap_sort(original),
    shell_sort(original)
]

expected: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9]
all_match: bool = True
for r in results:
    for i in range(len(r)):
        if r[i] != expected[i]:
            all_match = False
            break

print(b"All sorts match:", all_match)
