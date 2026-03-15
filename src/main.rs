use std::io;

fn main() {

 // initialize input buffers

    let mut first_input = String::new();
    let mut second_input = String::new();
    let mut operator_input = String::new();

    // read inputs from user

    println!("Enter first number : ");
    io::stdin()
        .read_line(&mut first_input)
        .expect("Failed to load");

    println!("Enter  operator : ");
    io::stdin()
        .read_line(&mut operator_input)
        .expect("Failed to load");
    println!("Enter second number : ");
    io::stdin()
        .read_line(&mut second_input)
        .expect("Failed to load");

    // parse strings to integers

    let num1: i32 = first_input.trim().parse().expect("enter valid number");
    let operator = operator_input.trim().to_string();
    let num2: i32 = second_input.trim().parse().expect("enter val;id number");

   // calculate and print result

    let result = cal_result(num1, operator, num2);
    println!("{result}")
}


   
// calculates result based on operator

fn cal_result(first_input: i32, operator: String, second_input: i32) -> i32 {
    if operator == "+" {
        first_input + second_input
    } else if operator == "-" {
        first_input - second_input
    } else if operator == "*" {
        first_input * second_input
    } else if operator == "/" {
        first_input / second_input
    } else {
        println!("Invalid Input");
        0
    }
}
