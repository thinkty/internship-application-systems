# Cloudflare Internship Application: Systems

## What is it?

A small Ping CLI application for Linux.
The CLI app accepts a hostname or an IP address as its argument, then send ICMP "echo requests" in a loop to the target while receiving "echo reply" messages.
It reports loss and RTT times for each sent message.

Made with Rust

## Rust Tutorial

- [The Rust Programming Language](https://doc.rust-lang.org/book/index.html)

## Requirements

### 0. Permission

This program requires "sudo" to run properly

### 1. Language used

I am taking this as an opportunity to learn Rust. I am intrigued by the ability of Rust to manage memory without the user having to consistently worry about freeing it. Since I am learning Rust, there will be notes left on the source code for my studies.

### 2. Build a tool with a CLI interface

The tool should accept as a positional terminal argument a hostname or IP address.

### 3. Send ICMP "echo requests" in an infinite loop

As long as the program is running it should continue to emit requests with a periodic delay.

### 4. Report loss and RTT times for each message

Packet loss and latency should be reported as each message received.

## Extra Credit

1. Added support for both IPv4 and IPv6