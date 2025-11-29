# Comprehensive test: All data structures with all builtin functions
# Tests list, dict, set, bytes with len, min, max, abs, round, pow and all their methods

# LIST: append, extend, insert, pop, remove, clear, index, count, sort, reverse, copy
list1: list[int] = [1, 2, 3, 4, 5]
print(len(list1))  # 5
list1.append(6)
print(len(list1))  # 6
ext: list[int] = [7, 8]
list1.extend(ext)
print(len(list1))  # 8
list1.insert(0, 0)
print(list1[0])  # 0
v1: int = list1.pop(0)
print(v1)  # 0
list1.remove(2)
idx1: int = list1.index(3)
print(idx1)  # 1
cnt_list: list[int] = [1, 2, 2, 3, 2]
c1: int = cnt_list.count(2)
print(c1)  # 3
cp1: list[int] = list1.copy()
print(len(cp1))  # 7
sort1: list[int] = [5, 2, 8, 1]
sort1.sort()
print(sort1[0])  # 1
rev1: list[int] = [1, 2, 3]
rev1.reverse()
print(rev1[0])  # 3
clr1: list[int] = [1, 2, 3]
clr1.clear()
print(len(clr1))  # 0

# LIST with builtins
mm1: list[int] = [10, 20, 30]
print(min(mm1[0], mm1[1]))  # 10
print(max(mm1[1], mm1[2]))  # 30
ab1: list[int] = [-5, -10, 3]
print(abs(ab1[0]))  # 5
pw1: list[int] = [2, 3, 4]
print(pow(pw1[0], pw1[1]))  # 8
empty_list: list[int] = list()
print(len(empty_list))  # 0

# DICT: get, pop, clear, copy
dict1: dict[int, int] = {1: 10, 2: 20, 3: 30}
print(len(dict1))  # 3
v2: int = dict1.get(1, 0)
print(v2)  # 10
v3: int = dict1.get(99, -1)
print(v3)  # -1
v4: int = dict1.pop(2)
print(v4)  # 20
print(len(dict1))  # 2
cp2: dict[int, int] = dict1.copy()
print(len(cp2))  # 2
clr2: dict[int, int] = {1: 100}
clr2.clear()
print(len(clr2))  # 0

# DICT with builtins
mm2: dict[int, int] = {1: 5, 2: 10, 3: 15}
print(min(mm2[1], mm2[2]))  # 5
print(max(mm2[2], mm2[3]))  # 15
ab2: dict[int, int] = {1: -42}
print(abs(ab2[1]))  # 42
pw2: dict[int, int] = {1: 2, 2: 10}
print(pow(pw2[1], pw2[2]))  # 1024
empty_dict: dict[int, int] = dict()
print(len(empty_dict))  # 0

# SET: add, remove, discard, pop, clear, copy, union, intersection, difference,
#      symmetric_difference, issubset, issuperset, isdisjoint
set1: set[int] = {1, 2, 3, 4, 5}
print(len(set1))  # 5
set1.add(6)
print(len(set1))  # 6
set1.remove(1)
print(len(set1))  # 5
set1.discard(2)
print(len(set1))  # 4
v5: int = set1.pop()
print(len(set1))  # 3
cp3: set[int] = set1.copy()
print(len(cp3))  # 3
seta: set[int] = {1, 2, 3}
setb: set[int] = {3, 4, 5}
u1: set[int] = seta.union(setb)
print(len(u1))  # 5
i1: set[int] = seta.intersection(setb)
print(len(i1))  # 1
d1: set[int] = seta.difference(setb)
print(len(d1))  # 2
s1: set[int] = seta.symmetric_difference(setb)
print(len(s1))  # 4
b1: bool = seta.issubset(u1)
print(b1)  # True
b2: bool = u1.issuperset(seta)
print(b2)  # True
setc: set[int] = {10, 11}
b3: bool = seta.isdisjoint(setc)
print(b3)  # True
clr3: set[int] = {1, 2}
clr3.clear()
print(len(clr3))  # 0
empty_set: set[int] = set()
print(len(empty_set))  # 0

