use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // let untuk membuat variable baru
    let mut guess = String::new(); // default variable di rust itu immutable, untuk menjadi mutable kita harus menambahkan keyword mut  
    let _apples = 45; // immutable
    let _bananas = 60; // mutable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
