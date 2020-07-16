use std::env;

pub fn fmt_command(cmd: &str) -> String {
    env::var("PREFIX").unwrap_or("!".to_string()) + cmd
}
