mod random_info;
// mod pointer_ref;
// mod structures;
// mod vect_hash_maps;

use random_info::*;
const PI: f32 = std::f32::consts::PI;

#[allow(dead_code)]
#[derive(Clone)]
struct DougsData {
    some_bool: bool,
    some_float: f32,
    some_int: i32,
    random: RandomInfo,
}

impl RandomInfo {
    pub fn is_larger(&self, compare_to: i32) -> bool {
        self.some_int > compare_to
    }
}

impl SomeTrait for DougsData {
    fn is_valid(&self) -> bool {
        true
    }
}

impl Default for DougsData {
    fn default() -> Self {
        Self {
            some_bool: false,
            some_float: PI,
            some_int: 5,
            random: RandomInfo::default(),
        }
    }
}

fn pring_if_is_valid(check_me: &dyn SomeTrait) {
    if check_me.is_valid() {
        println!("Horray!");
    }
}

#[allow(unused_variables)]
fn main() {
    // pointer_ref::run();
    // structures::run();
    let mut random_info_var = RandomInfo {
        call_count: 0,
        some_int: 334,
        some_bool: true,
    };

    let is_this_smaller = random_info_var.is_smaller(500);
    let is_this_larger = random_info_var.is_larger(100);
    let is_valid = random_info_var.is_valid();

    let mut dougs_var = DougsData {
        some_bool: true,
        some_float: 33.3,
        some_int: 34,
        // random: RandomInfo { // initialize random variable - method 1
        //     some_bool: false,
        //     some_int: 339,
        // }
        random: RandomInfo::new(true), // initialize random variable - method 2
    };

    dougs_var.some_int = 100;

    let dougs_var_2 = DougsData {
        some_int: 200,
        ..dougs_var.clone()
    };

    pring_if_is_valid(&random_info_var);
    pring_if_is_valid(&dougs_var);
}
