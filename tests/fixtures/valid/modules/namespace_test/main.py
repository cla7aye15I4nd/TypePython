import prime_utils
import sequence_utils

# Test prime utilities
prime_count: int = prime_utils.count_primes(20)
next_p: int = prime_utils.next_prime(10)

# Test sequence utilities
tri5: int = sequence_utils.triangular(5)
pent5: int = sequence_utils.pentagonal(5)
hex5: int = sequence_utils.hexagonal(5)
seq_sum: int = sequence_utils.sum_sequence(5)

print(b"Primes up to 20:", prime_count)
print(b"Next prime after 10:", next_p)
print(b"Triangular(5):", tri5)
print(b"Pentagonal(5):", pent5)
print(b"Hexagonal(5):", hex5)
print(b"Sequence sum:", seq_sum)
