//! This module is used for validating the input arguments

use crate::network_operator::InputType;

const DEBUG: bool = false;
const VAILD_LENGTH: usize = 2;

/**
 * This function checks the length of the input argument.
 * Returns true for valid argument length.
 * 
 * `args` - Reference to the arguments given to the program
 */
pub fn is_valid_length(args: &Vec<String>) -> bool {

    // since we are borrowing the immutable vector of arguments, we cannot modify it
    // however even if we pass in and take a mutable reference, the reference can only
    // be borrowed once in this scope (data race)
    // immutable borrows are fine

    if DEBUG {
        // Some types in Rust require a formatter to print
        println!("[DEBUG] validator >> args: {:?}", args);
    }

    // args should be of length 2 including the program path
    if args.len() != VAILD_LENGTH {
        return false;
    }

    // this is a valid expression for evaluating (return) the function
    // putting a semicolon will turn this from an expression into a statement
    true
}

/**
 * This function checks the type (IP or hostname) by matching with a regular expression.
 * Returns an enum with "hostname", "ipv4/6", or just UNKNOWN if unavailable.
 * This function has support for IPv4 and also IPv6
 * 
 * `_arg` - Reference to argument potentially containing the hostname / ip
 */
pub fn get_type_of_arg(_arg: &String) -> InputType {
    
    // check if it is an IPv4 or IPv6
    if is_ipv4(_arg) {
        return InputType::IPv4;
    } else if is_ipv6(_arg) {
        return InputType::IPv6;
    }

    // check if it is a hostname
    if is_hostname(&_arg) {
        return InputType::HOSTNAME;
    }
    
    // could not recognize type
    InputType::UNKNOWN
}

/**
 * This function is a helper function to check if a given argument is an IPv6
 * The format is based on the reference to IBM's knowledge center
 * https://www.ibm.com/support/knowledgecenter/en/STCMML8/com.ibm.storage.ts3500.doc/opg_3584_IPv4_IPv6_addresses.html
 * However, this function does not cover IPv6 dual addresses
 * 
 * `arg` - Reference to an argument to check
 */
fn is_ipv6(_arg: &String) -> bool {

    // first check for IPv6 by splitting by :
    let ipv6: Vec<&str> = _arg.split(":").collect();

    // 8 fields(segments) in an IPv6
    if ipv6.len() != 8 {
        return false;
    }

    // all four segments should be from 0 ~ 255
    for segment in ipv6 {

        // parse the segment from hex to 32 bit unsigned
        // using the match keyword instead of expect,
        // return false if unable to parse the segment
        // return the parsed num if successful
        let segment_hex: u32 = match u32::from_str_radix(segment, 16) {
            Ok(num) => num,
            Err(_) => {
                return false;
            },
        };

        if segment_hex > 0xFFFF {
            return false;
        }
    }

    true
}

/**
 * This function is a helper function to check if a given argument is an IPv4
 * The format is based on the reference to IBM's knowledge center
 * https://www.ibm.com/support/knowledgecenter/en/STCMML8/com.ibm.storage.ts3500.doc/opg_3584_IPv4_IPv6_addresses.html
 * 
 * `arg` - Reference to an argument to check
 */
fn is_ipv4(_arg: &String) -> bool {

    // first check for IPv4 by splitting by .
    let ipv4: Vec<&str> = _arg.split(".").collect();

    // 4 fields in an IPv4
    if ipv4.len() != 4 {
        return false;
    }

    // all four fields should be from 0 ~ 255
    for octet in ipv4 {

        // parse the octet into a 32-bit unsigned integer
        // using the match keyword instead of expect,
        // return false if unable to parse the octet
        // return the parsed num if successful
        let octat_u32: u32 = match octet.parse() {
            Ok(num) => num,
            Err(_) => {
                return false;
            },
        };

        if octat_u32 > 255 {
            return false;
        }
    }

    true
}

/**
 * This function is a helper function to check if a given argument 
 * is a format of a host name.
 * 
 * `arg` - Reference to an argument to check
 */
fn is_hostname(_arg: &String) -> bool {

    // hostname should be of a length less than 255 and atleast 1
    if _arg.len() > 255 && _arg.len() > 0 {
        return false;
    }

    // since it could be a local network,
    // the hostname could be anything
    true
}