# MinHeap: Min heap using a list

class MinHeap:
    items: list[int]
    size: int

    def __init__(self) -> None:
        self.items = [0]
        self.size = 0

    def push(self, value: int) -> None:
        if self.size == 0:
            self.items[0] = value
        else:
            self.items.append(value)
        self.size = self.size + 1
        # Bubble up
        idx: int = self.size - 1
        while idx > 0:
            parent_idx: int = (idx - 1) // 2
            if self.items[idx] < self.items[parent_idx]:
                # Swap
                temp: int = self.items[idx]
                self.items[idx] = self.items[parent_idx]
                self.items[parent_idx] = temp
                idx = parent_idx
            else:
                idx = 0  # Exit loop

    def peek(self) -> int:
        if self.size == 0:
            return 0
        return self.items[0]

    def pop(self) -> int:
        if self.size == 0:
            return 0

        result: int = self.items[0]
        self.size = self.size - 1

        if self.size == 0:
            return result

        # Move last to root
        self.items[0] = self.items[self.size]

        # Bubble down
        idx: int = 0
        done: int = 0
        while done == 0:
            left_idx: int = 2 * idx + 1
            right_idx: int = 2 * idx + 2
            smallest: int = idx

            if left_idx < self.size:
                if self.items[left_idx] < self.items[smallest]:
                    smallest = left_idx

            if right_idx < self.size:
                if self.items[right_idx] < self.items[smallest]:
                    smallest = right_idx

            if smallest == idx:
                done = 1
            else:
                # Swap
                temp: int = self.items[idx]
                self.items[idx] = self.items[smallest]
                self.items[smallest] = temp
                idx = smallest

        return result

    def get_size(self) -> int:
        return self.size


def test_heap_basic() -> int:
    h: MinHeap = MinHeap()
    h.push(5)
    h.push(3)
    h.push(7)
    h.push(1)
    return h.peek()  # Expected: 1 (minimum)


def test_heap_pop() -> int:
    h: MinHeap = MinHeap()
    h.push(5)
    h.push(3)
    h.push(7)
    h.push(1)
    first: int = h.pop()   # 1
    second: int = h.pop()  # 3
    return first + second  # Expected: 4


def test_heap_sort() -> int:
    # Use heap to sort: 4, 2, 6, 1, 3 -> 1, 2, 3, 4, 6
    h: MinHeap = MinHeap()
    h.push(4)
    h.push(2)
    h.push(6)
    h.push(1)
    h.push(3)
    # Pop all and sum first 3 (should be 1+2+3=6)
    a: int = h.pop()
    b: int = h.pop()
    c: int = h.pop()
    return a + b + c  # Expected: 6
