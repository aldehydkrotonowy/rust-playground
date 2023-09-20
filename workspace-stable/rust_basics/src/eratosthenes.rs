
// use std::io;
use std::f64;
use std::u32;

fn check_if_prime(n: u32) -> bool{
    if n <= 1 {
        return false;
    }
    if n == 3 || n == 5 {
        return true
    }
    if n%2==0 || n%3==0 || n%5==0 {
        return false;
    }

    let mut k = 1;
    while u32::pow(6*k+1, 2) <= n {
        if n % 6*k+1 == 0 || n % 6*k+3 == 0  {
             return false;
        }
        k+=1;
    }
    
    // for i in 5..(i*i<=n).step_by(6*i+1) {
    //     if n % i == 0  {
    //          return false;
    //     }
    // }
    return true;
}

fn main_era(){
    // const N: u64 = 10;

    let numbers: [f64; 10] = [0.0; 10];

    println!("array numbers has {} elements", numbers.len());
    // for i in numbers.iter() {
    //     println!("{}", i);
    // }

    // let mut input = String::new();
    // println!("Type something!");
    // io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    // println!("{}", input);

    for x in 0..99 {
        let result = check_if_prime(x);
        // if result == true {
        // }
        println!("the number {} is prime = {}",x, result)
   }
}
