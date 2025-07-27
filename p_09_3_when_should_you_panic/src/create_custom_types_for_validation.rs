// The guessing game in p1 had code that asked the user to guess
// a number between 1 and 100
// The user's guess was never validated to be between those numbers
// It was only checked to be higher or lower
// In that case, it was not a huge issue but the program would've been better
// if there was a response for guesses outside the valid range
// that would be different from say, a user typing a letter

// This is a snippet of a change to the guessing game
// that would include a range check for the guessed word
fn guess_snippet_1(guess: &str, secret_number: i32) {
    loop {
        // --snip--

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if (1..=100).contains(&guess) {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => continue,
            std::cmp::Ordering::Equal => break,
            std::cmp::Ordering::Greater => continue,
        }
    }
}

// That solution is still not the best
// Consider a program where it is absolutely critical that the program
// only operated on values between 1 and 100, and it had many functions with this requirement
// Constantly checking that the value is between 1 and 100 for every function would not only be
// tedious it might even impact performance

// Best solution
// Make a new type in a dedicated module and
// put the validations into a function to create an instance of the type
// this prevents repeating the validations everywhere
// This makes it safe for functions to use the new type in their signatures
// and be confident that the type adheres to its contract
// It would panic if value does not pass the test so you have to create
// an API documentation covering instances where it panics.
// More on that in chapter 14

// For the guessing game this would be implemented as so
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if !(1..=100).contains(&value) {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    // It is important that the underlying value is private
    // so code cannot set the value directly
    // Doing so would undermine the checks made in new()
    pub fn value(&self) -> i32 {
        self.value
    }
}
