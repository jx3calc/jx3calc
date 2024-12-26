pub fn say_hello(name: String) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = say_hello("world".to_string());
        assert_eq!(result, "Hello, world!");
    }
}
