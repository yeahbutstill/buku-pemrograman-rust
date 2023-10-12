use std::io; 
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    // let untuk membuat variable baru
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new(); // default variable di rust itu immutable, untuk menjadi mutable kita harus menambahkan keyword mut  
    let _apples = 45; // immutable
    let _bananas = 60; // mutable
    
    println!("The secret_number is: {secret_number}");

    println!("Please input your guess.");
    
    io::stdin()
        .read_line(&mut guess) // tanda & menunjukan bahwa argument ini adalah refrensi
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    let x = 15;
    let y = 10;
    println!("x = {x} and y + 2 = {}", y+2);
}
