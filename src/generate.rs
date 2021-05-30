use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

pub static DEFAULT_LENGTH: usize = 20;

static CHARS_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
static CHARS_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static CHARS_NUMERIC: &str = "0123456789";
static CHARS_SPECIAL: &str = "~!@#$%^&*()_+`-=[];',./{}|:<>?";

pub struct Generator {
    length: usize,
    lower: bool,
    upper: bool,
    numeric: bool,
    special: bool,
}

pub fn generate(length: Option<&str>, lower: bool, upper: bool, numeric: bool, special: bool) -> String {
    let length = match length {
        Some(x) => x.parse().unwrap(),
        None    => DEFAULT_LENGTH,
    };
    let default = length == DEFAULT_LENGTH && !(lower || upper || numeric || special);
    let lower = if default { true } else { lower };
    let upper = if default { true } else { upper };
    let numeric = if default { true } else { numeric };
    let special = if default { true } else { special };
    let generator = Generator { 
        length: length,
        upper: upper,
        lower: lower,
        numeric: numeric,
        special: special,
    };
    return generate_nonsense(generator);
}

fn generate_nonsense(generator: Generator) -> String {
    let mut chars = String::from("");
    let mut result = String::with_capacity(generator.length);
    let mut rng = ChaCha20Rng::from_entropy();

    if generator.lower {
        chars.push_str(CHARS_LOWER);
    }
    if generator.upper {
        chars.push_str(CHARS_UPPER);
    }
    if generator.numeric {
        chars.push_str(CHARS_NUMERIC);
    }
    if generator.special {
        chars.push_str(CHARS_SPECIAL);
    }

    for _ in 0..generator.length {
        let random_idx = rng.gen_range(0..chars.len());
        let pick = chars.chars().nth(random_idx).unwrap();
        result.push(pick);
    }
    
    return result;
}

fn generate_mnemonic(generator: Generator) -> String {
    
}

mod tests {
    use super::*;

    #[test]
    fn generate_default_length() {
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

    #[test]
    fn generate_mnemonic() {
        //TODO
    }
}
