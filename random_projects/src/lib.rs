#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_factorial(){
        assert_eq!(q1::query::factorial(5), 120);
        assert_eq!(q1::query::factorial(7), 5040);
    }
}

pub mod q1;
pub mod q2;
pub mod q3;
pub mod q4;
pub mod q5;
pub mod q7;
pub mod q8;
pub mod q9;

pub mod commons {
    use std::io;
    use std::io::Write;

    pub fn get_user_input_uint() -> u32{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to obtain the user input");
        let res =  buffer.trim().parse::<u32>();
        if res.is_err() {
            panic!("Error parsing the user input");
        }else{
            res.unwrap()
        }
    }
    pub fn get_user_input_str() -> String{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to obtain the user input");
        buffer
    }
    pub fn stdout_fflush() {
        io::stdout().flush().unwrap();
    }
}