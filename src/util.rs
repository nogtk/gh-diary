pub fn from_env(name: &str) -> String {
    match std::env::var(name) {
        Ok(v) => v,
        Err(err) => {
            println!("{}: {}", err, name);
            std::process::exit(1);
        }
    }
}
