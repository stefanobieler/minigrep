//! This is a combination lock

#![allow(dead_code)]
#![allow(unused_imports)]

use std::io::stdin; // using the stdin namespace.

/// All the different States that exist.
enum State {
    Locked,
    Failed,
    Unlocked,
}


// Where all the magic happens.
fn main() {
    let code = String::from("1234");     // First we set a combination.
    let mut state = State::Locked;                  // Set the initial state to locked.
    let mut entry = String::new();          // Set the entry to an empty string.

    // the main loop.
    loop {
        // Check what state we are currently in.
        match state {
            // If the current state is equal to locked.
            State::Locked => {
                // set the input to a new empty string.
                let mut input = String::new();
                /// read and check if we read successfully from the line.
                /// read_line returns an io::Result<T, Error> which is an enumeration that has
                /// two states; Ok and Err.
                match stdin().read_line(&mut input) {
                    // if read_line returned a result that was Ok
                    Ok(_) => {
                        entry.push_str(&input.trim_end()); // push the input onto the entry.
                    }
                    // if read_line returned a result that was Err
                    Err(_) => {
                        continue; // restart and read input again.
                    }
                }

                /// if the entry is equal to the code then unlock.
                if entry == code {
                    state = State::Unlocked;        // set state to unlock
                    continue;                       // return to main loop.
                }

                /// right when the user inputs the wrong code
                /// change the state to failed.
                if !code.starts_with(&entry) {
                    state = State::Failed;      // set the state to failed.
                    continue;                   // return to main loop
                }
            }

            // Failed state
            State::Failed => {
                println!("FAILED");     // print to the users that they failed.
                entry.clear();          // clear the entry string.
                state = State::Locked;  // Set the state = to locked for next loop
                continue;               // Restart at loop
            }

            State::Unlocked => {
                println!("UNLOCKED");
                return;
            }
        }
    }
}
