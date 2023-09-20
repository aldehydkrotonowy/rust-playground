fn check_if_prime(n: u32) -> bool{
    if n <= 1 {return false}
    if n <= 3 {return true}

    if n % 2 == 0 {return false}
    if n % 3 == 0 {return false}

    let mut i = 5;
    while i*i <= n {
        if n%i == 0 {return false}
         println!("div {} by {}",n,i);
        i +=1;
    }
    return true;
}
fn main() {
    for i in 0..30 {
        let p0 = 6*i;
        let p1 = 6*i+1;
        let p2 = 6*i+2;
        let p3 = 6*i+3;
        let p4 = 6*i+4;
        let p5 = 6*i+5;

        println!("p0={}, p1={}, p2={}, p3={}, p4={}, p5={}",p0,p1,p2,p3,p4,p5);
    }
    println!("-------------------------------------------------");
    for k in 1..40 {
        let is_prime = check_if_prime(k);
        println!("{} is prime = {}", k, is_prime)
    }
}
