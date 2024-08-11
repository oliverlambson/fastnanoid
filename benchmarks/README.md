# Benchmarks

```sh
maturin develop --release
python benchmarks/benchmark.py
```

Results:

```
$ python benchmarks/benchmark.py
Generating 1,000,000 IDs
- nanoid: 2.22s (2.215884291974362e-06 s/id)
- fastnanoid: 0.86s (8.563055000267923e-07 s/id) - 2.59x faster
- uuid (generate + base64encode): 1.28s (1.2837962500052527e-06 s/id) - 1.73x faster
- uuid (generate only): 0.98s (9.750179999973624e-07 s/id) - 2.27x faster
- uuid (base64encode only): 0.26s (2.631903750007041e-07 s/id) - 8.42x faster
```
