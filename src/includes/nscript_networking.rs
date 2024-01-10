use crate::*;
use std::collections::HashMap;

use std::net::{TcpListener, TcpStream};

pub struct NscriptTcp{
    pub streammap: HashMap<String,TcpStream>,

    pub listenermap: HashMap<String,TcpListener>,
    pub streamidcounter: u64,

    pub listeneridcounter: u64,
}

impl NscriptTcp{
    pub fn new() -> NscriptTcp {
        NscriptTcp{
            streammap: HashMap::new(),
            listenermap: HashMap::new(),
            streamidcounter: 1,
            listeneridcounter: 1,
        }
    }

    pub fn listener(&mut self,ip:&str,port:&str) -> String{
        let  listener: TcpListener;

        listener = TcpListener::bind(format!("{}:{}", ip, port)).unwrap();
        println!("Server started at http://{}:{}",  ip, port);


        #[cfg(windows)]
        listener.set_nonblocking(true).expect("Cannot set non-blocking");
        #[cfg(not(windows))]
        let mut newid = String::new();
        match listener.set_nonblocking(true){
            Ok(_) => {
                let newi = self.listeneridcounter + 1;
                newid = "nc_listener_".to_owned() + &newi.to_string();
                self.listenermap.insert(newid.clone(), listener);
                println!("socked ok!");

            }
            Err(e) => {
                println!("Error on NscriptTcp listener, cant set nonblocking tcp code:{}",e);
            }
        };
        newid
    }
    pub fn accept(&mut self,id: &str)->String{
        let g = self.listenermap.get_key_value(id);
        let listener: &TcpListener;
        let mut newid = String::new();
        match g {
            None => {
                println!("listener error, does not exist!id={}",id);
                return newid;
            },
            Some((_i, k)) => listener = k,
        }
        match listener.accept() { // add the stream to the map
            Ok((stream, _)) => {
                let newi = self.streamidcounter + 1;
                newid = "nc_listener_".to_owned() + &newi.to_string();
                self.streammap.insert(newid.clone(), stream);
                println!("stream ok!");

            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No incoming connections yet,
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
        newid
    }
    pub fn connect(&mut self,ip: &str,ports:&str)->String{
        let stream: TcpStream;
        let mut newid = String::new();
        let mut port: u16 = 8888;
        if let Ok(port) = ports.to_owned().parse::<u16>() {
            println!("Parsed port number: {}", port);
            // Use the port number (port) here in your code
        } else {
            port = 8888;
            println!("Failed to parse port number");
            // Handle the case when parsing fails
        }
        match TcpStream::connect((ip, port)){
            Ok(e) => {
                let newi = self.streamidcounter + 1;
                newid = "nc_stream_".to_owned() + &newi.to_string();
                self.streammap.insert(newid.clone(), e);


            }
            Err(r) => {
                println!("Error:{}",r);
            }
        };
        newid
    }
    pub fn send(&mut self,id:&str,msg: &str)->String{
        let g = self.streammap.get_key_value(id);
        let mut stream: &TcpStream;
        match g {
            None => {
                println!("stream error, does not exist!id={}",id);
                return "error".to_owned();

            },
            Some((_i, k)) => {
                stream = k;
            },


        }
        stream.write(msg.as_bytes()).unwrap();
        return "ok".to_owned();
    }
    pub fn receive(&mut self,id:&str)->String{
        let g = self.streammap.get_key_value(id);
        let mut stream: &TcpStream;
        let mut response = String::new();
        match g {
            None => {
                println!("stream error, does not exist!id={}",id);
                return "".to_owned();

            },
            Some((_i, k)) => {
                stream = k;
            },


        }
        let mut buffer = [0; 1024];

        match stream.read(&mut buffer) {
            Ok(bytes_read) => {

                //println!("\nbytesRead!{}\n",bytes_read);

                response = "".to_owned() + &String::from_utf8_lossy(&buffer[0..bytes_read]);

                // procceed the connection.

            }
            Err(_) => {

                // handle OS error on connection-reset

            }
        }
            response
    }
}

