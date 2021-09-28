use std::{
    io::{Read, Write},
    net::TcpListener,
    thread::spawn,
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                spawn(move || {
                    fun_name(s);
                });
            }
            Err(_) => todo!(),
        }
    }
}

fn fun_name(mut s: std::net::TcpStream) {
    let mut data = [0 as u8; 50];
    println!("Connected!");
    while match s.read(&mut data[0..50]) {
        Ok(size) => {
            s.write(&data[0..size]).unwrap();
            true
        }
        Err(_) => {
            println!("Errors occured");
            false
        }
    } {}
}
