# Complex bitwise operations

def extract_bits(value: int, start: int, count: int) -> int:
    # Extract 'count' bits starting at position 'start'
    mask: int = (1 << count) - 1
    return (value >> start) & mask

def set_bit(value: int, pos: int) -> int:
    return value | (1 << pos)

def clear_bit(value: int, pos: int) -> int:
    return value & ~(1 << pos)

def toggle_bit(value: int, pos: int) -> int:
    return value ^ (1 << pos)

def check_bit(value: int, pos: int) -> bool:
    return ((value >> pos) & 1) == 1

# Test extract_bits
val: int = 0b11010110  # 214
extracted: int = extract_bits(val, 2, 4)  # Should get 0101 = 5
print(b"Extract 4 bits from pos 2 of 214:", extracted)

# Test set_bit
num: int = 0b00000000
num = set_bit(num, 0)
num = set_bit(num, 2)
num = set_bit(num, 4)
print(b"Set bits 0,2,4:", num)  # Should be 21

# Test clear_bit
num2: int = 0b11111111  # 255
num2 = clear_bit(num2, 0)
num2 = clear_bit(num2, 7)
print(b"Clear bits 0,7 of 255:", num2)  # Should be 126

# Test toggle_bit
num3: int = 0b10101010  # 170
num3 = toggle_bit(num3, 0)  # Set bit 0
num3 = toggle_bit(num3, 1)  # Clear bit 1
print(b"Toggle bits 0,1 of 170:", num3)  # Should be 169

# Test check_bit
test_val: int = 0b10101010
bit0: bool = check_bit(test_val, 0)
bit1: bool = check_bit(test_val, 1)
bit7: bool = check_bit(test_val, 7)
print(b"Bit 0 of 170:", bit0)
print(b"Bit 1 of 170:", bit1)
print(b"Bit 7 of 170:", bit7)

# Bit counting using shifts
def count_ones(n: int) -> int:
    count: int = 0
    while n > 0:
        count = count + (n & 1)
        n = n >> 1
    return count

ones: int = count_ones(0b11011011)  # 219
print(b"Ones in 219:", ones)  # Should be 6

# Swap without temp using XOR
a: int = 15
b: int = 27
print(b"Before swap: a=", a)
print(b"Before swap: b=", b)
a = a ^ b
b = a ^ b
a = a ^ b
print(b"After swap: a=", a)
print(b"After swap: b=", b)
