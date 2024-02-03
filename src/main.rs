struct Person {
    name: String,
    age: u8,
}

enum TrafficLight {
    Red,
    // Yellow,
    // Green,
}

fn main() {
    // Lesson 1: 
    // 🦀 Rust installation, Hello World, Hello Cargo - Rust tutorial - https://www.youtube.com/watch?v=R33h77nrMqc
    println!("Hello, world!");

    // Lesson 2:
    // Variables, Shadowing, Constants in Rust 🦀 Rust Tutorial ♋ - https://www.youtube.com/watch?v=6Ag0MZUlvBE
    let mut x = 10; // variables are immutable by default, to mutate them use 'mut' keyword
    // All variables must be initialized
    println!("x = {}", x); // 10
    x = 5;
    println!("x = {}", x); // 5

    // shadowing
    let x = 10;
    println!("x = {}", x); // 10    

    let x = 5;
    println!("x = {}", x); // 5
    let x = x+1;
    println!("x = {}", x); // 6

    // scopes
    {
        let x = 9;
        println!("x = {}", x); // 9
    }

    let x = x+1;
    println!("x = {}", x); // 7

    // x = "Hi"; // error, since x is defined i32

    let x = "Hi";
    println!("x = {}", x); // Hi
    
    // let x; // allowed
    // x = 5;
    const MAX_POINTS: u32 = 100_000;
    // type is mandatory for const, value must be assigned, shadowing doesnt work, cannot be mut
    println!("MAX_POINTS = {}", MAX_POINTS);

    // Lesson 3:
    // Rust Data Types - Rust tutorial - https://www.youtube.com/watch?v=NyqJp5M3hRE
    // data types
    // Scalar types
    // - Integers
    let small_int: u8 = 5;
    let big_int: u128 = 123456789;
    let small_negative: i8 = -127;
    println!("{}, {}, {}", small_int, big_int, small_negative);
    let decimal = 98.22;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
    println!("{}, {}, {}, {}, {}", decimal, hex, octal, binary, byte);
    // - Floating point
    let float_num: f64 = 3.14; // default
    let float_num_2: f32 = 3.44;
    println!("{}, {}", float_num, float_num_2);
    let quotient = 5 / 3; // 1
    let remainder = 5 % 3; // 2
    let quotient2 = 5.0 / 3.0; // 1.66666667
    println!("{}, {}, {}", quotient, remainder, quotient2);
    // - Boolean
    let is_active = true;
    if is_active {
        println!("active");
    } else {
        println!("inactive");
    }
    // - Character
    let letter = 'a'; // single quote for char, double quote for string
    let x_logo = '𝕏';
    let smiley = '✅';
    println!("{}, {}, {}", letter, x_logo, smiley);
    // Iterate over characters in a string
    for c in "namaste".chars() {
        println!("{}", c);
    }
    // Compound types
    // - Tuples
    let tup: (i32, f64, char) = (5, 4.4, 'x'); // tuples are generally returned from a function
    // Destructuring
    let (x,y,z) = tup;
    println!("{}, {}, {}", x, y, z);
    // Accessing for index
    println!("{}", tup.0);
    // - Arrays
    let arr = [1,2,3,4,5]; // size must be fixed and elements must be of same type
    println!("{}", arr[0]);
    for i in arr.iter() {
        println!("{}", i);
    }
    // Custom types
    // - Structs
    let person = Person {name: String::from("John"), age: 30};
    println!("{}", person.name);
    println!("{}", person.age);
    // - Enums
    let light = TrafficLight::Red;
    match light { // like switch case
        TrafficLight::Red => println!("Red"), // all cases must be handled otherwise error
        // TrafficLight::Green => println!("Green"),
        // TrafficLight::Yellow => println!("Yellow"),
    }

    // Lesson 4:
    // Functions in Rust - Rust Crash Course - https://www.youtube.com/watch?v=hJLc2Zu405A
    another_function(42, 'a'); // order of function declaration is not important

    // let num100 = num200 = 42; // error
    
    let res = {
        let x = 5;
        x+1 // semicolon should not present
    };
    println!("res = {}", res); // 6

    let sumdiff = sum_diff(5, 4);
    println!("sum = {}", sumdiff.0); // 9
    println!("diff = {}", sumdiff.1); // 1
    println!("{:?}", sumdiff); // 9,1 {:?} is used to print the tuple

    // Lesson 5:
    if 5<6 {
        println!("5<6");
    }
    let ret = if true { // if is an expression, not a statement
        5
    } else {
        6
    };
    println!("ret = {}", ret); // 5
    if 5<6 && 6>7 {
        println!("inside if");
    } else {
        println!("inside else"); 
    }
    let match_light = TrafficLight::Red;
    match match_light { // like switch case
        TrafficLight::Red => println!("Red"), // all cases must be handled otherwise error
        // TrafficLight::Green => println!("Green"),
        // TrafficLight::Yellow => println!("Yellow"),
    }
    // Infinite loop
    // loop {
    //     println!("Hello");
    // }
    
    let mut ctr = 0; // mutable is necessary
    let result = loop {
        ctr += 1; // ctr++ does not work
        if ctr == 10 {
            break ctr*2;
        }
    };
    println!("result = {}", result);

    let mut ctr = 3;
    while ctr != 0 {
        println!("ctr = {}", ctr);
        ctr -=1;
    }

    let a = [1, 2, 3, 4, 5];
    for element in a.iter() {
        println!("element = {}", element);
    }
    let s = "hello world";
    for c in s.chars() {
        println!("c = {}", c);
    }
    for num in 1..4 { // 1 to 3 only
        println!("num = {}", num); 
    }
    // Fizz Buzz
    // for num in 1..=100 { // or 101
    //     if num % 15 == 0 {
    //         println!("FizzBuzz");
    //     } else if num % 3 == 0 {
    //         println!("Fizz");
    //     } else if num % 5 == 0 {
    //         println!("Buzz"); 
    //     } else {
    //         println!("{}", num);
    //     }
    // }
}

fn another_function(num: i32, letter: char) { // type of parameter is mandatory
    println!("num = {}, letter = {}", num, letter);
}

// functions must be snake case, otherwise warning
fn sum_diff(num1: i32, num2: i32) -> ( i32, i32 ) {
    ( num1 + num2, num1 - num2 )
    // semicolon must not be present if using last line of block as return without return keyword
    // if using return keyword, semicolon is optional
}
