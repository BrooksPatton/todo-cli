// use std::env;

fn main() {
    // let arguments: Vec<String> = env::args().collect();
    // println!("{:?}", arguments);

    // declare variables
    let variable_name = "hello world!";
    println!("{}", variable_name);

    // inferred most of the time

    // manually declare the type

    // some types (str, String, integer, float)
    let my_str: &str = "hi there";
    let my_string: String = String::from("I am a String");
    let my_full_int: i32 = -55;
    let mut my_positive_int: u32 = 190;
    let my_float: f32 = 190.2345;

    // shadowing variables
    let my_str = String::from("hello world");

    // scope rules
    let my_var_in_this_scope;
    {
        let my_blocked_number = 45;
        my_var_in_this_scope = 1000;
    }
    // println!("blocked number: {}", my_blocked_number);
    println!("positive int: {}", my_positive_int);
    println!("var in this scope: {}", my_var_in_this_scope);

    // inmutable by default
    my_positive_int = my_positive_int + 1;

    // memory management
}
