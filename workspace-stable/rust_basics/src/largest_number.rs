fn largest(list &[i32]) -> i32 {
  let mut largest = list[0];

  for &item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn main(){
  let lis1 = vec![23,24,26,33, 52, 66, 28];
  let result = largest(&lis1);
  println!("Largest number is {:?}", result);
  
  let list2 = vec![776,887,963,263,739,864];
  let result = largest(&list2);
  println!("Largest number is {:?}", result);
  
}