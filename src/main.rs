//Terminal Texting Program
//@rdar

extern crate hyper;

use std::io;
use std::io::Read;

use hyper::Client;
//use hyper::header::Connection;

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
    //first create the client
    let mut client = Client::new();

    //create the post request 
    let mut request = client.post("http://textbelt.com/text")
        .body("number=7342395992&message=thisisatesttest").send().unwrap();

    //read the response
    let mut body = String::new();
    request.read_to_string(&mut body).unwrap();

    //tell the user what's up
    println!("We sent your text message, here is the response from the textbelt api: {}",body);
}
