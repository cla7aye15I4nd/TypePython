# Test iteration patterns in string algorithms

# Reverse string
def reverse_str(s: str) -> str:
    result: str = ""
    for c in s:
        result = c + result
    return result

print(b"Reverse 'hello':", reverse_str("hello"))

# Reverse words
def reverse_words(s: str) -> str:
    words: list[str] = s.split()
    reversed_words: list[str] = []
    for w in reversed(words):
        reversed_words.append(w)
    return " ".join(reversed_words)

print(b"Reverse words:", reverse_words("hello world python"))

# Palindrome check
def is_palindrome(s: str) -> bool:
    n: int = len(s)
    for i in range(n // 2):
        if s[i] != s[n - 1 - i]:
            return False
    return True

print(b"Palindrome 'radar':", is_palindrome("radar"))
print(b"Palindrome 'hello':", is_palindrome("hello"))

# Longest palindromic substring
def longest_palindrome(s: str) -> str:
    if len(s) == 0:
        return ""

    longest: str = s[0]
    for i in range(len(s)):
        # Odd length
        left: int = i
        right: int = i
        while left >= 0 and right < len(s) and s[left] == s[right]:
            if right - left + 1 > len(longest):
                longest = s[left:right + 1]
            left = left - 1
            right = right + 1

        # Even length
        left = i
        right = i + 1
        while left >= 0 and right < len(s) and s[left] == s[right]:
            if right - left + 1 > len(longest):
                longest = s[left:right + 1]
            left = left - 1
            right = right + 1

    return longest

print(b"Longest palindrome in 'babad':", longest_palindrome("babad"))

# Anagram check
def is_anagram(s1: str, s2: str) -> bool:
    if len(s1) != len(s2):
        return False

    freq1: dict[str, int] = {}
    freq2: dict[str, int] = {}

    for c in s1:
        freq1[c] = freq1.get(c, 0) + 1

    for c in s2:
        freq2[c] = freq2.get(c, 0) + 1

    for k, v in freq1.items():
        if freq2.get(k, 0) != v:
            return False

    return True

print(b"Anagram 'listen' 'silent':", is_anagram("listen", "silent"))
print(b"Anagram 'hello' 'world':", is_anagram("hello", "world"))

# Character frequency
def char_frequency(s: str) -> dict[str, int]:
    freq: dict[str, int] = {}
    for c in s:
        freq[c] = freq.get(c, 0) + 1
    return freq

print(b"Frequency:", char_frequency("mississippi"))

# First non-repeating character
def first_non_repeating(s: str) -> str:
    freq: dict[str, int] = {}
    for c in s:
        freq[c] = freq.get(c, 0) + 1

    for c in s:
        if freq[c] == 1:
            return c

    return ""

print(b"First non-repeating in 'leetcode':", first_non_repeating("leetcode"))

# Longest common prefix
def longest_common_prefix(strs: list[str]) -> str:
    if len(strs) == 0:
        return ""

    prefix: str = strs[0]
    for s in strs[1:]:
        while not s.startswith(prefix):
            prefix = prefix[:-1]
            if len(prefix) == 0:
                return ""

    return prefix

print(b"Common prefix:", longest_common_prefix(["flower", "flow", "flight"]))

# Count substrings
def count_substring(s: str, sub: str) -> int:
    count: int = 0
    for i in range(len(s) - len(sub) + 1):
        if s[i:i + len(sub)] == sub:
            count = count + 1
    return count

print(b"Count 'is' in 'mississippi':", count_substring("mississippi", "is"))

# Run-length encoding
def rle_encode(s: str) -> str:
    if len(s) == 0:
        return ""

    result: str = ""
    current: str = s[0]
    count: int = 1

    for i in range(1, len(s)):
        if s[i] == current:
            count = count + 1
        else:
            result = result + current + str(count)
            current = s[i]
            count = 1

    result = result + current + str(count)
    return result

print(b"RLE encode 'aaabbbcc':", rle_encode("aaabbbcc"))

# Run-length decoding
def rle_decode(s: str) -> str:
    result: str = ""
    i: int = 0
    while i < len(s):
        char: str = s[i]
        i = i + 1
        num_str: str = ""
        while i < len(s) and s[i].isdigit():
            num_str = num_str + s[i]
            i = i + 1
        count: int = int(num_str)
        for _ in range(count):
            result = result + char
    return result

print(b"RLE decode 'a3b3c2':", rle_decode("a3b3c2"))

# Longest substring without repeating
def longest_unique_substring(s: str) -> int:
    max_len: int = 0
    start: int = 0
    seen: dict[str, int] = {}

    for i in range(len(s)):
        c: str = s[i]
        if c in seen and seen[c] >= start:
            start = seen[c] + 1
        seen[c] = i
        if i - start + 1 > max_len:
            max_len = i - start + 1

    return max_len

print(b"Longest unique in 'abcabcbb':", longest_unique_substring("abcabcbb"))

# String rotation check
def is_rotation(s1: str, s2: str) -> bool:
    if len(s1) != len(s2):
        return False
    combined: str = s1 + s1
    return s2 in combined

print(b"Rotation 'abcde' 'cdeab':", is_rotation("abcde", "cdeab"))

# Compress string
def compress(s: str) -> str:
    if len(s) == 0:
        return ""

    result: str = ""
    count: int = 1

    for i in range(1, len(s)):
        if s[i] == s[i - 1]:
            count = count + 1
        else:
            result = result + s[i - 1]
            if count > 1:
                result = result + str(count)
            count = 1

    result = result + s[-1]
    if count > 1:
        result = result + str(count)

    if len(result) >= len(s):
        return s
    return result

print(b"Compress 'aabcccccaaa':", compress("aabcccccaaa"))

# Find all permutations
def permutations(s: str) -> list[str]:
    if len(s) <= 1:
        return [s]

    result: list[str] = []
    for i in range(len(s)):
        char: str = s[i]
        remaining: str = s[:i] + s[i + 1:]
        for perm in permutations(remaining):
            result.append(char + perm)

    return result

print(b"Permutations of 'abc':", permutations("abc"))
