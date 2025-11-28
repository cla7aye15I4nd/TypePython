# Loops with complex conditional logic inside
def count_multiples(limit: int) -> int:
    count_3: int = 0
    count_5: int = 0
    count_both: int = 0
    i: int = 1

    while i <= limit:
        if i % 3 == 0:
            if i % 5 == 0:
                count_both = count_both + 1
            else:
                count_3 = count_3 + 1
        else:
            if i % 5 == 0:
                count_5 = count_5 + 1
        i = i + 1

    return count_3 * 100 + count_5 * 10 + count_both

def sum_conditional(n: int) -> int:
    sum_even: int = 0
    sum_odd: int = 0
    i: int = 1

    while i <= n:
        if i % 2 == 0:
            sum_even = sum_even + i
        else:
            sum_odd = sum_odd + i
        i = i + 1

    return sum_even + sum_odd * 2

def fizzbuzz_sum(n: int) -> int:
    sum: int = 0
    i: int = 1

    while i <= n:
        if i % 15 == 0:
            sum = sum + 15
        else:
            if i % 3 == 0:
                sum = sum + 3
            else:
                if i % 5 == 0:
                    sum = sum + 5
                else:
                    sum = sum + i
        i = i + 1

    return sum

result1: int = count_multiples(100)
result2: int = sum_conditional(20)
result3: int = fizzbuzz_sum(30)

print(b"Count multiples:", result1)
print(b"Sum conditional:", result2)
print(b"FizzBuzz sum:", result3)
