
extern crate argon2;
extern crate rand;

use std::env;
use argon2::{Config, ThreadMode, Variant, Version};
use rand::{OsRng, RngCore};


fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
                match args[1].parse() {
                    Ok(password) => println!("{}", make_pwdhash(password)),
                    _ => println!("This is not the answer to life."),
                }
            },
        _ => println!("{}", print_help()),
    }
}


fn print_help() -> String {
    "
    Generates Argon2id hashes with random salt and using some defaults.
    The first argument is the password which you use to generate the hash
    ".to_string()
}


fn make_salt(length: usize) -> Vec<u8>{
    let mut salty: Vec<u8> = vec![0; length];
    let mut rng = match OsRng::new() {
        Ok(g) => g,
        Err(e) => panic!("Failed to obtain OS RNG: {}", e)
    };
    rng.fill_bytes(&mut salty);
    return salty;
}


fn make_pwdhash(pwd: String) -> String{
    let password = pwd.as_bytes();
    let salt = make_salt(8);
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 65536,
        time_cost: 10,
        lanes: 1,
        thread_mode: ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 32
    };
    let hash = argon2::hash_encoded(&password, &salt, &config).unwrap();
    return hash;
}
