# Benchmarks

```sh
maturin develop --release
python benchmarks/benchmark.py
```

Results:

```
$ python benchmarks/benchmark.py
Generating 1,000,000 IDs
- nanoid: 2.20s (2.1954383330303244e-06 s/id)
- fastnanoid: 0.78s (7.816055839648471e-07 s/id) - 2.81x faster
- uuid (generate + base64encode): 1.29s (1.2920875830459408e-06 s/id) - 1.70x faster
- uuid (generate only): 0.96s (9.631779579794965e-07 s/id) - 2.28x faster
- uuid (base64encode only): 0.27s (2.659827920142561e-07 s/id) - 8.25x faster
```
