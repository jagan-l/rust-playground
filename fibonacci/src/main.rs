use std::io;
/*
fn fibonacci(n:i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fibonacci(n-1) + fibonacci(n-2)
}



fn main() {
    println!("Enter a number");

    let mut number = String::new();
    io::stdin().read_line(&mut number)
        .expect("please a enter a positive number");
        let number: i32 = number.trim().parse()
        .expect("Invalid number");

    if number < 0 {
        panic!("Invalid number for Fibonnaci");
    }

    println!("Fibonacci of {} is {}", number, fibonacci(number));
    

}*/

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 2,
        _ => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn main() {
    println!("Type \"quit\" to end the program");
    loop {
        let mut n = String::new();
        println!("Enter a positive integer:");
        io::stdin().read_line(&mut n)
            .expect("failed to read the integer");
        
        if n.trim() == "quit" {
            break;
        }
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("{}", fibonacci(n));
    }

}