//traditional struct

struct Color {
	red: u8,
	green: u8,
	blue: u8,
}

//tuple structure
struct Color2(u8, u8, u8);

// something more complicated
struct Person {
	first_name: String,
	last_name: String,
}

impl Person {
	fn new(first: &str, last: &str) -> Person {
		Person {
			first_name: first.to_string(),
			last_name: last.to_string(),
		}
	}
	fn full_name(&self) -> String {
		format!("{} {}", self.first_name, self.last_name)
	}
	fn set_last_name(&mut self, last: &str) {
		self.last_name = last.to_string();
	}
}

pub fn run() {
	let mut c = Color {
		red: 255,
		blue: 0,
		green: 0,
	};

	c.red = 200;

	println!("Color1 {} {} {}", c.red, c.blue, c.green);

	let mut c2 = Color2(255, 99, 5);
	c2.0 = 200;
	println!("Color2 {} {} {}", c2.0, c2.1, c2.2);

	let mut p = Person::new("Jon", "Doe");
	println!(
		"Person first name is: {} and last name is: {}",
		p.first_name, p.last_name
	);
	println!("Person full name is {}", p.full_name());
	p.set_last_name("William");
	println!("Person full name is {}", p.full_name());
}
