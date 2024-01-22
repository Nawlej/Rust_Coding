#![allow(non_snake_case)]
fn main() {
    let temp: f32 = convert_To_Temp("c", 50.0);
    println!("{temp}");

    let index = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for number in index {
        let sum: i64 = fibonnacci_num(number);
        println!("the fibonnacci number at {number} is: {sum}")
    }

    //using mutable strings and not string literals
    let mike = employee(String::from("Mike"), String::from("Ock"), 17, String::from("CEO"));
    println!("{}", mike.1);

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    print!("{hello}");
    print!("{world}");

    let mark = build_employee( String::from("Mark"),  String::from("Johnson"), 32,  String::from("Janitor"));
    println!("{}", mark.last_name);
    
}

//convert to farenheit or to celsius
fn convert_To_Temp(text:&str, temp: f32) -> f32{
    if text.to_lowercase() == "f" {
        temp*9.0/5.0+32.0
    } else if text.to_lowercase() == "c"{
        (temp-32.0)*5.0/9.0
    } else {
        println!("Unknown temperature. Use F to indicate converting to Farenheit or C for Celsius");
        -0.0
    }
}

//generate the nth fibonacci number
fn fibonnacci_num(index: i64) -> i64{
    let mut num1:i64 = 0;
    let mut num2:i64 = 1;
    let mut sum:i64 = 0;
    let mut iterate:i64 = 2;
    if index < 2 {
        return index
    }
    while iterate <= index{
        sum = num1 + num2;
        num1 = num2;
        num2 = sum;
        iterate += 1;
    }
    sum
}

//creating a tuple with a function
fn employee (first_name: String, last_name: String, age: i32, position: String) -> (String, String, i32, String) {
    (first_name, last_name, age, position)
}

fn build_employee (first_name: String, last_name: String, age: i32, position: String) -> employee {
    employee{
        first_name,
        last_name,
        age,
        position,
    }
}
struct employee {
    first_name: String,
    last_name: String,
    age: i32,
    position: String,
}