# Comprehensive test for bytes operations
# Testing: basic creation, concatenation, comparison, repetition

# ============================================================================
# 1. Basic bytes creation and assignment
# ============================================================================
print(b"1. Basic creation:")
empty: bytes = b""
single: bytes = b"x"
hello: bytes = b"Hello"
world: bytes = b"World"
print(hello)
print(world)

# ============================================================================
# 2. Bytes concatenation (+)
# ============================================================================
print(b"2. Concatenation:")
concat1: bytes = hello + b" " + world
print(concat1)
concat2: bytes = b"Type" + b"Python"
print(concat2)
concat3: bytes = b"a" + b"b" + b"c" + b"d" + b"e"
print(concat3)

# ============================================================================
# 3. Bytes repetition (*)
# ============================================================================
print(b"3. Repetition:")
repeat1: bytes = b"ab" * 3
print(repeat1)
repeat2: bytes = b"-" * 5
print(repeat2)
repeat3: bytes = b"xyz" * 0
print(b"empty repeat len:", len(repeat3))
repeat4: bytes = b"!" * 1
print(repeat4)
repeat5: bytes = b"xy" * 4
print(repeat5)

# ============================================================================
# 4. Bytes equality (==)
# ============================================================================
print(b"4. Equality:")
s1: bytes = b"test"
s2: bytes = b"test"
s3: bytes = b"TEST"
s4: bytes = b"other"
eq1: bool = s1 == s2
eq2: bool = s1 == s3
eq3: bool = s1 == s4
eq4: bool = b"" == b""
print(eq1)
print(eq2)
print(eq3)
print(eq4)

# ============================================================================
# 5. Bytes inequality (!=)
# ============================================================================
print(b"5. Inequality:")
ne1: bool = s1 != s2
ne2: bool = s1 != s3
ne3: bool = s1 != s4
ne4: bool = b"a" != b"b"
print(ne1)
print(ne2)
print(ne3)
print(ne4)

# ============================================================================
# 6. Bytes ordering comparisons (<, <=, >, >=)
# ============================================================================
print(b"6. Ordering comparisons:")
lt1: bool = b"apple" < b"banana"
lt2: bool = b"zebra" < b"apple"
lt3: bool = b"abc" < b"abd"
print(lt1)
print(lt2)
print(lt3)

le1: bool = b"apple" <= b"apple"
le2: bool = b"apple" <= b"banana"
le3: bool = b"banana" <= b"apple"
print(le1)
print(le2)
print(le3)

gt1: bool = b"banana" > b"apple"
gt2: bool = b"apple" > b"banana"
gt3: bool = b"abd" > b"abc"
print(gt1)
print(gt2)
print(gt3)

ge1: bool = b"apple" >= b"apple"
ge2: bool = b"banana" >= b"apple"
ge3: bool = b"apple" >= b"banana"
print(ge1)
print(ge2)
print(ge3)

# ============================================================================
# 7. Bytes length - len()
# ============================================================================
print(b"7. Length:")
len1: int = len(b"Hello")
len2: int = len(b"")
len3: int = len(b"TypePython Compiler")
print(len1)
print(len2)
print(len3)

# ============================================================================
# 8. Bytes contains (in operator)
# ============================================================================
print(b"8. Contains:")
haystack: bytes = b"Hello World"
in1: bool = b"World" in haystack
in2: bool = b"Python" in haystack
in3: bool = b"" in haystack
in4: bool = b"llo" in haystack
print(in1)
print(in2)
print(in3)
print(in4)

# ============================================================================
# 9. Bytes not in operator
# ============================================================================
print(b"9. Not in:")
notin1: bool = b"Python" not in haystack
notin2: bool = b"World" not in haystack
print(notin1)
print(notin2)

# ============================================================================
# 10. Escape sequences
# ============================================================================
print(b"10. Escape sequences:")
esc1: bytes = b"Line1\nLine2"
esc2: bytes = b"Tab\there"
esc3: bytes = b"Quote\"here"
esc4: bytes = b"Back\\slash"
esc5: bytes = b"Bell\a"
esc6: bytes = b"Hex:\x41\x42"
print(esc1)
print(esc2)
print(esc3)
print(esc4)
print(len(esc5))
print(esc6)

