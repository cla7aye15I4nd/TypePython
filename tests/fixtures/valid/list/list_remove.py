# Test list remove() method

# Basic remove
lst1: list[int] = [1, 2, 3, 4, 5]
print(len(lst1))
lst1.remove(3)
print(len(lst1))
print(lst1[0])
print(lst1[1])
print(lst1[2])
print(lst1[3])

# Remove first element
lst2: list[int] = [10, 20, 30, 40]
lst2.remove(10)
print(len(lst2))
print(lst2[0])

# Remove last element
lst3: list[int] = [100, 200, 300]
lst3.remove(300)
print(len(lst3))
print(lst3[0])
print(lst3[1])

# Remove from middle
lst4: list[int] = [1, 2, 3, 4, 5, 6]
lst4.remove(3)
print(len(lst4))
print(lst4[2])

# Remove when there are duplicates (removes first occurrence)
lst5: list[int] = [1, 2, 3, 2, 4, 2]
print(len(lst5))
lst5.remove(2)
print(len(lst5))
print(lst5[0])
print(lst5[1])
print(lst5[2])
print(lst5[3])
print(lst5[4])

# Remove multiple times
lst6: list[int] = [5, 10, 15, 20, 25]
lst6.remove(5)
lst6.remove(15)
lst6.remove(25)
print(len(lst6))
print(lst6[0])
print(lst6[1])

# Remove from single element list
lst7: list[int] = [99]
print(len(lst7))
lst7.remove(99)
print(len(lst7))
