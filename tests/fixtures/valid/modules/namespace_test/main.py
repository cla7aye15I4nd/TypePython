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

print("Primes up to 20:", prime_count)
print("Next prime after 10:", next_p)
print("Triangular(5):", tri5)
print("Pentagonal(5):", pent5)
print("Hexagonal(5):", hex5)
print("Sequence sum:", seq_sum)
