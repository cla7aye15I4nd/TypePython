# Large-scale stress tests using random number generation
from rng.random import RNG, make_rand_list
from basic.primitives.operators import test_add, test_sub, test_mult, test_mod
from basic.primitives.operators import test_eq, test_lt, test_gt
from basic.primitives.binops import test_bitand, test_bitor, test_bitxor
from algorithm.factorial import factorial
from algorithm.fibonacci import fibonacci
from datastructure.hashmap import HashMap
from datastructure.hashset import HashSet
from datastructure.bst import BinaryTree
from datastructure.heap import MinHeap

# ============================================================
# Arithmetic stress tests - run many operations with random inputs
# ============================================================

def stress_arithmetic(iterations: int, seed: int) -> int:
    # Run many arithmetic operations and return a checksum
    rng: RNG = RNG(seed)
    checksum: int = 0
    i: 'int' = 0
    while i < iterations:
        a: int = rng.rand_range(1, 1000)
        b: int = rng.rand_range(1, 100)

        # Test all basic operators
        checksum = checksum + test_add(a, b)
        checksum = checksum + test_sub(a, b)
        checksum = checksum + test_mult(a, b)
        checksum = checksum + test_mod(a, b)

        # Keep checksum in reasonable bounds
        checksum = checksum % 1000000007
        i = i + 1
    return checksum


def stress_comparisons(iterations: int, seed: int) -> int:
    # Run many comparison operations
    rng: RNG = RNG(seed)
    true_count: int = 0
    i: int = 0
    while i < iterations:
        a: int = rng.rand_range(0, 100)
        b: int = rng.rand_range(0, 100)

        true_count = true_count + test_eq(a, b)
        true_count = true_count + test_lt(a, b)
        true_count = true_count + test_gt(a, b)

        i = i + 1
    return true_count


def stress_bitwise(iterations: int, seed: int) -> int:
    # Run many bitwise operations
    rng: RNG = RNG(seed)
    checksum: int = 0
    i: int = 0
    while i < iterations:
        a: int = rng.rand_range(0, 65535)
        b: int = rng.rand_range(0, 65535)

        checksum = checksum + test_bitand(a, b)
        checksum = checksum + test_bitor(a, b)
        checksum = checksum + test_bitxor(a, b)

        checksum = checksum % 1000000007
        i = i + 1
    return checksum


# ============================================================
# List stress tests - large lists and many operations
# ============================================================

def stress_list_sum(size: int, seed: int) -> int:
    # Create a large list and sum all elements
    rng: RNG = RNG(seed)
    nums: list[int] = make_rand_list(rng, size, 1, 100)

    total: int = 0
    i: int = 0
    while i < len(nums):
        total = total + nums[i]
        i = i + 1
    return total


def stress_list_modify(size: int, iterations: int, seed: int) -> int:
    # Create a list and modify elements many times
    rng: RNG = RNG(seed)
    nums: list[int] = make_rand_list(rng, size, 0, 100)

    i: int = 0
    while i < iterations:
        idx: int = rng.rand_range(0, size - 1)
        val: int = rng.rand_range(0, 1000)
        nums[idx] = val
        i = i + 1

    # Return sum as verification
    total: int = 0
    j: int = 0
    while j < len(nums):
        total = total + nums[j]
        j = j + 1
    return total % 1000000007


def stress_list_search(size: int, iterations: int, seed: int) -> int:
    # Create a list and search for elements many times
    rng: RNG = RNG(seed)
    nums: list[int] = make_rand_list(rng, size, 0, 1000)

    found_count: int = 0
    i: int = 0
    while i < iterations:
        target: int = rng.rand_range(0, 1000)

        # Linear search
        j: int = 0
        while j < len(nums):
            if nums[j] == target:
                found_count = found_count + 1
            j = j + 1

        i = i + 1
    return found_count


def stress_list_max(size: int, seed: int) -> int:
    # Find maximum in a large list
    rng: RNG = RNG(seed)
    nums: list[int] = make_rand_list(rng, size, 0, 1000000)

    max_val: int = nums[0]
    i: int = 1
    while i < len(nums):
        if nums[i] > max_val:
            max_val = nums[i]
        i = i + 1
    return max_val