# BYTES: upper, lower, ljust, rjust, center, zfill, strip, lstrip, rstrip,
#        find, count, startswith, endswith, isalnum, isalpha, isdigit, isspace,
#        islower, isupper, replace
bytes1: bytes = b"Hello"
print(len(bytes1))  # 5
bv1: int = bytes1[0]
bv2: int = bytes1[1]
print(min(bv1, bv2))  # 72
print(max(bv1, bv2))  # 101
cnt_b: bytes = b"hello hello"
c2: int = cnt_b.count(b"hello")
print(c2)  # 2
up1: bytes = bytes1.upper()
print(len(up1))  # 5
lo1: bytes = bytes1.lower()
print(len(lo1))  # 5
st1: bytes = b"  hello  "
st2: bytes = st1.strip()
print(len(st2))  # 5
st3: bytes = st1.lstrip()
print(len(st3))  # 7
st4: bytes = st1.rstrip()
print(len(st4))  # 7
rp1: bytes = b"abc"
rp2: bytes = rp1.replace(b"a", b"x")
print(len(rp2))  # 3
b4: bool = bytes1.startswith(b"Hello")
print(b4)  # True
b5: bool = bytes1.endswith(b"lo")
print(b5)  # True
ce1: bytes = b"hi"
ce2: bytes = ce1.center(10)
print(len(ce2))  # 10
lj1: bytes = ce1.ljust(10)
print(len(lj1))  # 10
rj1: bytes = ce1.rjust(10)
print(len(rj1))  # 10
zf1: bytes = b"42"
zf2: bytes = zf1.zfill(5)
print(len(zf2))  # 5
al1: bytes = b"hello"
b6: bool = al1.isalpha()
print(b6)  # True
di1: bytes = b"123"
b7: bool = di1.isdigit()
print(b7)  # True
aln1: bytes = b"hello123"
b8: bool = aln1.isalnum()
print(b8)  # True
sp1: bytes = b"   "
b9: bool = sp1.isspace()
print(b9)  # True
iu1: bytes = b"HELLO"
b10: bool = iu1.isupper()
print(b10)  # True
il1: bytes = b"hello"
b11: bool = il1.islower()
print(b11)  # True
fi1: bytes = b"hello world"
fi2: int = fi1.find(b"world")
print(fi2)  # 6

# CROSS-STRUCTURE len() on all types
xlist: list[int] = [1, 2, 3]
xdict: dict[int, int] = {1: 10, 2: 20}
xset: set[int] = {10, 20, 30}
xbytes: bytes = b"test"
print(len(xlist))  # 3
print(len(xdict))  # 2
print(len(xset))  # 3
print(len(xbytes))  # 4

# BUILTINS: abs, round, min, max, pow
print(abs(-42))  # 42
print(abs(42))  # 42
print(round(3.7))  # 4
print(round(3.2))  # 3
print(round(3.14159, 2))  # 3.14
print(min(5, 3))  # 3
print(min(1, 2, 3))  # 1
print(max(5, 3))  # 5
print(max(1, 2, 3))  # 3
print(pow(2, 3))  # 8
print(pow(2, 10))  # 1024
print(pow(2, 3, 5))  # 3

# COMBINATIONS: Using builtins with structure elements
combo_list: list[int] = [10, -20, 30]
combo_dict: dict[int, int] = {1: 5, 2: -15}
print(abs(combo_list[1]))  # 20
print(abs(combo_dict[2]))  # 15
print(min(combo_list[0], combo_list[2]))  # 10
print(max(combo_dict[1], 10))  # 10
print(pow(2, combo_list[0]))  # 1024

# NESTED: List of lists, dict with multiple operations
nested: list[list[int]] = [[1, 2], [3, 4, 5]]
print(len(nested))  # 2
print(len(nested[0]))  # 2
print(len(nested[1]))  # 3

# EDGE CASES: Empty structures, single elements
edge_list: list[int] = []
edge_dict: dict[int, int] = {}
edge_set: set[int] = set()
edge_bytes: bytes = b""
print(len(edge_list))  # 0
print(len(edge_dict))  # 0
print(len(edge_set))  # 0
print(len(edge_bytes))  # 0

single_list: list[int] = [42]
single_dict: dict[int, int] = {1: 42}
single_set: set[int] = {42}
single_bytes: bytes = b"x"
print(len(single_list))  # 1
print(len(single_dict))  # 1
print(len(single_set))  # 1
print(len(single_bytes))  # 1

# CHAINED OPERATIONS
chain1: list[int] = []
chain1.append(1)
chain1.append(2)
chain1.append(3)
print(len(chain1))  # 3
chain2: int = chain1.pop(1)
print(chain2)  # 2
print(len(chain1))  # 2

chain_set: set[int] = {1, 2, 3, 4, 5}
chain_set.remove(2)
chain_set.discard(4)
print(len(chain_set))  # 3

chain_bytes: bytes = b"  test  "
chain_b1: bytes = chain_bytes.strip()
chain_b2: bytes = chain_b1.upper()
print(len(chain_b2))  # 4

print(999)  # Success marker
