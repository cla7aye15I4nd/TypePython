# Test tab indentation (tabs are treated as 4 spaces)

def func_with_tabs() -> int:
	x: int = 1
	if x > 0:
		x = x + 1
	return x

result: int = func_with_tabs()
print(result)
