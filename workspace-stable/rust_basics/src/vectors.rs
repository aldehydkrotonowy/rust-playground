pub fn run(){
	let mut numbers: Vec<i32> = vec![1,2,3,4];
	numbers[2] = 20;

	numbers.push(5);
	numbers.push(6);
	numbers.pop();
	println!("{:?}", numbers);

	println!("Single value: {}", numbers[0]);
	println!("Vector length: {}", numbers.len());
	println!("Vector occupies {} bytes", std::mem:size_of_val(&numbers));
	
	let slice: &[i32] = &numbers[0..2];

	for x in numbers.iter() {
		println!("Number: {}", x);
	}

	for x in numbers.iter_mut() {
		*x *=2;
	}
	println!("Numbers after mutation: {:?}", numbers);
}