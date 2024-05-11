
// Disables "unused" warnings
#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind}; // for working with files
use std::fs::File;
use std::cmp::Ordering;
use std::string::String;
use std::collections::HashMap;

use crate::business::{toyota, Company};

mod business;

fn main() {
    /*
     * ---------- SEND ERROR ----------
     */
    //panic!("This is an error!");

    /* 
     * ---------- FILE IO ----------
    */
    // ----- WRITING -----
    let file_path: &str = "my_file.txt";
    let output = File::create(file_path);

    // Create file
    let mut result: File = match output {
        Ok(file) => file,
        Err(error) => panic!("Failed to create file!"),
    };

    // Write to file
    write!(result, "Hello, World!").expect("Failed to write to file!");

    // ----- READING -----
    let input = File::open(file_path).unwrap();
    let reader = BufReader::new(input);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }

    /*
     * ---------- CLOSURES ----------
     */
    let my_closure = || println!("This message is from a closure!");

    my_closure();

    /*
     * ---------- PRINT ----------
     */
    println!("Hello, world!");

    // "mut" means "mutable" meaning it can be changed
    let mut name = String::new();

    // Variables are immutable (cannot change) by default
    let greeting: &str = "Welcome to my program!";

    println!("What is your name?");

    // Gets the users input and sets it to the mutable variable "name"
    // ".expect" is like an error catcher
    io::stdin().read_line(&mut name).expect("Didn't receive input");

    // trim_end() prevents new lines
    println!("Hello {}! {}", name.trim_end(), greeting);

    // "const" means constant
    // u32 means "32-bit unsigned integer type and cannot have negative numbers"
    let age: &str = "18";

    // "f64" has double precision

    // Converts the string value "age" to a number.
    let mut age_num: u32 = age.trim().parse().expect("Age isnt assigned a number");

    // Boolean
    let mut can_drink: bool = age_num >= 18;

    // Match is basically a switch in java 17 when it was used in "return" and stuff.
    let drink_message = match can_drink {
        false => "Geuxy is too young to drink!",
        true => "Geuxy is able to drink!",
    };

    println!("{}", drink_message);
    println!("Max value of u32 is {}", u32::MAX);

    /*
     * ---------- F32 AND F64 NUMBERS ----------
     */
    // F32 has less decimal numbers than F64 (twice as much)
    let thirty_two: f32 = 1.111111111111111111;
    let sixty_four: f64 = 1.111111111111111111;

    println!("F32 - {}", thirty_two);
    println!("F64 - {}", sixty_four);

    /* 
     * ---------- RANDOM NUMBERS ----------
     */
    // Get a random number between the range of 1-69 (1 to 69).
    let random_num = rand::thread_rng().gen_range(1..69);
    println!("Random number is {}", random_num);

    // WHAT IS THIS GOOFY AHH CODE???
    let can_vote: bool = if age_num >= 18 {
        true
    } else {
        false
    };

    let demerit_points: i32 = 8;

    /*
     * ---------- MATCH ----------
     * Match is literally just a switch statement
     */
    match demerit_points {
        // Range in 1 to 3, or is 5
        1..=3 | 5 => println!("Geuxy got a fine!"),
        
        // Is 4 or 6
        4 | 6 => println!("Geuxy got a fine and lost their permit!"),
        7..=i32::MAX => println!("Geuxy got a fat fine and is now in jail!"),

        // "_" means default, in this scenario anything thats not included in the cases above are
        _ => println!("Bad number, Congratulations! you escaped \"the matrix\"!"),
    };

    // "match" on numbers in a stranger but interesting way
    // "cmp" means "compare" and is used to compare something with another thing.
    let age_to_drink = 18;

    match age_num.cmp(&age_to_drink) {

        // If age_to_drink is less than age_num
        Ordering::Less => println!("Too young to drink!"),
        Ordering::Equal | Ordering::Greater => println!("Legal to drink!"),
    };

    /*
     * _--------- ARRAYS ----------
     */
    // "&str" (string) is the value type and "4" is the maximum array size.
    let my_array: [&str; 4] = ["Im", "Currently", "Learning", "Rust!"];

    // Foreach loop
    for var in my_array {
        println!("{}", var);
    }

    // For loop
    for i in 0..my_array.len() {
        println!("{}", my_array[i]);
    }

    // Prints the length of the array (the amount of elements)
    println!("Array length is: {}", my_array.len());
    println!("First array element is: {}", my_array[0]);

    /*
     * ---------- TUPLES (PYTHON MENTIONED!?!??) ----------
     */
    // bro what
    let my_tuple: (&str, &str, &str, &str) = ("OMG", "Python", "Mentioned", "?!??!");

    let(what, is, this, bruh) = my_tuple;

    println!("{} {} {} {}", what, is, this, bruh);
    
    /*
     * ---------- CASTING ----------
     */
    let my_num: i32 = 10;
    let casted_num: u32 = (my_num as u32);

    println!("Casted Number: {}", casted_num);

    /*
     * ---------- ENUMS (my beloved) ----------
     */
    enum Activity {
        EAT,
        CODE,
        SLEEP

    

    }

    impl Activity {

        // A function that checks if the activity equals "CODE"
        fn is_highest_priority(&self) -> bool {
            match self {
                Activity::CODE => true,
                _ => false,
            }
        }
    }

    let activity:Activity = Activity::CODE;

    println!("{}", activity.is_highest_priority());

    /*
     * ---------- Vectors ----------
     */
    // Vectors are basically arrays but is resizable

    // Initialize empty
    let mut my_vec: Vec<i32> = Vec::new();

    // Initialize with values
    let my_vec2: Vec<i32> = vec![1, 2, 3];

    /*
     * ---------- Functions ----------
     */
    println!("{}", is_age_of_consent(age_num));

    /*
     * ---------- String Pushing / different string class thingy? ----------
     */
    let mut trimmed_name: String = String::from(name.clone().trim_end());

    // Adds this to the end of the string
    trimmed_name.push_str(" is learning rust!");

    println!("{}", trimmed_name);

    /*
     * ---------- Hash Maps ----------
     */
    let mut languages: HashMap<u32, &str> = HashMap::new();

    languages.insert(0, "Rust");
    languages.insert(1, "Java");
    languages.insert(2, "C#");
    languages.insert(3, "Python");

    for(key, val) in languages.iter() {
        println!("Rank {} goes to {}", key, val);
    }

    // "1" is the key aka Java as value
    if(languages.contains_key(&1)) {

        let java = languages.get(&1);

        // Prints the key
        // .unwrap() takes the Some("") value out the value.
        println!("Index 1 key: {}", java.unwrap());

        // Checking with Some()
        match java {
            Some(x) => println!("Java is a language!"),
            None => println!("Java is not a language!"),
        };
    }

    /*
     * ---------- Struct ----------
     */
    struct User<'a> {
        name: &'a str,
        adult: bool,
    }

    let geuxy: User = User{name: "Geuxy", adult: true};

    println!("{} is an adult?: {}", geuxy.name, geuxy.adult);

    /*
     * ---------- Traits ----------
     * Literally an interface in java terms
     */
    trait TwoDim {
        // Constructor
        fn new(a: i32, b: i32) -> Self;
        fn print(&self);
    }

    struct Pos {a: i32, b: i32};

    impl TwoDim for Pos {
        fn new(a: i32, b: i32) -> Pos {
            return Pos{a, b};
        }

        fn print(&self) {
            println!("X: {}, Y: {}", self.a, self.b);
        }
    }

    let my_pos: Pos = TwoDim::new(52, 14);

    my_pos.print();

    /*
     * ---------- MODULES ----------
     */

    // Using structs from module
    // "toyota()" is from the business module
    let toyota: Company = toyota();
    toyota.print();



}

fn my_func(text: &str, num: i32) {
    println!("{} = {}", text, num);
}

fn is_age_of_consent(age: u32) -> bool {
    return age >= 18;
}
 