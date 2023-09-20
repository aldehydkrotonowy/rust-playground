//https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

pub fn run() {
    let k = String::from("hello");
    let mut u = String::from("Elephant");

    //pass immutable reference (structure which ptr points to ptr of owner?)
    let len = calculate_len(&k);
    let _new_string = change(&mut u);

    println!("The length of the input is {}", len);
}

fn calculate_len(u: &str) -> usize {
    //reference to string
    u.len() //u goes out of scope but ownership of memory still belongs to k
}

fn change(incoming: &mut String) {
    incoming.push_str(" is big");
}
