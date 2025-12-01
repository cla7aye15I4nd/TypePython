# Test comprehensive reversed patterns

# Reversed list
nums: list[int] = [1, 2, 3, 4, 5]
for n in reversed(nums):
    print(b"Reversed:", n)

# Original unchanged
print(b"Original:", nums)

# Reversed string
text: str = "hello"
for c in reversed(text):
    print(b"Char:", c)

# Reversed range
for i in reversed(range(5)):
    print(b"Rev range:", i)

for i in reversed(range(10, 20)):
    print(b"Rev range 10-20:", i)

for i in reversed(range(0, 10, 2)):
    print(b"Rev evens:", i)

# Reversed tuple
t: tuple[int, int, int, int, int] = (1, 2, 3, 4, 5)
for x in reversed(t):
    print(b"Rev tuple:", x)

# Collect reversed into list
original: list[str] = ["a", "b", "c", "d"]
rev_list: list[str] = []
for item in reversed(original):
    rev_list.append(item)
print(b"Rev list:", rev_list)

# Reversed with enumerate
for i, val in enumerate(reversed([10, 20, 30, 40, 50])):
    print(b"Enum rev:", i, val)

# Reversed with zip
a: list[int] = [1, 2, 3]
b: list[int] = [10, 20, 30]
for x, y in zip(reversed(a), reversed(b)):
    print(b"Zip rev:", x, y)

# Reversed bytes
for b in reversed(b"hello"):
    print(b"Rev byte:", b)

# Build reversed string
s: str = "python"
rev_s: str = ""
for c in reversed(s):
    rev_s = rev_s + c
print(b"Rev string:", rev_s)

# Reversed nested iteration
matrix: list[list[int]] = [[1, 2], [3, 4], [5, 6]]
for row in reversed(matrix):
    for val in reversed(row):
        print(b"Rev matrix:", val)

# Reversed with condition
for x in reversed([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]):
    if x % 2 == 0:
        print(b"Rev even:", x)

# Reversed with break
for x in reversed([1, 2, 3, 4, 5]):
    if x == 3:
        print(b"Found 3, break")
        break
    print(b"Rev:", x)

# Reversed custom iterator
class Countdown:
    n: int
    current: int

    def __init__(self, n: int) -> None:
        self.n: int = n
        self.current: int = n

    def __iter__(self) -> 'Countdown':
        self.current = self.n
        return self

    def __next__(self) -> int:
        if self.current < 0:
            raise StopIteration
        val: int = self.current
        self.current = self.current - 1
        return val

    def __reversed__(self) -> list[int]:
        result: list[int] = []
        for i in range(self.n + 1):
            result.append(i)
        return result

cd: Countdown = Countdown(5)
for x in reversed(cd):
    print(b"Rev countdown:", x)

# Double reversal (back to original order)
data: list[int] = [1, 2, 3, 4, 5]
temp: list[int] = []
for x in reversed(data):
    temp.append(x)
final: list[int] = []
for x in reversed(temp):
    final.append(x)
print(b"Double rev:", final)

# Reversed with sum
total: int = 0
for x in reversed([1, 2, 3, 4, 5]):
    total = total + x
print(b"Sum reversed:", total)

# Reversed palindrome check
s2: str = "radar"
is_palindrome: bool = True
chars: list[str] = []
for c in s2:
    chars.append(c)
for c1, c2 in zip(s2, reversed(chars)):
    if c1 != c2:
        is_palindrome = False
        break
print(b"Is palindrome:", is_palindrome)
