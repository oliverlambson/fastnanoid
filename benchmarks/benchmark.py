from timeit import timeit

import fastnanoid
import nanoid


if __name__ == "__main__":
    n = 1_000_000
    print(f"Generating {n:,} IDs")
    nanotime = timeit(nanoid.generate, number=n)
    fastnanotime = timeit(fastnanoid.generate, number=n)
    print(f"nanoid: {nanotime:.2f}s ({nanotime/n} s/id)")
    print(f"fastnanoid: {fastnanotime:.2f}s ({fastnanotime/n} s/id)")
    print(f"{nanotime/fastnanotime:.2f}x faster")
