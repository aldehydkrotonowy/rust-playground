use std::collections::HashMap;


#[derive(Debug)]
enum Example {
  Float(f64),
  Int(i32),
  Text(String)
}

pub fn run(){
  let x = vec![1,2,3,4];
  let mut v = Vec::new();

  v.push(5);
  v.push(6);
  v.push(7);
  v.push(8);

  for i in &v {
    println!("{} ", i);
  }

  println!("{:?} {} {} {:?}", &v, v.len(), v.capacity(), x);

  println!("{:?}", v.pop());

  let r = vec![
    Example::Int(1234),
    Example::Float(12.43),
    Example::Text(String::from("test string"))
  ];

  println!("{:?}", &r);


  let mut hm = HashMap::new();

  hm.insert(String::from("random text"), 12);
  hm.insert(String::from("dureń z niego"), 49);

  for (k,v) in &hm {
    println!("{}: {}", k, v);
  }

  match hm.get(&String::from("random text")) {
    Some(&n) => println!("{}", n),
    _ => println!("no match")
  }
}