def stress_list_sort_check(size: int, seed: int) -> int:
    # Create list, bubble sort it, verify sorted
    rng: RNG = RNG(seed)
    nums: list[int] = make_rand_list(rng, size, 0, 10000)

    # Bubble sort (inefficient but tests nested loops heavily)
    n: int = len(nums)
    i: int = 0
    while i < n - 1:
        j: int = 0
        while j < n - i - 1:
            if nums[j] > nums[j + 1]:
                temp: int = nums[j]
                nums[j] = nums[j + 1]
                nums[j + 1] = temp
            j = j + 1
        i = i + 1

    # Verify sorted
    sorted_ok: int = 1
    k: int = 0
    while k < n - 1:
        if nums[k] > nums[k + 1]:
            sorted_ok = 0
        k = k + 1
    return sorted_ok


# ============================================================
# Recursion stress tests
# ============================================================

def stress_factorial(iterations: int, seed: int) -> int:
    # Call factorial many times with various inputs
    rng: RNG = RNG(seed)
    checksum: int = 0
    i: int = 0
    while i < iterations:
        n: int = rng.rand_range(1, 12)  # Keep small to avoid overflow
        result: int = factorial(n)
        checksum = (checksum + result) % 1000000007
        i = i + 1
    return checksum


def stress_fibonacci(iterations: int, seed: int) -> int:
    # Call fibonacci many times
    rng: RNG = RNG(seed)
    checksum: int = 0
    i: int = 0
    while i < iterations:
        n: int = rng.rand_range(1, 20)  # Keep small due to exponential growth
        result: int = fibonacci(n)
        checksum = (checksum + result) % 1000000007
        i = i + 1
    return checksum


# ============================================================
# Class/object stress tests
# ============================================================

class StressPoint:
    x: int
    y: int

    def __init__(self, px: int, py: int) -> None:
        self.x = px
        self.y = py

    def dist_squared(self) -> int:
        return self.x * self.x + self.y * self.y

    def add(self, other: "StressPoint") -> "StressPoint":
        return StressPoint(self.x + other.x, self.y + other.y)


def stress_class_creation(iterations: int, seed: int) -> int:
    # Create many class instances
    rng: RNG = RNG(seed)
    total: int = 0
    i: int = 0
    while i < iterations:
        x: int = rng.rand_range(0, 100)
        y: int = rng.rand_range(0, 100)
        p: StressPoint = StressPoint(x, y)
        total = total + p.dist_squared()
        total = total % 1000000007
        i = i + 1
    return total


def stress_class_list(size: int, seed: int) -> int:
    # Create a list of class instances and operate on them
    rng: RNG = RNG(seed)
    dummy: StressPoint = StressPoint(0, 0)
    points: list[StressPoint] = [dummy]
    points_size: int = 0

    i: int = 0
    while i < size:
        x: int = rng.rand_range(0, 100)
        y: int = rng.rand_range(0, 100)
        if points_size == 0:
            points[0] = StressPoint(x, y)
        else:
            points.append(StressPoint(x, y))
        points_size = points_size + 1
        i = i + 1

    # Sum all distances
    total: int = 0
    j: int = 0
    while j < len(points):
        total = total + points[j].dist_squared()
        j = j + 1
    return total % 1000000007


def stress_class_modify(size: int, iterations: int, seed: int) -> int:
    # Create class instances and modify their fields
    rng: RNG = RNG(seed)
    dummy: StressPoint = StressPoint(0, 0)
    points: list[StressPoint] = [dummy]
    points_size: int = 0

    i: int = 0
    while i < size:
        if points_size == 0:
            points[0] = StressPoint(0, 0)
        else:
            points.append(StressPoint(0, 0))
        points_size = points_size + 1
        i = i + 1

    # Randomly modify points
    j: int = 0
    while j < iterations:
        idx: int = rng.rand_range(0, size - 1)
        points[idx].x = rng.rand_range(0, 1000)
        points[idx].y = rng.rand_range(0, 1000)
        j = j + 1

    # Sum all x values
    total: int = 0
    k: int = 0
    while k < len(points):
        total = total + points[k].x
        k = k + 1
    return total % 1000000007


# ============================================================
# Nested loop stress tests
# ============================================================

def stress_nested_loops(n: int) -> int:
    # Triple nested loop - O(n^3) complexity
    total: int = 0
    i: int = 0
    while i < n:
        j: int = 0
        while j < n:
            k: int = 0
            while k < n:
                total = total + 1
                k = k + 1
            j = j + 1
        i = i + 1
    return total


