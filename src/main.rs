fn main() {
    println!("Hello, world!");
}

// https://doc.rust-jp.rs/book-ja/ch11-01-writing-tests.html
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn it_failed() {
        assert_eq!(2 + 2, 1);
    }
}