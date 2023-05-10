use anyhow;
use anyhow::{Context, Result};
use std::{error, fmt};

#[derive(Debug)]
enum MyError {
    ErrorOne,
    ErrorTwo,
    ErrorThree,
    ErrorFour,
}
impl error::Error for MyError {}
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::ErrorOne => {
                write!(f, "Error One Found")
            }
            MyError::ErrorTwo => {
                write!(f, "Error Two Found")
            }
            MyError::ErrorThree => {
                write!(f, "Error Three Found")
            }
            MyError::ErrorFour => {
                write!(f, "Error Three Found")
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
    err_works(2).unwrap();
}

fn err_works(rand_num: u64) -> Result<()> {
    let err = match rand_num % 2 {
        0 => Err(anyhow::anyhow!(MyError::ErrorOne)),
        1 => Ok(()),
        _ => Err(anyhow::anyhow!(MyError::ErrorFour)),
    };
    err
}
