fn main() {
    println!("Hello, world!");
}

fn target_method(arg: i32) -> String {
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
}