def stress_matrix_mult(n: int, seed: int) -> int:
    # Simulate n x n matrix multiplication
    rng: RNG = RNG(seed)

    # Create two "matrices" as flat lists
    a: list[int] = make_rand_list(rng, n * n, 0, 10)
    b: list[int] = make_rand_list(rng, n * n, 0, 10)

    # Compute one cell of result (just for testing, not full mult)
    result: int = 0
    i: int = 0
    while i < n:
        result = result + a[i] * b[i * n]
        i = i + 1
    return result


# ============================================================
# HashMap stress tests
# ============================================================

def stress_hashmap_insert(size: int, seed: int) -> int:
    # Insert many key-value pairs
    rng: RNG = RNG(seed)
    m: HashMap = HashMap()

    i: int = 0
    while i < size:
        key: int = rng.rand_range(0, size * 2)
        val: int = rng.rand_range(0, 10000)
        m.put(key, val)
        i = i + 1

    return m.get_size()


def stress_hashmap_lookup(size: int, lookups: int, seed: int) -> int:
    # Insert keys then perform many lookups
    rng: RNG = RNG(seed)
    m: HashMap = HashMap()

    # Insert known keys
    i: int = 0
    while i < size:
        m.put(i, i * 10)
        i = i + 1

    # Random lookups
    total: int = 0
    j: int = 0
    while j < lookups:
        key: int = rng.rand_range(0, size * 2)
        total = total + m.get(key)
        j = j + 1

    return total % 1000000007


def stress_hashmap_update(size: int, updates: int, seed: int) -> int:
    # Insert then update values many times
    rng: RNG = RNG(seed)
    m: HashMap = HashMap()

    # Initial insert
    i: int = 0
    while i < size:
        m.put(i, 0)
        i = i + 1

    # Many updates
    j: int = 0
    while j < updates:
        key: int = rng.rand_range(0, size)
        val: int = rng.rand_range(0, 1000)
        m.put(key, val)
        j = j + 1

    # Sum all values
    total: int = 0
    k: int = 0
    while k < size:
        total = total + m.get(k)
        k = k + 1

    return total % 1000000007


# ============================================================
# HashSet stress tests
# ============================================================

