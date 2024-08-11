# fastnanoid

fastnanoid is a tiny, secure URL-friendly, and fast unique string ID generator for Python, written in Rust.

It works as a drop in replacement for [py-nanoid](https://github.com/puyuan/py-nanoid)'s `generate()`:

```diff
- from nanoid import generate
+ from fastnanoid import generate
```

It's 2.7x faster than the original.

## When not to use it

If you need the same amount of entropy as uuid, you may as well use uuid and
base64url encode it:

```python
import uuid
from fastnanoid import urlid_to_uuid, uuid_to_urlid
# say you have a uuid, maybe from your database:
id_ = uuid.uuid4() # type: uuid.UUID
# you can encoded it in base64url so it displays as a short string:
urlid = uuid_to_urlid(id_) # type: str
# and when you read it back in from the user, you can convert it back to a normal UUID:
decoded_urlid = urlid_to_uuid(urlid) # type: UUID
```

This is simpler than using a nanoid which is not compliant with any existing standards.
If you already have a generated UUID (say from a database),
this is _much_ faster than generating a new nanoid.
(If you don't have a UUID, generating one plus encoding it in base64url is about 50% slower than fastnanoid.)

\* these are very simple helper functions, you can easily implement them
yourself and save a dependency.

## Contributing

```sh
# local env
python -m venv .venv
source .venv/bin/activate
pip install -r requirements-dev.txt
# build and use
maturin develop
python -c 'import fastnanoid; print(fastnanoid.generate())'
# test
cargo test
pytest
mypy
ruff check
ruff format --check
```

## Credits

Inspired by [py-nanoid](https://github.com/puyuan/py-nanoid)
