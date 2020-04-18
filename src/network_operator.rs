//! This module is used for handling operations related to networking

use std::thread;
use std::time;
use oping::{Ping};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum InputType {
    IPv4,
    IPv6,
    HOSTNAME,
    UNKNOWN
}

static mut IS_LOOPING: bool = true;
static mut SUCCESS: u32 = 0;
static mut FAILURE: u32 = 0;
static mut TIME: f64 = 0.0;

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
 * `address` - Value of the address
 */
pub fn ping(address: &str) {

    // keep pinging 
    unsafe {
        while IS_LOOPING {

            // ping every 1 second
            thread::sleep(time::Duration::from_millis(1000));

            // create ICMP packet using external library oping
            // while searching for external rust libraries, I realized
            // many of them are not being updated anymore or was abandoned.
            let mut ping = Ping::new();
            // max wait time is 5 seconds
            match ping.set_timeout(5.0) {
                Ok(_) => {},
                Err(err) => {
                    println!("{:?}", err);
                } 
            }
            // set host based on address type
            match ping.add_host(address) {
                Ok(_) => {},
                Err(err) => {
                    println!("{:?}", err);
                }
            }

            // send ICMP packet
            let responses = match ping.send() {
                Ok(iter) => iter,
                Err(err) => {
                    // timeout or sudo
                    println!("Please run with \"sudo\"");
                    println!("{}", err);
                    // update result as failure
                    FAILURE += 1;
                    continue;
                }
            };

            // check response and update result
            for response in responses {
                if response.dropped > 0 {
                    println!("No response from {} (loss)", response.address);
                } else {
                    // display success result
                    println!("Response from {} with {} ms", response.address, response.latency_ms);
                    // update result
                    SUCCESS += 1;
                    TIME += response.latency_ms;
                }
            }
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
    unsafe {
        println!("TOTAL  : {} packets", SUCCESS + FAILURE);
        println!("SUCCESS: {}", SUCCESS);
        println!("FAILURE: {}", FAILURE);
        // safe casting using keyword as
        println!("TIME   : {:.3} ms", TIME / (SUCCESS + FAILURE) as f64);
    }
}