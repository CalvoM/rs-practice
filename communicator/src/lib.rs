pub mod network;
pub mod client;

#[cfg(test)]
mod tests {
    use super::client;
    #[test]
    fn it_works() {
        // let result = 2 + 2;
        // assert_eq!(result, 4);
        client::connect();
    }
}
