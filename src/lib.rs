use pyo3::prelude::*;
use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};

const DEFAULT_ALPHABET: &'static str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_-";

/// Generates a nanoid string.
/// this is a drop in replacement for py-nanoid's nanoid.generate() function.
#[pyfunction]
#[pyo3(signature = (alphabet=DEFAULT_ALPHABET, size=21))]
fn generate(alphabet: Option<&str>, size: Option<usize>) -> PyResult<String> {
    let alphabet = alphabet.unwrap_or(DEFAULT_ALPHABET).to_string();
    let size = size.unwrap_or(21);

    let alphabet_vec: Vec<char> = alphabet.chars().collect();
    let alphabet_len = alphabet.chars().count();

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
    use rstest::*;

    const ALPHABET_EXAMPLE: &'static str = "asdf🌍";

    #[rstest]
    #[case(None)]
    #[case(Some(ALPHABET_EXAMPLE))]
    fn id_generated_with_both_default_values_is_21_chars_long(#[case] alphabet: Option<&str>) {
        assert_eq!(generate(alphabet, None).unwrap().chars().count(), 21);
    }

    #[rstest]
    fn chosen_alphabet_is_respected() {
        let id = generate(Some(ALPHABET_EXAMPLE), None).unwrap();
        for char in id.chars() {
            if !ALPHABET_EXAMPLE.contains(char) {
                panic!("\"{}\" char was not expected because it is not present in the given alphabet", char);
            }
        }
    }

    #[rstest]
    #[case(None)] // default
    #[case(Some(11))]
    #[case(Some(12))]
    #[case(Some(13))]
    fn choosing_an_alphabet_does_not_affect_size(#[case] size: Option<usize>) {
        assert_eq!(generate(Some(ALPHABET_EXAMPLE), size).unwrap().chars().count(), size.unwrap_or(21));
    }

    #[rstest]
    #[case(11)]
    #[case(12)]
    #[case(13)]
    fn generated_id_length_can_be_chosen(#[case] size: usize) {
        assert_eq!(generate(None, Some(size)).unwrap().chars().count(), size);
    }


    #[test]
    fn it_works_in_python() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|_| {
            assert_eq!(generate(None, None).unwrap().chars().count(), 21);
            assert_eq!(generate(None, Some(11)).unwrap().chars().count(), 11);
            assert_eq!(
                generate(Some(ALPHABET_EXAMPLE), None)
                    .unwrap()
                    .chars()
                    .count(),
                21
            );
            assert_eq!(
                generate(Some(ALPHABET_EXAMPLE), Some(11))
                    .unwrap()
                    .chars()
                    .count(),
                11
            );
        });
    }
}