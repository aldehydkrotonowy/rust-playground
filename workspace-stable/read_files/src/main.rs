// use std::env;
// use std::fs;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // print!("{:?}\n", args);

    // let config = Config::new(&args);

    // print!("file name is {}\n", config.filename.unwrap());
    // let file_content =
    //     fs::read_to_string(config.filename.unwrap()).expect("cannot read content of file");
    // print!("content of file is {}\n", file_content);
}

// enum Params {
//     FILENAME,
//     PATH,
// }

// impl Params {
//     fn as_string(&self) -> &str {
//         match *self {
//             Params::FILENAME => "-f",
//             Params::PATH => "-p",
//         }
//     }
// }

// struct Config {
//     filename: Option<String>,
// }

// impl Config {
//     fn new(args_vec: &Vec<String>) -> Self {
//         if args_vec.len() < 2 {
//             print!("not enough parameters")
//         }
//         for (index, input) in args_vec.iter().enumerate() {
//             match input {
//                 Params::FILENAME::as_string() => Some(args_vec[index + 1]),
//                 Params::PATH::as_string() => Some(args_vec[index + 1]),
//                 _ => None,
//             };
//         }
//         let filename: String = args_vec[1].clone();
//         Config {
//             filename: Some(filename),
//         }
//     }
// }
