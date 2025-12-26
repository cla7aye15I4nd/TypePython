# Data structure tests module
from datastructure.hashmap import test_hashmap_basic, test_hashmap_update, test_hashmap_contains
from datastructure.hashset import test_hashset_basic, test_hashset_contains
from datastructure.bst import test_bst_insert, test_bst_contains
from datastructure.heap import test_heap_basic, test_heap_pop, test_heap_sort

def test() -> int:
    # Data structure tests - HashMap
    print(test_hashmap_basic())      # 200
    print(test_hashmap_update())     # 999
    print(test_hashmap_contains())   # 2

    # Data structure tests - HashSet
    print(test_hashset_basic())      # 3
    print(test_hashset_contains())   # 2

    # Data structure tests - Binary Search Tree
    print(test_bst_insert())         # 5
    print(test_bst_contains())       # 2

    # Data structure tests - MinHeap
    print(test_heap_basic())         # 1
    print(test_heap_pop())           # 4
    print(test_heap_sort())          # 6

    return 0
