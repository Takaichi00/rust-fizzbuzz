fn main() {
    println!("Hello, world!");
}

fn target_method(arg: i32) -> String {

    if arg % 15 == 0 {
        return String::from("FizzBuzz");
    }

    if arg % 3 == 0 {
        return String::from("Fizz");
    }

    if arg % 5 == 0 {
        return String::from("Buzz");
    }

    return arg.to_string();
}

// https://doc.rust-jp.rs/book-ja/ch11-01-writing-tests.html
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn _1のときは1を取得する() {
        assert_eq!(target_method(1), "1");
    }

    #[test]
    fn _2のときは2を取得する() {
        assert_eq!(target_method(2), "2");
    }

    #[test]
    fn _3のときは_fizzを取得する() {
        assert_eq!(target_method(3), "Fizz");
    }

    #[test]
    fn _5のときは_buzzを取得する() {
        assert_eq!(target_method(5), "Buzz");
    }

    #[test]
    fn _6のときは_fizzを取得する() {
        assert_eq!(target_method(6), "Fizz");
    }

    #[test]
    fn _10のときは_buzzを取得する() {
        assert_eq!(target_method(10), "Buzz");
    }

    #[test]
    fn _15のときは_fizzbuzzを取得する() {
        assert_eq!(target_method(15), "FizzBuzz");
    }
}