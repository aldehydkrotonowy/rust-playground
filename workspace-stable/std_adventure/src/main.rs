mod list_folder;
mod ownership;
mod reference_and_borrowing;

// use rand::Rng;

fn main() {
    list_folder::run();
    ownership::run();
    reference_and_borrowing::run();

    // let v: Vec<i32> = Vec::new();

    // let mut rng = rand::thread_rng();

    // hello
    // this is not me
    // and another line
}
