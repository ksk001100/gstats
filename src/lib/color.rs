use std::fmt::Display;

pub fn red<T: Display>(t: T) -> String {
    format!("\x1b[31m{}\x1b[0m", t)
}

pub fn green<T: Display>(t: T) -> String {
    format!("\x1b[32m{}\x1b[0m", t)
}

pub fn yellow<T: Display>(t: T) -> String {
    format!("\x1b[33m{}\x1b[0m", t)
}

pub fn blue<T: Display>(t: T) -> String {
    format!("\x1b[34m{}\x1b[0m", t)
}