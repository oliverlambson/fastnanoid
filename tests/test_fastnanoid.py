from fastnanoid import generate


def test_generate():
    assert len(generate()) == 21
    assert len(generate(size=10)) == 10
