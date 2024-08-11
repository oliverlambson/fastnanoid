from timeit import timeit

import fastnanoid
import nanoid
import uuid


if __name__ == "__main__":
    n = 1_000_000
    print(f"Generating {n:,} IDs")

    print("- nanoid: ", end="", flush=True)
    nanotime = timeit(nanoid.generate, number=n)
    print(f"{nanotime:.2f}s ({nanotime/n} s/id)")

    print("- fastnanoid: ", end="", flush=True)
    fastnanotime = timeit(fastnanoid.generate, number=n)
    print(
        f"{fastnanotime:.2f}s ({fastnanotime/n} s/id) - {nanotime/fastnanotime:.2f}x faster"
    )

    print("- uuid (generate + base64encode): ", end="", flush=True)
    uuidb64time = timeit(lambda: fastnanoid.uuid_to_urlid(uuid.uuid4()), number=n)
    print(
        f"{uuidb64time:.2f}s ({uuidb64time/n} s/id) - {nanotime/uuidb64time:.2f}x faster"
    )

    print("- uuid (generate only): ", end="", flush=True)
    uuidtime = timeit(uuid.uuid4, number=n)
    print(f"{uuidtime:.2f}s ({uuidtime/n} s/id) - {nanotime/uuidtime:.2f}x faster")

    print("- uuid (base64encode only): ", end="", flush=True)
    _u = uuid.uuid4()
    uuidtime = timeit(lambda: fastnanoid.uuid_to_urlid(_u), number=n)
    print(f"{uuidtime:.2f}s ({uuidtime/n} s/id) - {nanotime/uuidtime:.2f}x faster")
