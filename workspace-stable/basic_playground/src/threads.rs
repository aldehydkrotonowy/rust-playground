use rand::Rng;
use std::{thread, thread::JoinHandle, time::Duration};

pub fn spawn_new_thread() {
    let handle: JoinHandle<i32> = thread::spawn(|| {
        let delay = rand::thread_rng().gen_range(10..=2000);
        thread::sleep(Duration::from_millis(delay));
        print!("hello from thread \n");
        5
    });

    match handle.join() {
        Err(e) => print!("Oh my god, some error: {:?}", e),
        Ok(i) => print!("this is value {:?} \n", i),
    }
}

pub fn thread_builder() {
    let handles: Vec<JoinHandle<String>> = (0..=10)
        .map(|i| {
            let delay = rand::thread_rng().gen_range(10..=2000);
            let name = format!("Thread num {}", i);

            let bulder = thread::Builder::new().name(name);

            bulder
                .spawn(move || {
                    let thread_name = thread::current().name().unwrap().to_owned();
                    println!("thread started = {}", thread_name);
                    thread::sleep(Duration::from_millis(delay));
                    thread_name.to_owned()
                })
                .unwrap()
        })
        .collect();

    for h in handles {
        let r = h.join().unwrap();
        println!("Thread done {:?}", r);
    }
}
