// Works under Rust 1.14.0 :)
extern crate rand;

use std::io::prelude::*;
use std::io;
use rand::{thread_rng, Rng};

const MAX_ATTEMPTS:i16 = 5;

fn play(guess:u8, target:u8, remaining_guesses:i16) -> bool {
	if guess == target {
		// We won!
		println!("Noice! You gone done did good, kid.");
		return true;
	} else {
		if guess < target {
			// Go lower
			println!("Hmm too low, bro.");
		} else if guess > target {
			// Go higher
			println!("Yo too high, guy.");
		}
		println!("You have {0} guesses left.
", remaining_guesses);
		return false;
	}
}

fn main() {
	
	println!("
Yo! Welcome to Guess the Number with Rust!
Rust will, like, pick a number between 1 and 100 and, like, you've gotta guess it or whatever.

Oh. And you only get 5 guesses. Bonne chance!
");
	let mut rng = thread_rng();
	let _number = rng.gen_range::<u8>(1, 100);
	let mut _attempt = 1;
	
	// Our game loop
	loop {
		let position = MAX_ATTEMPTS - _attempt;
		
		if position < 0 {
			println!("Ouf. Sorry but you're all out of guesses. You lose =(");
			println!("The number you were looking for is {}", _number);
			break;
		}
		
		// Get input:
		let mut input_text = String::new();
		
		print!("Take a guess: ");
		io::stdout().flush().ok(); //.expect("Could not flush stdout");
		io::stdin().read_line(&mut input_text).unwrap();
		
		match input_text.trim().parse::<u8>().ok() {
			Some(i) => if play(i, _number, position) {
				break;
			} else {
				_attempt += 1;
			},
			None => println!("Enter a number please...")
		}
	}
}