use std::fmt::Display;
use std::io::Error;
use std::{fs, io};

fn main() {
    let ans = is_even();

    println!("This is even {:?}", ans);

    println!("Enter a string");

    let mut val = String::new();

    io::stdin()
        .read_line(&mut val)
        .expect("Failed to read line");

    println!("Your string length is {}", find_str_len(&val));

    let user = User::new(
        String::from("Bhanu"),
        String::from("bhau@beti.com"),
        1000,
        true,
    );

    println!("{:?}", user);

    user.greeting();

    let nums = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    for (i, num) in nums.iter().enumerate() {
        println!("index -> {i} value -> {num}");
    }

    let my_direction = Direction::Up;

    match my_direction {
        Direction::Up => {
            println!("You got the up!");
        }
        Direction::Down => {
            println!("You got the down!");
        }
        Direction::Left => {
            println!("You got the left!");
        }
        Direction::Right => {
            println!("You got the right!");
        }
    }

    let dr = give_direction(Direction::Right);

    println!("You got the dr! {:?}", dr);

    let radius = Shape::Circle(14.00);
    let square = Shape::Square(24.00);

    match radius {
        Shape::Circle(circle) => {
            println!("The radius of circle is {}", circle * circle);
        }
        Shape::Square(square) => {
            println!("The area is {}", square * square);
        }
    }

    println!("The circle is {}", radius.area());
    println!("The square is {}", square.area());

    //two properties of enums Options/Results

    //they are used for null pointers values and error handling

    let character = find_input_char(String::from("Meet Kumar"), 'e');

    match character {
        Some(a) => println!("The character is present on index number {}", a),
        None => println!("The character is not present"),
    }

    let file_path: Result<String, Error> = fs::read_to_string("src/test.txt");

    file_result(file_path);

    let ss = "test";
    borrowing_str(ss);

    let mut mutation_string = String::from("Meet Kumar");

    mutating_str(&mut mutation_string);

    println!("The mutation_string is {}", mutation_string);
}

fn mutating_str(s: &mut String) {
    s.push_str(" checking mutation");
}

fn borrowing_str(s: &str) {
    // println!("{:p}", &s); // print address
    println!("{s}");
}

// Results ->  if it will return either Ok value or Err value
fn file_result<Error: Display>(file_path: Result<String, Error>) {
    match file_path {
        Ok(file_content) => println!("{}", file_content),
        Err(error) => println!("{}", error),
        // Err(error) => panic!("There was a problem opening the file: {:?}", error),
    }
}

// Options ->  if return none/null/nil values from a function, means it will return either some value or not value
fn find_input_char(s: String, c: char) -> Option<i32> {
    for (i, ch) in s.chars().enumerate() {
        if ch == c {
            return Some(i as i32);
        }
    }

    return None;
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
}

impl Shape {
    fn area(&self) -> f64 {
        let res = match self {
            Shape::Circle(circle) => circle * circle,
            Shape::Square(square) => square * square,
        };

        return res;
    }
}

fn give_direction(direction: Direction) -> Direction {
    return direction;
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn greeting(&self) {
        println!("Hello, {}", self.username);
    }

    fn new(username: String, email: String, sign_in_count: u64, active: bool) -> User {
        return User {
            active: active,
            username: username,
            email: email,
            sign_in_count: sign_in_count,
        };
    }
}

fn find_str_len(val: &str) -> usize {
    val.chars().count() - 1
}

fn is_even() -> &'static str {
    println!("Enter your number: ");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let res: i32 = number.trim().parse().expect("Not a number");

    let res = if res % 2 == 0 { "yes" } else { "no" };

    return res;
}
