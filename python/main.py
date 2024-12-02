import time


def find_primes(limit):
    primes = []
    for num in range(2, limit + 1):
        is_prime = all(num % i != 0 for i in range(2, int(num**0.5) + 1))
        if is_prime:
            primes.append(num)
    return primes


start_time = time.time()
primes = find_primes(10_000_000)
end_time = time.time()

print(f"Found {len(primes)} primes in {end_time - start_time:.2f} seconds.")
