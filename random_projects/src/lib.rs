#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_factorial(){
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(7), 5040);
    }
}

pub fn factorial(num: u32) -> u32 {
    if num < 1 {
        1
    } else {
        num * factorial(num -1)
    }
}
pub mod commons {
    use std::io;

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
}