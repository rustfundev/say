pub fn say(word: &str) {
    println!("{}", word);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = say("hello");
        assert_eq!(result, ());
    }
}
