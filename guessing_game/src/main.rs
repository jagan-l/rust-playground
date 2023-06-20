//using the rand crate as an external dependency
extern crate rand;


/*io (input/output) library. io library comes from the standard library(std)
 if the type you want to use isn't in the prelude, you should bring the tyepe into scope with the "use" stataement
*/
use std::io;
//now we can use anything in the rand crate by placing rand:: before it
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,100);
    println!("The secret number is: {}", secret_number);
    //rand::thread_rng = this generates the perticular random number generator that we are going to use.
    // gen_range = method takes 2 numbers as arguments and generates a random number between them

    println!("Please input your guess. ");
/*let statement, used to create as variable, Variable are immutable by default.
mut before the variable name to make a variable mutable
*/
    let mut guess = String::new();
//let mut guess will introduce a mutable variable named guess which is a result of calling String::new, a function that returns a new instance of a string.
//The stdin function returns an instances of std::io::stdin, which is a type that represents a handle to the standard input to the terminal
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
//read_line takes the user types into the standard input and place it into the string, 
//"&" indicates the reference which gives you a way to let multiple parts of your code to access one piece of data without needing to copy that data into memory multiple times.
// like variables, references are immutable by default, hence we added "&mut guess" to make it mutable

    let guess: u32 = guess.trim().parse()
       .expect("Please type a number! ");
//creating a shawdow value witht he new one.
//trim method on a String instance will eliminate any whitespaces at the beginning and end.
//parse method on strings parses a string into some kind of number.
//(:) after guess tells Rust will annotate the variable's type and then uses u32 bit integers.


    println!("you guessed: {}", guess);



    match guess.cmp(&secret_number) {
        //cmp method compares 2 values, here its comparing the guess to the secret number
        //Ordering is another enum like Results, varients are Less, Greater and Equal
        //match decides what to do next based on which variant of Ordering was returned from the call to cmp with the values in guess and secret number

        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

/*
For example, when the code compares 50 to 38, cmp method will return Ordering:greater, match expression gets the Ordering::Greater value and starts checking each arm's pattern. when it does match the assocaited code in that arm will excecute and print Too big!
*/