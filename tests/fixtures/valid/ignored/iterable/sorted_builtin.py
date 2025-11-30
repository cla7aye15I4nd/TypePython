# Test sorted() builtin

# Sort list of integers
nums: list[int] = [3, 1, 4, 1, 5, 9, 2, 6]
sorted_nums: list[int] = sorted(nums)
print(b"Sorted:", sorted_nums)
print(b"Original:", nums)

# Sort with reverse
reversed_nums: list[int] = sorted(nums, reverse=True)
print(b"Reversed:", reversed_nums)

# Sort strings
words: list[str] = ["banana", "apple", "cherry", "date"]
sorted_words: list[str] = sorted(words)
print(b"Sorted words:", sorted_words)

# Sort with key function
def by_length(s: str) -> int:
    return len(s)

by_len: list[str] = sorted(words, key=by_length)
print(b"By length:", by_len)

# Sort with key and reverse
by_len_rev: list[str] = sorted(words, key=by_length, reverse=True)
print(b"By length reversed:", by_len_rev)

# Sort set (returns list)
num_set: set[int] = {5, 2, 8, 1, 9}
sorted_set: list[int] = sorted(num_set)
print(b"Sorted set:", sorted_set)

# Sort dict keys
d: dict[str, int] = {"c": 3, "a": 1, "b": 2}
sorted_keys: list[str] = sorted(d)
print(b"Sorted dict keys:", sorted_keys)

# Sort string (returns list of chars)
s: str = "python"
sorted_chars: list[str] = sorted(s)
print(b"Sorted chars:", sorted_chars)
