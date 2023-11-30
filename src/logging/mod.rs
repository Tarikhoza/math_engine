pub fn env_info(env_var: &'static str, log: String) {
    if std::env::var(env_var).is_ok() || std::env::var("all").is_ok() {
        println!("env_info - {} : {}", env_var, log);
    }
}

pub fn env_warn(env_var: &'static str, log: String) {
    if std::env::var(env_var).is_ok() || std::env::var("all").is_ok() {
        println!("env_warn - {} : {}", env_var, log);
    }
}

pub fn env_error(env_var: &'static str, log: String) {
    if std::env::var(env_var).is_ok() || std::env::var("all").is_ok() {
        println!("env_error - {} : {}", env_var, log);
    }
}
