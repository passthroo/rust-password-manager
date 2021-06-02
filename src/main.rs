#[macro_use]
extern crate clap;
use clap::App;

mod generate;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if let Some(matches) = matches.subcommand_matches("generate") {
        let length = matches.value_of("length");
        let lower = matches.is_present("lower");
        let upper = matches.is_present("upper");
        let numeric = matches.is_present("numeric");
        let special = matches.is_present("special");
        let mnemonic = matches.is_present("mnemonic");
        let result = generate::generate(length, lower, upper, numeric, special, mnemonic);
        println!("{}", result);
    }
}
