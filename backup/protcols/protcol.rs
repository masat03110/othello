use std::net::TcpStream;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::env;
use std::io;
use std::process;

enum State {
    WaitMatch,
    WaitAck,
    MyTurn,
    OppositeTurn,
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 6 {
        println!("Usage: ./protocol -H <hostname> -p <port> -n <name>");
        return;
    }

    let mut hostname = "";
    let mut port = "";
    let mut name = "";

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-H" => {
                hostname = &args[i + 1];
                i += 2;
            }
            "-p" => {
                port = &args[i + 1];
                i += 2;
            }
            "-n" => {
                name = &args[i + 1];
                i += 2;
            }
            _ => {
                println!("Invalid option: {}", args[i]);
                return;
            }
        }
    }

    if hostname.is_empty() || port.is_empty() {
        println!("Hostname and port are required");
        return;
    }

    // サーバーに接続するためのTcpStreamを作成
    let address = format!("{}:{}", hostname, port);
    let open_message = format!("OPEN {}\n",name);
    let mut stream = TcpStream::connect(address.clone()).expect("Failed to connect to server");
    stream.write_all(open_message.as_bytes()).expect("Failed to send data to server");

    print!("Sent: {}",open_message);

    let mut status: State = State::WaitMatch;

    loop {
        let reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        for line in reader.lines(){
            match line{
                Ok(string) => 
                {
                    loop{
                        match status{
                            State::WaitMatch => {
                                println!("Receved: {}",string);
                                let parts: Vec<&str> = string.split(" ").collect();
                                let Some(command) = parts.get(0) else { todo!() };

                                if *command == "BYE"{
                                    process::exit(0);
                                }
                                else if *command == "START"{
                                
                                    let Some(color) = parts.get(1) else { todo!() };

                                    if *color == "BLACK" {
                                        status = State::MyTurn;
                                    }else{// "WHITE"
                                        status = State::OppositeTurn;
                                        break;
                                    }
                                }else{
                                    panic!("Invalid command at wait_mutch");
                                }
                            }
                            State::WaitAck => {
                                println!("Receved: {}",string);
                                let parts: Vec<&str> = string.split(" ").collect();
                            
                                let Some(command) = parts.get(0) else { todo!()};
                            
                                if *command == "ACK"{
                                    status = State::OppositeTurn;
                                    break;
                                }
                                else if *command == "END"{
                                    status = State::WaitMatch;
                                    break;
                                }else{
                                    panic!("Invalid command at wait_mutch");
                                }
                            }
                            State::MyTurn => {
                                let mut input = String::new();

                                io::stdin().read_line(&mut input)
                                .expect("Failed to read line");
                            
                                let chars: Vec<char> = input
                                    .trim()
                                    .chars()
                                    .collect();
                            
                                let move_message = format!("MOVE {}{}\n",chars[0],chars[1]);
                                writer.write(move_message.as_bytes()).expect("Failed to send data to server");
                                writer.flush().unwrap();
                        
                                print!("Sent: {}",move_message);
                                status = State::WaitAck;
                                break;
                            }
                            State::OppositeTurn => {
                                println!("Receved: {}",string);
                                let parts: Vec<&str> = string.split(" ").collect();
                            
                                let Some(command) = parts.get(0) else { todo!() };
                            
                                if *command == "END"{
                                    status = State::WaitMatch;
                                    break;
                                }
                                else if *command == "MOVE"{
                                    //let Some(number) = parts.get(1) else { todo!() };
                                    status = State::MyTurn;
                                }else{
                                    panic!("Invalid command at wait_mutch");
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading line: {}", e);
                    break;
                }
            }
        }
    }
}