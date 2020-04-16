//! This is the main file for the ping CLI.
//! In this file, the program will ask the user to input a hostname or an IP address.
//! Then, the program will send an ICMP "echo request" in a loop.
//! Finally, the program will report task results (loss, time) for each request.

use std::env; // for program arguments
mod network_operator;
mod validator;

fn main() {

    // fetch argument
    // args() returns an iterator and collect() turns it into a vector
    let args: Vec<String> = env::args().collect();

    // validate argument - check argument length
    // due to ownership, we give a reference of the vector to the function
    if !validator::is_valid_length(&args) {
        println!("Usage: ping [<hostname> | <ip>]");
        return;
    }

    // get argument type (ip, hostname)
    let arg_type: network_operator::InputType = validator::get_type_of_arg(&args[1]);
    if arg_type == network_operator::InputType::UNKNOWN {
        println!("Error: could not recognize the input type (hostname or IP)");
        return;
    }
    
    // println! is a macro
    println!("__________[ Ping ]__________");
    println!("Received a type: {:?}", &arg_type);
    println!("Use ctrl-C to get report");

    // add signal interrupt handler
    network_operator::register_sig_action();

    // ping
    network_operator::ping(arg_type, &args[1][..]);
}
