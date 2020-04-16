//! This module is used for handling operations related to networking
extern crate nix;

use nix::sys::signal;

pub enum InputType {
    IPv4,
    IPv6,
    HOSTNAME,
    UNKNOWN
}

/**
 * This function is the callback function that will be called
 * when an interrupt signal is thrown.
 */
extern "C" fn handle_sigint(_: i32) {

    println!("Ouch!");

}

/**
 * This function registers an signal action handler to SIGINT.
 * At signal interrrupt, the function should let the ping loop
 * stop and create a report. 
 */
pub fn register_sig_action() {

    let sig_action = signal::SigAction::new(
        handle_sigint, 
        signal::SaFlags::SA_RESTART, 
        signal::SigSet::empty()
    );

}

/**
 * This function sends a ping and waits for response.
 * Based on the result, it updates the success_ratio.
 * 
 * `addr_type` - Type of input
 * `address` - Value of the address
 */
pub fn ping(addr_type: InputType, address: &str) {
    
}