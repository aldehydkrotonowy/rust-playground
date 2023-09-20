pub fn run(){
	let mut numbers: [i32; 4] = [1,2,3,4];
	numbers[2] = 20;
	println!("{:?}", numbers);

	println!("Single value: {}", numbers[0]);
	println!("Array length: {}", numbers.len());
	println!("Array occupies {} bytes", std::mem:size_of_val(&numbers));
	
	let slice: &[i32] = &numbers[0..2];

	
}