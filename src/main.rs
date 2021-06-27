use std::fs;    //for file handling
use std::io::prelude::*; //read & write to stream
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); //binding host and port to listen for connection

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; //holding the read data, note that buffer is mutable so that it may handle any pr                                       -efetched data.
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n"; //reading raw bytes into the buffer

    if buffer.starts_with(get) {    //checking to see of root path is asked 
        let content1 = fs::read_to_string("index.html").unwrap(); //read the files to string

        let response1 = format!(                        //formatting response to render 
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            content1.len(),
            content1);
        
        stream.write(response1.as_bytes()).unwrap();
        stream.flush().unwrap();    //flush stops program until all bytes are written to connection
    }



   
    else {              //this block returns a 404 error when any endpoints other than "/" is tried to be accessed.
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

