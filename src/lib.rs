use pyo3::prelude::*;
use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};

/// Generates a nanoid string.
/// this is a drop in replacement for py-nanoid's nanoid.generate() function.
#[pyfunction]
#[pyo3(signature = (alphabet="0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_-", size=21))]
fn generate(alphabet: Option<&str>, size: Option<usize>) -> PyResult<String> {
    let alphabet = match alphabet {
        Some(alphabet) => alphabet.to_string(),
        None => "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_-".to_string(),
    };
    let size = size.unwrap_or(21);
    let mut alphabet_vec = Vec::with_capacity(size); // Not sure what size this should be but this
                                                     // should guarantee no reallocs

    let mut alphabet_len = 0;
    for char in alphabet.chars() {
        alphabet_vec.push(char);
        alphabet_len += 1;
    }
    let mut nanoid = String::with_capacity(size);
    let uniform = Uniform::new(0, alphabet_len);
    let mut rng = StdRng::from_entropy();
    for _ in 0..size {
        let i = rng.sample(uniform);
        nanoid.push(alphabet_vec[i]);
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
        Python::with_gil(|_| {
            assert_eq!(generate(None, None).unwrap().chars().count(), 21);
            assert_eq!(generate(None, Some(11)).unwrap().chars().count(), 11);
            assert_eq!(
                generate(Some("asdf🌍".to_string()), None)
                    .unwrap()
                    .chars()
                    .count(),
                21
            );
            assert_eq!(
                generate(Some("asdf🌍".to_string()), Some(11))
                    .unwrap()
                    .chars()
                    .count(),
                11
            );
        });
    }
}
