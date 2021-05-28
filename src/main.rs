#[macro_use]
extern crate clap;
use clap::App;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

static LENGTH_DEFAULT: u64 = 20;

fn generate(length: u64, special: bool) {
    let mut rng = ChaCha20Rng::from_entropy();
    let mut chars = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");

    if special {
        // everything except " \
        chars.push_str("~!@#$%^&*()_+`-=[];',./{}|:<>?");
    }
    
    let mut result = String::from("");
    for i in 0..length {
        let idx = rng.gen_range(0..chars.len());
        let pick = chars.chars().nth(idx).unwrap();
        result.push(pick);
    }

    println!("{}", result);
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("generate") {
        let length: u64 = match matches.value_of("length") {
            Some(x) => x.parse().unwrap(),
            None    => LENGTH_DEFAULT,
        };
        let special: bool = matches.is_present("special");

        println!("You want a password of length {:?}", length);
        println!("You {}want special characters", match special {
            true    => "",
            false   => "do not ",
        });

        generate(length, special);
    }
}
