//! This module is used for handling operations related to networking

use std::thread;
use std::time;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum InputType {
    IPv4,
    IPv6,
    HOSTNAME,
    UNKNOWN
}

static mut IS_LOOPING: bool = true;

/**
 * This function registers an signal action handler to SIGINT.
 * At signal interrrupt, the function should let the ping loop
 * stop and create a report. 
 */
pub fn register_sig_action() {
    
    // set an action handler
    match ctrlc::set_handler(move || {
        // set the value of is_pinging to false to stop
        // since it is not threadsafe, use unsafe
        // however, this program only uses main thread so it's fine
        unsafe {
            IS_LOOPING = false;
        }

    }) {
        Ok(_) => {}
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

/**
 * This function sends a ping and waits for response.
 * Based on the result, it updates the success_ratio.
 * 
 * `addr_type` - Type of input
 * `address` - Value of the address
 */
pub fn ping(addr_type: InputType, address: &str) {

    // keep pinging 
    unsafe {
        while IS_LOOPING {

            // ping every 1 second
            thread::sleep(time::Duration::from_millis(1000));

            // get start time
            let curr_time: time::SystemTime = time::SystemTime::now();

            // create ICMP packet


            // send ICMP packet

            // wait for response
            let resp_time: time::SystemTime = time::SystemTime::now();
            let diff: time::Duration = resp_time.duration_since(curr_time)
                .expect("Error with getting duration");
            println!("Received from {} time={:?}ms", &address, &diff.as_millis());

            // parse response

            // update result

        }
    }

    // print result
    print_result();
}

/**
 * This function prints the result of pings to the terminal
 */
fn print_result() {
    println!("--- Ping result ---");

}

/**
 * This function creates an ICMP packet to send
 * 
 * `addr_type` - Type of input
 * `address` - Value of the address
 */
fn create_icmp_packet(addr_type: InputType, address: &str) {

    let mut socket_addr: SocketAddr;

    match addr_type {
        InputType::HOSTNAME => {
            match socket_addr = address.to_socket_addrs() {
                Ok(addr) => addr
                Err(err) => {
                    println!("{}", err)
                }
            }

        }
        InputType::IPv4 => {

        }
        InputType::IPv6 => {

        }
        InputType::UNKNOWN => {

        }
    }

}