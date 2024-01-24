pub mod server_comm{
    pub use std:: {
        fs,
        io::{prelude::*, BufReader},
        net::{TcpListener, TcpStream},
    };

    pub fn server_run(listener: &TcpListener) {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let http_request = handle_request(&stream);
            handle_reponse(&stream, http_request);
        }  
    }

    //for unwrap, replace with error handling methods
    pub fn handle_request(stream: &TcpStream) -> String{
        let buf_reader = BufReader::new(stream);
        let http_request = buf_reader.lines().next().unwrap().unwrap();
        http_request
}

    pub fn handle_reponse(stream: &TcpStream, http_request:String) {  
         //here we check for acceptable conditions to respond
         if http_request == "GET / HTTP/1.1" {
            respond_good(stream, http_request);
        } else {
            //some other request or error
            respond_bad(stream, http_request)
        }
        
    }

    pub fn respond_good(mut stream: &TcpStream, http_request: String) {
        println!("Request: {:#?}", http_request); let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("Webpages/hello.html").unwrap();
        let length = contents.len();
    
        let response = format!
            ("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    }

    pub fn respond_bad(mut stream: &TcpStream, http_request: String) {
        println!("Request: {:#?}", http_request); let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("Webpages/goodbye.html").unwrap();
        let length = contents.len();
    
        let response = format!
            ("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    }
}



