//https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
pub fn run() {
    let s = String::from("hello");
    take_ownership(s);

    let x = 5;
    make_copy(x);

    let _v1 = give_ownership();
    let v2 = String::from("elephant");
    let _s3 = take_and_gives_back(v2);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_value: i32) {
    println!("{}", some_value);
}

fn give_ownership() -> String {
    String::from("foo")
}

fn take_and_gives_back(a: String) -> String {
    a
}
