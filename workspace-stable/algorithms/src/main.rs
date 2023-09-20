mod mergesort;

use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();

    let mut vec: Vec<i32> = Vec::new();

    for _ in 0..4 {
        let num: i32 = rng.gen_range(0..10000);
        vec.push(num);
    }
    let mut a = String("adeko");
    

    println!("before {:?}", vec);
    mergesort::sort(&mut vec);
    println!("after {:?}", lala);
}
