use std::env;
use std::net::{TcpListener, TcpStream};
use std::io::{copy};
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();

    let server_ip = &args[2];
    let host_port = &args[1];
    let server_port = &args[3];

    let listener = match TcpListener::bind(String::from(String::from("127.0.0.1") + ":" + host_port))
    {
        Ok(r) => r,
        Err(e) => {
            println!("Failed to bind to port {:?}", e);
            return;
        }
    };

    let downstream = match listener.accept()
    {
        Ok((socket, _addr)) => socket,
        Err(e) => {
            println!("Unable to get connection {:?}", e);
            return;
        }
    };

    let upstream = match TcpStream::connect(String::from(server_ip.to_owned() + ":" + server_port))
    {
        Ok(r) => r,
        Err(e) => {
            println!("Unable to connect to server {:?}", e);
            return;
        }
    };

    let mut ds_for_ds = match downstream.try_clone()
    {
        Ok(r) => r,
        Err(e) => {
            println!("Unable to clone TcpStream {:?}", e);
            return;
        }
    };
    let mut us_for_ds = match upstream.try_clone()
    {
        Ok(r) => r,
        Err(e) => {
            println!("Unable to clone TcpStream {:?}", e);
            return;
        }
    };

    println!("Listening for input.");

    let downstream_thread = thread::spawn(move || { // downstream handler

        loop {

            match copy(&mut ds_for_ds, &mut us_for_ds)
            {
                Ok(r) => r as usize,

                Err(e) => {
                    println!("Couldn't copy contents of downstream buffer to buffer {:?}", e);
                    break;
                }
            };

        }
    });

    let mut ds_for_us = match downstream.try_clone()
    {
        Ok(r) => r,
        Err(e) => {
            println!("Unable to clone TcpStream {:?}", e);
            return;
        }
    };
    let mut us_for_us = match upstream.try_clone()
    {
        Ok(r) => r,
        Err(e) => {
            println!("Unable to clone TcpStream {:?}", e);
            return;
        }
    };

    let upstream_thread = thread::spawn(move || { // upstream handler

        loop {

            match copy(&mut us_for_us, &mut ds_for_us)
            {
                Ok(r) => r as usize,

                Err(e) => {
                    println!("Couldn't copy contents of upstream buffer to buffer {:?}", e);
                    break;
                }
            };

        }
    });


    downstream_thread.join().unwrap();
    upstream_thread.join().unwrap();

}
