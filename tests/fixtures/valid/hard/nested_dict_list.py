# Test Dict[str, List[int]] nested type support

# Create a dict with list values
data: dict[str, list[int]] = {"a": [1, 2, 3], "b": [4, 5, 6]}

# Access list from dict
a_list: list[int] = data["a"]
print(a_list[0])  # 1
print(a_list[1])  # 2
print(a_list[2])  # 3

# Access nested elements directly
print(data["b"][0])  # 4
print(data["b"][1])  # 5
print(data["b"][2])  # 6

# Modify nested list
data["a"].append(10)
print(data["a"][3])  # 10

# Add new list to dict
data["c"] = [7, 8, 9]
print(data["c"][0])  # 7
print(data["c"][2])  # 9
