# String methods
s: str = "hello world"
print(s.upper())
print(s.lower())
print(s.capitalize())
print(s.title())
print(s.strip())
print(s.lstrip())
print(s.rstrip())
print(s.split(" "))
print(s.replace("world", "python"))
print(s.startswith("hello"))
print(s.endswith("world"))
print(s.find("o"))
print(s.count("l"))
print(s.isalpha())
print(s.isdigit())
print(s.isspace())
print(len(s))

# Test join method
lst: list[str] = ["a", "b", "c"]
sep: str = ","
result: str = sep.join(lst)
print(result)
print("-".join(lst))
print("".join(lst))
