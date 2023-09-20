pub fn run(){
	let age = 18;
	if age >= 21 {
		println!("Bartender: a dsafkj");
	} else {
		println!("Sorry you to young");
	}

	let is_of_age = if age >= 21 {true} else {false};
	println!("Is of age {}", is_of_age);
}