def stress_hashset_insert(size: int, seed: int) -> int:
    # Insert many values (some duplicates)
    rng: RNG = RNG(seed)
    s: HashSet = HashSet()

    i: int = 0
    while i < size:
        val: int = rng.rand_range(0, size // 2)  # Force duplicates
        s.add(val)
        i = i + 1

    return s.get_size()


def stress_hashset_contains(size: int, lookups: int, seed: int) -> int:
    # Insert values then check containment many times
    rng: RNG = RNG(seed)
    s: HashSet = HashSet()

    # Insert values
    i: int = 0
    while i < size:
        s.add(i * 2)  # Only even numbers
        i = i + 1

    # Check containment
    found: int = 0
    j: int = 0
    while j < lookups:
        val: int = rng.rand_range(0, size * 4)
        found = found + s.contains(val)
        j = j + 1

    return found


# ============================================================
# Binary Search Tree stress tests
# ============================================================

def stress_bst_insert(size: int, seed: int) -> int:
    # Insert many values into BST
    rng: RNG = RNG(seed)
    tree: BinaryTree = BinaryTree()

    i: int = 0
    while i < size:
        val: int = rng.rand_range(0, size * 10)
        tree.insert(val)
        i = i + 1

    return tree.get_size()


def stress_bst_search(size: int, searches: int, seed: int) -> int:
    # Build BST then search many times
    rng: RNG = RNG(seed)
    tree: BinaryTree = BinaryTree()

    # Insert values
    i: int = 0
    while i < size:
        val: int = rng.rand_range(0, size * 2)
        tree.insert(val)
        i = i + 1

    # Search for values
    found: int = 0
    j: int = 0
    while j < searches:
        val: int = rng.rand_range(0, size * 2)
        found = found + tree.contains(val)
        j = j + 1

    return found


def stress_bst_sequential(size: int) -> int:
    # Insert sequential values (worst case - becomes linked list)
    tree: BinaryTree = BinaryTree()

    i: int = 0
    while i < size:
        tree.insert(i)
        i = i + 1

    # Verify all values present
    found: int = 0
    j: int = 0
    while j < size:
        found = found + tree.contains(j)
        j = j + 1

    return found


# ============================================================
# MinHeap stress tests
# ============================================================

def stress_heap_push(size: int, seed: int) -> int:
    # Push many values and check min
    rng: RNG = RNG(seed)
    h: MinHeap = MinHeap()

    min_val: int = 1000000
    i: int = 0
    while i < size:
        val: int = rng.rand_range(0, 100000)
        if val < min_val:
            min_val = val
        h.push(val)
        i = i + 1

    # Verify heap property: peek should return minimum
    heap_min: int = h.peek()
    if heap_min == min_val:
        return 1
    return 0


def stress_heap_sort(size: int, seed: int) -> int:
    # Use heap to sort random values, verify sorted
    rng: RNG = RNG(seed)
    h: MinHeap = MinHeap()

    # Push random values
    i: int = 0
    while i < size:
        val: int = rng.rand_range(0, 10000)
        h.push(val)
        i = i + 1

    # Pop all and verify sorted order
    prev: int = h.pop()
    sorted_ok: int = 1
    j: int = 1
    while j < size:
        curr: int = h.pop()
        if curr < prev:
            sorted_ok = 0
        prev = curr
        j = j + 1

    return sorted_ok


def stress_heap_push_pop(size: int, operations: int, seed: int) -> int:
    # Mix of push and pop operations
    rng: RNG = RNG(seed)
    h: MinHeap = MinHeap()

    # Initial fill
    i: int = 0
    while i < size:
        h.push(rng.rand_range(0, 10000))
        i = i + 1

    # Random push/pop operations
    checksum: int = 0
    j: int = 0
    while j < operations:
        op: int = rng.rand_range(0, 1)
        if op == 0:
            h.push(rng.rand_range(0, 10000))
        else:
            if h.get_size() > 0:
                checksum = checksum + h.pop()
        j = j + 1

    return checksum % 1000000007


# ============================================================
# Combined stress test runner
# ============================================================

def run_all_stress_tests() -> int:
    passed: int = 0

    # Arithmetic tests (1000 iterations)
    print(stress_arithmetic(1000, 42))
    passed = passed + 1

    print(stress_comparisons(1000, 123))
    passed = passed + 1

    print(stress_bitwise(1000, 456))
    passed = passed + 1

    # List tests (size 100-500)
    print(stress_list_sum(500, 789))
    passed = passed + 1

    print(stress_list_modify(100, 1000, 111))
    passed = passed + 1

    print(stress_list_search(50, 20, 222))
    passed = passed + 1

    print(stress_list_max(1000, 333))
    passed = passed + 1

    # Sort test (smaller size due to O(n^2))
    print(stress_list_sort_check(50, 444))
    passed = passed + 1

    # Recursion tests
    print(stress_factorial(100, 555))
    passed = passed + 1

    print(stress_fibonacci(50, 666))
    passed = passed + 1

    # Class tests
    print(stress_class_creation(500, 777))
    passed = passed + 1

    print(stress_class_list(200, 888))
    passed = passed + 1

    print(stress_class_modify(50, 500, 999))
    passed = passed + 1

    # Nested loop tests
    print(stress_nested_loops(20))
    passed = passed + 1

    print(stress_matrix_mult(50, 1234))
    passed = passed + 1

    # HashMap stress tests
    print(stress_hashmap_insert(200, 2001))
    passed = passed + 1

    print(stress_hashmap_lookup(100, 500, 2002))
    passed = passed + 1

    print(stress_hashmap_update(50, 500, 2003))
    passed = passed + 1

    # HashSet stress tests
    print(stress_hashset_insert(500, 3001))
    passed = passed + 1

    print(stress_hashset_contains(100, 300, 3002))
    passed = passed + 1

    # BST stress tests
    print(stress_bst_insert(200, 4001))
    passed = passed + 1

    print(stress_bst_search(100, 200, 4002))
    passed = passed + 1

    print(stress_bst_sequential(100))
    passed = passed + 1

    # MinHeap stress tests
    print(stress_heap_push(500, 5001))
    passed = passed + 1

    print(stress_heap_sort(200, 5002))
    passed = passed + 1

    print(stress_heap_push_pop(100, 500, 5003))
    passed = passed + 1

    return passed
