# Combinatorics algorithms
def combination(n: int, k: int) -> int:
    if k > n:
        return 0
    if k == 0 or k == n:
        return 1

    # C(n,k) = n! / (k! * (n-k)!)
    # Optimize by using C(n,k) = C(n,n-k) if k > n-k
    if k > n - k:
        k = n - k

    result: int = 1
    i: int = 0

    while i < k:
        result = result * (n - i)
        result = result // (i + 1)
        i = i + 1

    return result

def catalan(n: int) -> int:
    # Catalan number using C(2n, n) / (n+1)
    c: int = combination(2 * n, n)
    return c // (n + 1)

def pascal_triangle_sum(rows: int) -> int:
    sum: int = 0
    n: int = 0

    while n < rows:
        k: int = 0
        while k <= n:
            sum = sum + combination(n, k)
            k = k + 1
        n = n + 1

    return sum

result1: int = combination(10, 5)
result2: int = catalan(4)
result3: int = pascal_triangle_sum(5)

print(b"C(10,5):", result1)
print(b"Catalan(4):", result2)
print(b"Pascal triangle sum (5 rows):", result3)
