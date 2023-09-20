trait SomeTrait {
    fn method(&self) -> String;
}

impl SomeTrait for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}
impl SomeTrait for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

fn do_this<T:SomeTrait>(x:T){
    x.method();
}

fn main() {
    let a  = 5u8;
    let b = "Hello".to_string();
    
    do_this(a);
    do_this(b);
}