# ============================================================================
# 11. Complex concatenation expressions
# ============================================================================
print(b"11. Complex expressions:")
a: bytes = b"A"
b_var: bytes = b"B"
c: bytes = b"C"
complex1: bytes = a + b_var + c
complex2: bytes = (a + b_var) + c
complex3: bytes = a + (b_var + c)
print(complex1)
print(complex2)
print(complex3)

# ============================================================================
# 12. Empty bytes edge cases
# ============================================================================
print(b"12. Empty edge cases:")
empty_concat: bytes = b"" + b""
empty_repeat: bytes = b"" * 100
empty_eq: bool = b"" == b""
empty_len: int = len(b"")
print(len(empty_concat))
print(len(empty_repeat))
print(empty_eq)
print(empty_len)

# ============================================================================
# 13. Bytes in conditionals
# ============================================================================
print(b"13. Conditionals:")
cond_str: bytes = b"test"
if cond_str == b"test":
    print(b"matched test")
else:
    print(b"no match")

if b"es" in cond_str:
    print(b"contains es")
else:
    print(b"no contain")

# ============================================================================
# 14. Bytes comparison edge cases
# ============================================================================
print(b"14. Comparison edge cases:")
cmp1: bool = b"a" < b"aa"
cmp2: bool = b"aa" < b"a"
cmp3: bool = b"" < b"a"
cmp4: bool = b"a" < b""
print(cmp1)
print(cmp2)
print(cmp3)
print(cmp4)

# ============================================================================
# 15. Multiple repetitions chained
# ============================================================================
print(b"15. Chained repetition:")
chain1: bytes = b"a" * 2 + b"b" * 3
print(chain1)
chain2: bytes = (b"x" + b"y") * 3
print(chain2)

# ============================================================================
# 16. Bytes with numbers
# ============================================================================
print(b"16. With numbers:")
num1: bytes = b"123"
num2: bytes = b"abc"
num_concat: bytes = num1 + num2
print(num_concat)

# ============================================================================
# 17. Long concatenation
# ============================================================================
print(b"17. Long concat:")
long1: bytes = b"a" + b"b" + b"c" + b"d" + b"e" + b"f" + b"g" + b"h"
print(long1)

# ============================================================================
# 18. Repetition with zero and one
# ============================================================================
print(b"18. Rep zero/one:")
zero_rep: bytes = b"test" * 0
one_rep: bytes = b"test" * 1
print(len(zero_rep))
print(one_rep)

# ============================================================================
# 19. Mixed comparisons
# ============================================================================
print(b"19. Mixed comps:")
mix1: bool = b"abc" == b"abc"
mix2: bool = b"abc" != b"def"
mix3: bool = b"abc" < b"def"
mix4: bool = b"def" > b"abc"
print(mix1)
print(mix2)
print(mix3)
print(mix4)

# ============================================================================
# 20. Large repetition
# ============================================================================
print(b"20. Large rep:")
large_rep: bytes = b"x" * 20
print(len(large_rep))

# ============================================================================
# 21. Contains empty string
# ============================================================================
print(b"21. Empty contains:")
empty_in1: bool = b"" in b"hello"
empty_in2: bool = b"" in b""
print(empty_in1)
print(empty_in2)

# ============================================================================
# 22. Case sensitivity
# ============================================================================
print(b"22. Case sens:")
case1: bool = b"Hello" == b"hello"
case2: bool = b"HELLO" == b"hello"
case3: bool = b"Hello" < b"hello"
print(case1)
print(case2)
print(case3)

# ============================================================================
# 23. Single char operations
# ============================================================================
print(b"23. Single char:")
ch1: bytes = b"a"
ch2: bytes = b"b"
ch_concat: bytes = ch1 + ch2
ch_rep: bytes = ch1 * 5
ch_eq: bool = ch1 == b"a"
print(ch_concat)
print(ch_rep)
print(ch_eq)

# ============================================================================
# 24. Whitespace handling
# ============================================================================
print(b"24. Whitespace:")
ws1: bytes = b"   "
ws2: bytes = b"hello world"
ws_len: int = len(ws1)
print(ws_len)
print(ws2)

