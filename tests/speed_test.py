from time import perf_counter

start = perf_counter()

s = 0
for i in range(100_000_000):
    s += i

print(s)
print((perf_counter() - start) * 1000, "ms")