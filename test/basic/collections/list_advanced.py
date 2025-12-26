# Advanced list operations
def list_len(nums: list[int]) -> int:
    print("Calculating list length")
    length: int = len(nums)
    print("List length is", length)
    return length

def list_sum(nums: list[int]) -> int:
    print("Calculating list sum")
    total: int = 0
    i: int = 0
    while i < len(nums):
        print("Adding element", nums[i])
        total += nums[i]
        i += 1
    print("Total sum:", total)
    return total

def create_and_access() -> int:
    print("Creating list with 5 elements")
    nums: list[int] = [10, 20, 30, 40, 50]
    print("Accessing first and last")
    first: int = nums[0]
    last: int = nums[4]
    print("First:", first, "Last:", last)
    result: int = first + last
    print("Sum:", result)
    return result

def nested_access(nums: list[int], idx: int) -> int:
    print("Accessing element at index", idx)
    element: int = nums[idx]
    print("Element:", element)
    return element
