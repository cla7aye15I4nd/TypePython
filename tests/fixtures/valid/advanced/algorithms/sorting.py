# Sorting algorithm (bubble sort on fixed array)
def bubble_sort_sum() -> int:
    # Simulate sorting by computing what the sum would be
    # This tests complex loop logic
    n1: int = 64
    n2: int = 34
    n3: int = 25
    n4: int = 12
    n5: int = 22

    # Sorting network for 5 elements
    if n1 > n2:
        temp: int = n1
        n1 = n2
        n2 = temp

    if n3 > n4:
        temp: int = n3
        n3 = n4
        n4 = temp

    if n1 > n3:
        temp: int = n1
        n1 = n3
        n3 = temp

    if n2 > n4:
        temp: int = n2
        n2 = n4
        n4 = temp

    if n2 > n3:
        temp: int = n2
        n2 = n3
        n3 = temp

    if n4 > n5:
        temp: int = n4
        n4 = n5
        n5 = temp

    if n3 > n4:
        temp: int = n3
        n3 = n4
        n4 = temp

    if n2 > n3:
        temp: int = n2
        n2 = n3
        n3 = temp

    return n1 + n2 + n3 + n4 + n5

result: int = bubble_sort_sum()
print("Sorted sum:", result)
