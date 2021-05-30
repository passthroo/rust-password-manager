use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

pub static DEFAULT_LENGTH: usize = 20;

static CHARS_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
static CHARS_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static CHARS_NUMERIC: &str = "0123456789";
static CHARS_SPECIAL: &str = "~!@#$%^&*()_+`-=[];',./{}|:<>?";

pub fn generate(length: Option<&str>, lower: bool, upper: bool, numeric: bool, special: bool) -> String {
    let len;

    // is length specified?
    match length {
        Some(x) => len = x.parse::<usize>().unwrap(),
        None    => len = DEFAULT_LENGTH,
    }

    let mut chars = String::from("");
    let mut result = String::with_capacity(len);
    let mut rng = ChaCha20Rng::from_entropy();

    if lower {
        chars.push_str(CHARS_LOWER);
    }
    if upper {
        chars.push_str(CHARS_UPPER);
    }
    if numeric {
        chars.push_str(CHARS_NUMERIC);
    }
    if special {
        chars.push_str(CHARS_SPECIAL);
    }

    for _ in 0..len {
        let random_idx = rng.gen_range(0..chars.len());
        let pick = chars.chars().nth(random_idx).unwrap();
        result.push(pick);
    }
    
    return result;
}

mod tests {
    use super::*;

    #[test]
    fn generate_default() {
        let result = generate(None, true, true, true, true);
        assert_eq!(result.len(), DEFAULT_LENGTH);
    }

    #[test]
    fn generate_custom_length() {
        let length = 47;
        let result = generate(Some(&length.to_string()), true, true, true, true);
        assert_eq!(result.len(), length);
    }

    #[test]
    fn generate_lower() {
        let result = generate(None, true, false, false, false);
        assert!(result.chars().all(|x| String::from(CHARS_LOWER).contains(x)))
    }

    #[test]
    fn generate_upper() {
        let result = generate(None, false, true, false, false);
        assert!(result.chars().all(|x| String::from(CHARS_UPPER).contains(x)))
    }

    #[test]
    fn generate_numeric() {
        let result = generate(None, false, false, true, false);
        assert!(result.chars().all(|x| String::from(CHARS_NUMERIC).contains(x)))
    }

    #[test]
    fn generate_special() {
        let result = generate(None, false, false, false, true);
        assert!(result.chars().all(|x| String::from(CHARS_SPECIAL).contains(x)))
    }
}
