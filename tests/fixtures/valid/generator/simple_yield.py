# Test simple generator with yield
# Using a for loop with range instead of while to make state management easier

def count_up(n: int) -> int:
    i: int = 0
    while i < n:
        yield i
        i = i + 1

# Iterate over the generator
for x in count_up(5):
    print(b"Got:", x)

print(b"Done!")
