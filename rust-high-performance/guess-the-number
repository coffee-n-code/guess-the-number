use std::string::String;
use std::fs::File;
use std::io::{Read, BufRead};

static mut TRIES_LEFT: i32 = 5;
static mut RAND_INT: i32 = -1;
static mut GUESS: i32 = -1;

fn main() {
    unsafe {
    // Open /dev/random
    let mut devrandom = File::open("/dev/random").unwrap();
    // Create a 1 byte large buffer
    let mut randombyte: [u8; 1] = [0];
    // Read exactly 1 byte from /dev/random1 byte wide buffer
    devrandom.read_exact(&mut randombyte).unwrap();
    // Clamp it to 0-100 with modulo
    RAND_INT = (randombyte[0] as i32) % 100;

    // Get a handle to STDIN
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    // Create string to hold STDIN input
    let mut input = String::new();

    loop {
        if TRIES_LEFT == 0 {
            println!("Sorry, dude, but you lost. Better luck next time.");
            println!("The number you wanted was {}", RAND_INT);
            break;
        }
        println!("Make a guess: ");
        input.truncate(0); // clear any previous input
        handle.read_line(&mut input).unwrap();
        GUESS = match input.trim().parse() {
            Ok(integer) => integer,
            Err(_) => {
                println!("That's no integer, buddy.");
                continue;
            }
        };
        if GUESS < 0 || GUESS > 100 {
            println!("I can't believe you've done this! That's not in 0-100");
            continue;
        }

        // If we have a valid guess now, it counts as a try
        TRIES_LEFT -= 1;

        if GUESS == RAND_INT {
            println!("🎉 YOU WIN 🎉");
            break;
        } else if GUESS > RAND_INT {
            println!("Too high, guy.");
        } else {
            println!("Too low, bro.");
        }
    }
    }
}
