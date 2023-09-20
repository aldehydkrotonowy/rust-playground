pub trait SomeTrait {
    fn is_valid(&self) -> bool;
    // fn get_the_better_one(&self, some_other_dude: &Self) -> Self;
}
#[derive(Clone)]
pub struct RandomInfo {
    pub call_count: i64,
    pub some_bool: bool,
    pub some_int: i32,
}

impl SomeTrait for RandomInfo {
    fn is_valid(&self) -> bool {
        self.some_bool
    }
}

impl Default for RandomInfo {
    fn default() -> Self {
        Self {
            call_count: 0,
            some_bool: true,
            some_int: 15
        }
    }
}

impl RandomInfo {
    pub fn new(param_a: bool) -> Self {
        Self {
            call_count: 0,
            some_bool: !param_a,
            some_int: 8,
        }
    }
    pub fn is_smaller(&mut self,compare_to: i32) -> bool {
        self.call_count  += 1;
        self.some_int < compare_to
    }
}
