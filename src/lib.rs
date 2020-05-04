//! # Kibi
//!
//! Kibi is a text editor in ≤1024 lines of code.

pub use crate::{config::Config, editor::Editor, error::Error};

pub mod ansi_escape;
mod config;
mod editor;
mod error;
mod row;
mod syntax;
mod terminal;

#[cfg(windows)] mod windows;
#[cfg(windows)] use windows as sys;

#[cfg(unix)] mod unix;
#[cfg(unix)] use unix as sys;

#[macro_export]
macro_rules! write_str {
    ($($arg:expr),*) => {
        {
            let mut s = String::new();
            $(crate::Append::append(&mut s, $arg);)*
            s
        }
    }
}

trait Append<T> {
    fn append(&mut self, value: T);
}

impl Append<&str> for String {
    fn append(&mut self, value: &str) {
        self.push_str(value);
    }
}

impl Append<&String> for String {
    fn append(&mut self, value: &String) {
        self.push_str(value);
    }
}

impl Append<String> for String {
    fn append(&mut self, value: String) {
        self.push_str(&value);
    }
}

impl Append<char> for String {
    fn append(&mut self, value: char) {
        self.push(value);
    }
}

impl Append<usize> for String {
    fn append(&mut self, mut n: usize) {
        let mut buf = [0u8; 10];
        let mut i = 1;
        loop {
            buf[i-1] = (n % 10) as u8 + b'0';
            n = n / 10;
            if n == 0 {
                break;
            }
            i += 1;
        }
        for a in 0..i {
            self.push(buf[i - a - 1] as char);
        }
    }
}

macro_rules! impl_append_error {
    ($t:ty) => (
        #[allow(deprecated)]
        impl Append<$t> for String {
            fn append(&mut self, value: $t) {
                use std::error::Error as _;
                self.push_str(value.description());
            }
        }
    )
}

impl_append_error!(std::char::ParseCharError);
impl_append_error!(std::convert::Infallible);
impl_append_error!(std::io::Error);
impl_append_error!(std::num::ParseIntError);
impl_append_error!(std::num::ParseFloatError);
impl_append_error!(std::str::ParseBoolError);

enum Aligment {
    Left,
    Center,
    Right,
}
pub(crate) struct Pad<T>(Aligment, usize, T);

impl<T> Append<Pad<T>> for String
where String: Append<T> + for<'a> Append<&'a str>,
{
    fn append(&mut self, value: Pad<T>) {
        let mut s = write_str!(value.2);
        if s.len() < value.1 {
            let pad = value.1 - s.len();
            s = match value.0 {
                Aligment::Right => write_str!(" ".repeat(pad), s),
                Aligment::Left => write_str!(s, " ".repeat(pad)),
                Aligment::Center => {
                    let left = pad / 2;
                    let right = pad - left;
                    write_str!(" ".repeat(left), s, " ".repeat(right))
                }
            };
        }
        self.append(s);
    }
}
