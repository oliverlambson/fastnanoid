# rs-nanoid

rs-nanoid is a tiny, secure URL-friendly unique string ID generator for Python, written in Rust.

## Contributing

```sh
# local env
python -m venv .venv
source .venv/bin/activate
pip install maturin
# build and use
maturin develop
python -c 'import rs_nanoid; print(rs_nanoid.generate())'
# test
cargo test
```

## Credits

Inspired by [py-nanoid](https://github.com/puyuan/py-nanoid)
