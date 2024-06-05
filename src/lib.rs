use pyo3::{prelude::*, types::PyString};
use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};

/// Generates a nanoid string.
/// this is a drop in replacement for py-nanoid's nanoid.generate() function.
#[pyfunction]
fn generate(alphabet: Option<Bound<PyString>>, size: Option<usize>) -> PyResult<String> {
    let alphabet = match alphabet {
        Some(alphabet) => alphabet.to_string(),
        None => "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_-".to_string(),
    };
    let size = size.unwrap_or(21);
    let mut rng = StdRng::from_entropy();
    let alphabet_len = alphabet.chars().count(); // rather than using .len() - in UTF8 one char might be more than one byte
    let mut nanoid = String::with_capacity(size);
    for _ in 0..size {
        let i = rng.sample(Uniform::new(0, alphabet_len));
        nanoid.push(alphabet.chars().nth(i).unwrap());
    }
    Ok(nanoid)
}

#[pymodule]
#[pyo3(name = "fastnanoid")]
fn fastnanoid(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            assert_eq!(generate(None, None).unwrap().chars().count(), 21);
            assert_eq!(generate(None, Some(11)).unwrap().chars().count(), 11);
            assert_eq!(
                generate(Some(PyString::new_bound(py, "asdf🌍")), None)
                    .unwrap()
                    .chars()
                    .count(),
                21
            );
            assert_eq!(
                generate(Some(PyString::new_bound(py, "asdf🌍")), Some(11))
                    .unwrap()
                    .chars()
                    .count(),
                11
            );
        });
    }
}