# ============================================================================
# 25. Bytes indexing
# ============================================================================
print(b"25. Indexing:")
idx_str: bytes = b"Hello"
idx0: int = idx_str[0]
idx1: int = idx_str[1]
idx4: int = idx_str[4]
print(idx0)
print(idx1)
print(idx4)

# ============================================================================
# 26. Negative indexing
# ============================================================================
print(b"26. Neg index:")
neg_str: bytes = b"World"
neg1: int = neg_str[-1]
neg2: int = neg_str[-2]
neg5: int = neg_str[-5]
print(neg1)
print(neg2)
print(neg5)

# ============================================================================
# 27. Multiple contains checks
# ============================================================================
print(b"27. Multi contains:")
sentence: bytes = b"The quick brown fox"
c1: bool = b"quick" in sentence
c2: bool = b"slow" in sentence
c3: bool = b"The" in sentence
c4: bool = b"fox" in sentence
print(c1)
print(c2)
print(c3)
print(c4)

# ============================================================================
# 28. Repeated patterns
# ============================================================================
print(b"28. Patterns:")
pattern1: bytes = b"ab" * 5
pattern2: bytes = b"xyz" * 3
pattern3: bytes = b"--" * 4
print(pattern1)
print(pattern2)
print(pattern3)

# ============================================================================
# 29. Comparison with self
# ============================================================================
print(b"29. Self compare:")
self_str: bytes = b"test"
self_eq: bool = self_str == self_str
self_ne: bool = self_str != self_str
self_lt: bool = self_str < self_str
self_le: bool = self_str <= self_str
print(self_eq)
print(self_ne)
print(self_lt)
print(self_le)

# ============================================================================
# 30. Final complex test
# ============================================================================
print(b"30. Final test:")
final1: bytes = b"Start" + b"_" * 3 + b"End"
final2: bool = b"___" in final1
print(final1)
print(final2)

# ============================================================================
# 31. Basic slicing
# ============================================================================
print(b"31. Basic slicing:")
slice_str: bytes = b"Hello World"
s1: bytes = slice_str[0:5]
s2: bytes = slice_str[6:11]
s3: bytes = slice_str[0:1]
print(s1)
print(s2)
print(s3)

# ============================================================================
# 32. Slice with omitted bounds
# ============================================================================
print(b"32. Slice omit bounds:")
omit_str: bytes = b"TypePython"
om1: bytes = omit_str[:4]
om2: bytes = omit_str[4:]
om3: bytes = omit_str[:]
print(om1)
print(om2)
print(om3)

# ============================================================================
# 33. Negative slice indices
# ============================================================================
print(b"33. Negative slices:")
negsl_str: bytes = b"abcdefgh"
ns1: bytes = negsl_str[-3:]
ns2: bytes = negsl_str[:-3]
ns3: bytes = negsl_str[-5:-2]
print(ns1)
print(ns2)
print(ns3)

# ============================================================================
# 34. Slice with step
# ============================================================================
print(b"34. Slice step:")
step_str: bytes = b"abcdefghij"
st1: bytes = step_str[::2]
st2: bytes = step_str[1::2]
st3: bytes = step_str[0:6:2]
st4: bytes = step_str[::3]
print(st1)
print(st2)
print(st3)
print(st4)

# ============================================================================
# 35. Reverse with negative step
# ============================================================================
print(b"35. Negative step:")
rev_str: bytes = b"Hello"
rv1: bytes = rev_str[::-1]
rv2: bytes = rev_str[::-2]
rv3: bytes = rev_str[4:1:-1]
rv4: bytes = rev_str[3::-1]
print(rv1)
print(rv2)
print(rv3)
print(rv4)

# ============================================================================
# 36. Bytes in boolean context
# ============================================================================
print(b"36. Boolean context:")
non_empty: bytes = b"hello"
empty: bytes = b""
if non_empty:
    print(b"non_empty is truthy")
else:
    print(b"non_empty is falsy")
if empty:
    print(b"empty is truthy")
else:
    print(b"empty is falsy")
