// If an Option type has Some value or a Result type has a Ok value, the value inside them passes to the next step.
// If the Option type has None value or the Result type has Err value, program panics; If Err, panics with the error message.

/* 
  unwrap(), expect()
  unwrap_err(), expect_err()
  unwrap_or(), unwrap_or_default(), unwrap_or_else()

  unwrap_or() : With None or Err, the value you passes to unwrap_or() is passing to the next step. 
  But the data type of the value you passes should match with the data type of the relevant Some or Ok.

*/
fn main() {
  // Example with Option and match, before using unwrap()
  let x;
  match get_optional_value() {
    Some(v) => x = v,
    None => panic!() // panic without message
  };
  println!("Before using unwrap(). Get_optional_value set x variable to  {}", x);


  // example with Result and match, before using unwrap()
  let y;
  match function_with_error() {
    Ok(v) => y = v,
    Err(e) => panic!(e) // panic with error msg
  }
}

fn get_optional_value() -> Option<str> {
  if false {
    Some('abc')
  }
  None
}

fn function_with_error() -> Result<u64, str> {
  if false {
    Err("some error".to_string())
  }
  Ok(223)
}