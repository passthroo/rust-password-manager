#[macro_use]
extern crate clap;
use clap::App;

static LENGTH_DEFAULT: u64 = 20;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("generate") {
        let alpha: bool = matches.is_present("alpha");
        let length: u64 = match matches.value_of("length") {
            Some(x) => x.parse().unwrap(),
            None    => LENGTH_DEFAULT,
        };
        let special: bool = matches.is_present("special");

        println!("You want a password of length {:?}", length);
        println!("You {}want alphanumeric characters", match alpha {
            true    => "do not ",
            false   => "",
        });
        println!("You {}want special characters", match special {
            true    => "",
            false   => "do not ",
        });
    }
}
