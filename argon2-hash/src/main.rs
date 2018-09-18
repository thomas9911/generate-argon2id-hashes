
extern crate argon2;
extern crate rand;

use argon2::{Config, ThreadMode, Variant, Version};
use rand::{OsRng, RngCore};

#[macro_use]
extern crate clap;
use clap::App;


fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml)
            .name(crate_name!()); 

    let matches = app.get_matches();

    let pwd = matches.value_of("PASSWD").expect("This is required!!");
    let p =  value_t!(matches, "parallelism", u32).unwrap_or(1);
    let t =  value_t!(matches, "iterations", u32).unwrap_or(10);
    let m =  value_t!(matches, "memory_size", u32).unwrap_or(65536);
    let hash_len =  value_t!(matches, "hash_length", u32).unwrap_or(32);

    println!("{}", make_pwdhash(pwd.to_string(), p, t, m, hash_len));
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


fn make_pwdhash(pwd: String, p: u32, t: u32, m: u32, hash_len: u32) -> String{
    let password = pwd.as_bytes();
    let salt = make_salt(8);
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: m,
        time_cost: t,
        lanes: p,
        thread_mode: ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: hash_len
    };
    let hash = argon2::hash_encoded(&password, &salt, &config).unwrap();
    return hash;
}
