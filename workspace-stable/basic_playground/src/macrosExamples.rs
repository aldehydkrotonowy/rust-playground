#[macro_export] // annotation indicates that this macro should be made available whenever the crate in which the macro is defined is brought into scope.
macro_rules! myVec {
  ($($x:expr),*) => {// the pattern and code associated with this pattern
    let mut temp_vec = Vec::new();
    $(
      temp_vec.push($x);
    )*
    temp_vec
  };
}
