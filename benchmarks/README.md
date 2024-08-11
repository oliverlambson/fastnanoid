# Benchmarks

```sh
maturin develop --release
python benchmarks/benchmark.py
```

Results:

```
$ python benchmarks/benchmark.py
Generating 1,000,000 IDs
- nanoid: 2.19s (2.1881258339853956e-06 s/id)
- fastnanoid: 0.86s (8.555265420000069e-07 s/id) - 2.56x faster
- uuid (generate + base64encode): 1.28s (1.2831176670151764e-06 s/id) - 1.71x faster
- uuid (generate only): 0.97s (9.65138916973956e-07 s/id) - 2.27x faster
- uuid (base64encode only): 0.26s (2.62122415995691e-07 s/id) - 8.35x faster
```
