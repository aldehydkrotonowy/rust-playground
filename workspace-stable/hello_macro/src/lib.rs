// When creating procedural macros, the definitions must reside in their own crate with a special crate type.
pub trait HelloMacro {
    fn hello_macro();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
