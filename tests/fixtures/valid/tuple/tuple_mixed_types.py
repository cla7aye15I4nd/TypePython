# Tuple with mixed types - str, float, bool indexing
t1: tuple[str, str] = ("hello", "world")
print(t1[0])
print(t1[1])

t2: tuple[float, float, float] = (1.5, 2.5, 3.5)
print(t2[0])
print(t2[1])
print(t2[2])

t3: tuple[bool, bool] = (True, False)
print(t3[0])
print(t3[1])
