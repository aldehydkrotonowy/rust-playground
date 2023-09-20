#[warn(unused_variables)]
pub fn find_and_show() {
    let haystack = " ab c d e";
    let found = haystack.find(' ').unwrap();
    let exper = &haystack[..found];
    print!("{:?}", exper)
}
