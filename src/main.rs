//Terminal Texting Program
//@rdar

use std::io;

fn main() {
    //init vars
    let mut phone_number = String::new();
    let mut text_message = String::new();

    //greet the user
    println!("Hello, welcome to the terminal texting program. Please enter the number you wish to text: ");

    //grab the phone number
    io::stdin().read_line(&mut phone_number).ok().expect("Sorry, we were not able to read that phone number");

    //ask user for the text message
    println!("Thanks, please enter the message you wish to send: ");

    //grab the text message
    io::stdin().read_line(&mut text_message).ok().expect("Sorry, we were not able to read that text message");

    //send the text message
}
