# fastnanoid

fastnanoid is a tiny, secure URL-friendly, and fast unique string ID generator for Python, written in Rust.

It works as a drop in replacement for [py-nanoid](https://github.com/puyuan/py-nanoid)'s `generate()`:

```diff
- from nanoid import generate
+ from fastnanoid import generate
```

It's 2.7x faster than the original.

## Contributing

```sh
# local env
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
# build and use
maturin develop
python -c 'import fastnanoid; print(fastnanoid.generate())'
# test
cargo test
```

## Credits

Inspired by [py-nanoid](https://github.com/puyuan/py-nanoid)
