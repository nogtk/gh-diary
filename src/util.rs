pub fn from_env(name: &str) -> String {
    match std::env::var(name) {
        Ok(v) => v,
        Err(err) => {
            println!("{}: {}", err, name);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::from_env;

    #[test]
    fn success_from_env() {
        std::env::set_var("FOR_FROM_ENV_TEST", "VALUE");

        let result = from_env("FOR_FROM_ENV_TEST");
        assert_eq!(result, "VALUE");

        std::env::remove_var("FOR_FROM_ENV_TEST")
    }
}
