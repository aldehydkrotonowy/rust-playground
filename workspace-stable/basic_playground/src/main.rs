pub mod closures;
pub mod find_and_show;
pub mod slices;
pub mod threads;

use std::time::Instant;

#[warn(unused_variables)]
fn main() {
    // find_and_show::find_and_show();
    // closures::run();
    // slices::slices_play();
    // let _a: String = String::from("akapulko");
    // threads::spawn_new_thread();
    let now = Instant::now();
    threads::thread_builder();
    let elapsed = now.elapsed();

    println!("elapsed {:.2?}", elapsed);
}
