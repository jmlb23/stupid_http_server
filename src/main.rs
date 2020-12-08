use std::env::args;
use std::io::{BufReader, BufWriter};
use std::net::TcpStream;
use std::io::Write;
use std::fs::read_to_string;
use std::net::TcpListener;
use std::io::BufRead;

fn main() {
    let socket = TcpListener::bind("127.0.0.1:8080");

    let dir = args().skip(1)
                            .collect::<Vec<String>>()
                            .first()
                            .map(|x| String::from(x))
                            .unwrap_or("./example".to_string());

     match socket {
        Ok(tcp) => {
            tcp.incoming().for_each(|x| {
                match x {
                    Ok(stream) => {
                        let mut reader = BufReader::new(&stream);
                        let mut writer = BufWriter::new(&stream);
                        let mut  buffer = Vec::new();
                        let _ = reader.read_until(b'\n', &mut buffer);
                        let req_as_string = String::from_utf8(buffer).expect("Error parsing request");
                        let requested_path =parse_request(&req_as_string);
                        let content = read_html_file(requested_path.to_string(),&dir);
                        send_response(&mut writer, &content)
                    },
                    Err(e) => {
                        println!("{}",e)
                    },
                }
            });
        },
        Err(e) => {
            println!("{}",e)
        }
    }
}

fn read_html_file(name: String, dir: &String) -> String{
    let dir = format!("{}{}",dir,name);
    dbg!(&dir);
    return read_to_string(&dir).unwrap_or("Not Found".to_string());
}

fn parse_request(request: &String) -> &str {
    let splits = request.split(" ").collect::<Vec<&str>>();
    return splits[1];
}

fn send_response(stream: &mut BufWriter<&TcpStream>, content: &String) -> () {

    stream.write("HTTP/1.1 200 OK\r\n".as_bytes()).expect("Error");
    stream.write("Content-Type: text/html; charset=UTF-8\r\n".as_bytes()).expect("Error");
    stream.write(format!("Content-Length: {}\r\n\r\n", content.len()).as_bytes()).expect("Error");
    stream.write(content.as_bytes()).expect("Error");
    stream.flush().expect("Error flushing");

}