use std::net::TcpStream;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::env;
//use std::io;
use std::process;
use crate::reversi::OthelloBoard;
mod reversi;
use std::collections::HashSet;


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

    //established tactic

    let zhoseki: HashSet<&str> = [
        "F5D6",
        "F5D6C3",
        "F5D6C3D3C4",
        "F5D6C3D3C4B5",
        "F5D6C3D3C4B3",
        "F5D6C3D3C4B3D2F4",
        "F5D6C3D3C4F4E3",
        "F5D6C3D3C4F4E3F3E6B4",
        "F5D6C3D3C4F4E3F3E6C6",
        "F5D6C3D3C4B3D7",
        "F5D6C3D3C4B3C6B6",
        "F5D6C3D3C4F4E3F3E6F6",
        "F5D6C3D3C4F4E3F3G4",
        "F5D6C3D3C4F4E6",
        "F5D6C3D3C4F4E6F6",
        "F5D6C3D3C4F4E6F6E3C5C6B6",
        "F5D6C3D3C4F4E6B3C2",
        "F5D6C3D3C4F4E6B3D2",
        "F5D6C3D3C4F4E6B3E2",
        "F5D6C3D3C4F4D7",
        "F5D6C3D3C4F4F6",
        "F5D6C3D3C4F4F6F3",
        "F5D6C3D3C4F4F6F3G4",
        "F5D6C3D3C4F4F6F3G4G3C5",
        "F5D6C3D3C4F4F6F3E3",
        "F5D6C3D3C4F4F6F3E3G5",
        "F5D6C3D3C4F4F6F3E6E7F7",
        "F5D6C3D3C4F4F6F3E6E7F7C5B6G6G5C6E8B5D2",
        "F5D6C3D3C4F4F6F3E6E7F7C5B6B4",
        "F5D6C3D3C4F4F6F3E6E7C6",
        "F5D6C3D3C4F4F6F3E6E7C6G6G5F7G3",
        "F5D6C3D3C4F4F6F3E6E7D7B3E3E2",
        "F5D6C3D3C4F4F6F3E6E7D7G6F7",
        "F5D6C3D3C4F4F6F3E6E7D7G6D8",
        "F5D6C3D3C4F4F6F3E6E7D7G6G5",
        "F5D6C3D3C4F4F6F3E6E7D7G6G5C5B6",
        "F5D6C3D3C4F4F6F3E6E7D7G6G5C5C6",
        "F5D6C3D3C4F4F6F3E6E7D7G6F8",
        "F5D6C3D3C4F4F6F3E6E7D7G6F8F7H6",
        "F5D6C3D3C4F4F6F3E6E7D7G6F8F7G5H6H4E8",
        "F5D6C3D3C4F4F6F3E6E7D7G6F8C5",
        "F5D6C3D3C4F4F6F3E6E7D7G6F8F7G5H6H4G4H3H5H7",
        "F5D6C3D3C4F4F6F3E6E7D7G6F8F7E2",
        "F5D6C3D3C4F4F6G5",
        "F5D6C3D3C4F4F6G5E3",
        "F5D6C3D3C4F4F6G5E3F3G4H3G3F2",
        "F5D6C3D3C4F4F6G5E3F3G6",
        "F5D6C3D3C4F4F6G5E3F3E6",
        "F5D6C3D3C4F4F6G5E6C5",
        "F5D6C3D3C4F4F6G5C6",
        "F5D6C3D3C4F4F6G5E6D7C7",
        "F5D6C3D3C4F4F6G5E6D7E7",
        "F5D6C3D3C4F4F6G5E6D7F7",
        "F5D6C3D3C4F4F6G5E6D7E3C5F3E7",
        "F5D6C3D3C4F4F6G5E6F7",
        "F5D6C3D3C4F4F6G5F3",
        "F5D6C3D3C4F4F6B4",
        "F5D6C3D3C4F4F6B4C2",
        "F5D6C3D3C4F4F6B4C2E2",
        "F5D6C3D3C4F4F6B4C2F3",
        "F5D6C3D3C4F4F6B4F3E6E3G5",
        "F5D6C3D3C4F4C5",
        "F5D6C3D3C4F4C5B4",
        "F5D6C3D3C4F4C5B4B3C6",
        "F5D6C3D3C4F4C5B4B5C6F3E6E3",
        "F5D6C3D3C4F4C5B4B5C6F3E6E3G6F6G5D7G3",
        "F5D6C3D3C4F4C5B4B3B5",
        "F5D6C3D3C4F4C5B4B3E6",
        "F5D6C3D3C4F4C5B5",
        "F5D6C3D3C4F4C5B3E2",
        "F5D6C3D3C4F4C5B3D2",
        "F5D6C3D3C4F4C5B3C2",
        "F5D6C3D3C4F4C5B3C2B4",
        "F5D6C3D3C4F4C5B3C2B4E3E2",
        "F5D6C3D3C4F4C5B3C2B4E3E2A5D1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7D8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7",
        "F5D6C3D3C4F4C5B3C2B4C6",
        "F5D6C3D3C4F4C5B3C2E6",
        "F5D6C3D3C4F4C5B3C2E6B4",
        "F5D6C3D3C4F4C5B3C2E6B5",
        "F5D6C3D3C4F4C5B3C2E6C6B4A5",
        "F5D6C3D3C4F4C5B3C2E6C6B6",
        "F5D6C3D3C4F4C5B3C2E6C6B4B5F6",
        "F5D6C3D3C4F4C5B3C2E6C6B4B5D2E3A6C1D7",
        "F5D6C3D3C4F4C5B3C2E6C6B4B5D2E3A6C1B6",
        "F5D6C3D3C4F4C5B3C2E6C6B4B5D2E3A6C7",
        "F5D6C3D3C4F4C5B3C2E6C6B4B5D2E3A6F3",
        "F5D6C3D3C4F4C5B3C2E3",
        "F5D6C3D3C4F4C5B3C2E3D2C6B4A3",
        "F5D6C3D3C4F4C5B3C2E3D2C6B4A3G4",
        "F5D6C3D3C4F4C5B3C2E3D2C6B4B5",
        "F5D6C3D3C4F4C5B3C2E3D2C6B4A4",
        "F5D6C3D3C4F4C5B3C2E3D2C6B4F6",
        "F5D6C3D3C4F4C5B3C2E3D2C6B4E6",
        "F5D6C3D3C4F4C5B3C2E3D2B6",
        "F5D6C3D3C4F4C5B3C2E3D2B4",
        "F5D6C3D3C4F4C5B3C2E3D2E1",
        "F5D6C3D3C4F4C5B3C2D1",
        "F5D6C3D3C4F4C5B3C2D2",
        "F5D6C3D3C4F4C5B3C2F6",
        "F5D6C3D3C4F4C5B3C2F6A3B5B6E3F2B2",
        "F5D6C3F4",
        "F5D6C3F4F6D3F3B3",
        "F5D6C3F4F6D3F3B3C7",
        "F5D6C3F4F6D3C6",
        "F5D6C3F4F6D3F3G4",
        "F5D6C3F4F6C4E3F3F2E6C6C5B4",
        "F5D6C3F4F6F3F2E6C6",
        "F5D6C3F4F6D3E3F3F2E6D7",
        "F5D6C3G5",
        "F5D6C3D3C5",
        "F5D6C3D3C5F4E3C4F3C6C7",
        "F5D6C3D3C5G6",
        "F5D6C3D3C6",
        "F5D6C3F3",
        "F5D6C4",
        "F5D6C4D3C5",
        "F5D6C4D3C5F4E3F3C2C6",
        "F5D6C4D3C5F4D7",
        "F5D6C4D3C5F4D2F6D7",
        "F5D6C4D3C5F4E3F3C2B4B3",
        "F5D6C4D3C5F4E2",
        "F5D6C4D3C5F4E3F3E2",
        "F5D6C4D3C5F4E3F3E2C6E6F6D7C8E7",
        "F5D6C4D3C5B4",
        "F5D6C4G5",
        "F5D6C4G5C6",
        "F5D6C4G5C6C5D7D3B4C3E3F3",
        "F5D6C4G5C6C5E6",
        "F5D6C4G5C6C5F6",
        "F5D6C4G5C6C5B6",
        "F5D6C4G5F6",
        "F5D6C4G5E6D3C5B5C6D7",
        "F5D6C4D3E6",
        "F5D6C4D3E6F4E3F3C6F6G5G6E7H6",
        "F5D6C4B3",
        "F5D6C4B3B4G5C6D3",
        "F5D6C4F4",
        "F5D6C5",
        "F5D6C5B4",
        "F5D6C5B6",
        "F5D6C5F4E3",
        "F5D6C5F4E3F6",
        "F5D6C5F4E3G6",
        "F5D6C5F4E3G5",
        "F5D6C5F4E3D3",
        "F5D6C5F4E3C4",
        "F5D6C5F4E3C4E6",
        "F5D6C5F4E3C6E6",
        "F5D6C5F4E3C6E6F6",
        "F5D6C5F4E3C6E6F7",
        "F5D6C5F4E3C6E6F7D7E8F3F6",
        "F5D6C5F4E3C6E6B4",
        "F5D6C5F4E3C6D7",
        "F5D6C5F4E3C6F3",
        "F5D6C5F4E3C6D3G5",
        "F5D6C5F4E3C6D3G5E6F6G3C4B4B3B5A4A2A3A5E2D2D1C1",
        "F5D6C5F4E3C6D3E2",
        "F5D6C5F4E3C6D3F3",
        "F5D6C5F4E3C6D3F3E6F7G4C3",
        "F5D6C5F4E3C6D3F6E6D7",
        "F5D6C5F4E3C6D3F6E6D7G3",
        "F5D6C5F4E3C6D3F6E6D7G3C4",
        "F5D6C5F4E3C6D3F6E6D7G3C4B4",
        "F5D6C5F4E3C6D3F6E6D7G3C4B4B3G5C3B5A5A4A3B6A6D8",
        "F5D6C5F4E3C6D3F6E6D7G3C4G5",
        "F5D6C5F4E3C6D3F6E6D7G3C4G5C3B4E2",
        "F5D6C5F4E3C6D3F6E6D7G3C4G6",
        "F5D6C5F4E3C6D3F6E6D7G3C4E7",
        "F5D6C5F4E3C6D3F6E6D7G4",
        "F5D6C5F4E3C6D3F6E6D7G4G3",
        "F5D6C5F4E3C6D3F6E6D7G4C4",
        "F5D6C5F4E3C6D3F6E6D7G4C4G6",
        "F5D6C5F4E3C6D3F6E6D7G4C4B4",
        "F5D6C5F4E3C6D3F6E6D7G4C4E7",
        "F5D6C5F4E3C6D3F6E6D7G4C4G5C3F7D2E7F2",
        "F5D6C5F4E3C6D3F6E6D7G4C4G5C3F7D2E7F2E2",
        "F5D6C5F4E3C6D3F6E6D7G4C4G5C3F7D2E7F2E2F3C8G3F1H4H5H6G6D8H3H2B4A4B3",
        "F5D6C5F4E3C6D3F6E6D7G4C4G5C3F7D2E7F2E2F3C8G3F1H4H5H6G6D8H3H2B4A4C2",
        "F5D6C5F4E3C6D3F6E6D7G4C4G5C3F7D2E7F2E2F3C8E1C7D8E8G3H2",
        "F5D6C5F4E3C6D3F6E6D7G4C4G5C3F7D2E7F2E2F3C8G3F1H4H5H6G6E1",
        "F5D6C5F4E3C6D3F6E6D7G4C4G5C3F7D2E7F2C8F3C7D8E8G3H2",
        "F5D6C5F4E3C6D3F6E6D7G4C4G5C3B4",
        "F5D6C5F4E3C6D3F6E6D7G4C4G5C3F7D2C1",
        "F5D6C5F4E3C6D3F6E6D7G4C4G5C3F7D2C1E2B5G3F3H6H4",
        "F5D6C5F4E3C6D3F6E6D7E7",
        "F5D6C5F4E3C6D3F6E6F7",
        "F5D6C5F4E3C6D3F6E6C3",
        "F5D6C5F4D3",
        "F5D6C5F4D3E3F6E6C6C4B3C3F3",
        "F5D6C5F4D3E3G4G5F2",
        "F5D6C5F4F3",
        "F5D6C5F4D7",
        "F5D6C5F4E7F6G5E6E3",
        "F5D6C6",
        "F5D6C7",
        "F5D6C7F3C3C4E3D3C5F4G3",
        "F5D6C7F3C3C4E3F4C5D3G3",
        "F5F6",
        "F5F6D3F4",
        "F5F6C4F4",
        "F5F6E6F4E3",
        "F5F6E6F4E3C5C4",
        "F5F6E6F4E3C5C4E7",
        "F5F6E6F4E3C5C4E7G4",
        "F5F6E6F4E3C5C4E7G4G3D7H4F7",
        "F5F6E6F4E3C5C4E7C6E2",
        "F5F6E6F4E3C5C4E7C6E2F7D7F3",
        "F5F6E6F4E3C5C4E7C6D2",
        "F5F6E6F4E3C5C4E7B5E2",
        "F5F6E6F4E3C5C4D7",
        "F5F6E6F4E3C5C4D2",
        "F5F6E6F4E3C5C4D6",
        "F5F6E6F4E3C5C6D6E7",
        "F5F6E6F4E3C5C6D6E7F7C3C7",
        "F5F6E6F4E3C5C6D6E7F7C3C7G3",
        "F5F6E6F4E3C5C6D6E7F7C3C7G3F8E8F2",
        "F5F6E6F4E3C5C6D6E7F7C3C7G4",
        "F5F6E6F4E3C5C6D6E7F7C3C7D7",
        "F5F6E6F4E3C5C6D6E7D7C3C7G3E2F7D3C8B3",
        "F5F6E6F4E3C5C4D3C3",
        "F5F6E6F4E3C5C4D3F3E2",
        "F5F6E6F4E3D6",
        "F5F6E6F4E3D6E7",
        "F5F6E6F4E3D6C4E2D7",
        "F5F6E6F4E3D7",
        "F5F6E6F4E3D7G4H3",
        "F5F6E6F4E3D7G4F3G6E2",
        "F5F6E6F4E3C5G5",
        "F5F6E6F4E3C5G5D6G6",
        "F5F6E6F4E3C5G5G3G4F3C4",
        "F5F6E6F4E3C5G5G3G4F3G6",
        "F5F6E6F4E3C5G6",
        "F5F6E6F4E3C5G6F3G5G3D3E2F1",
        "F5F6E6F4E3D3",
        "F5F6E6F4G5",
        "F5F6E6F4G5D6",
        "F5F6E6F4G5D6F3G6C7",
        "F5F6E6F4G5G6",
        "F5F6E6F4G5G6G4E7F7F8H6H4H5H7D8D6",
        "F5F6E6F4G5G6G4E7F7H6H4H5H7F8D8D6",
        "F5F6E6F4G5C6",
        "F5F6E6F4G5E7F7D6",
        "F5F6E6F4G5E7F7C5F3",
        "F5F6E6F4G5E7F7H5E8C5",
        "F5F6E6F4G5E7F7H5",
        "F5F6E6F4G5E7D7",
        "F5F6E6F4G5E7E3",
        "F5F6E6F4G5E7E3F3D3C3G2",
        "F5F6E6F4G5E7E3F3C5H5",
        "F5F6E6F4G5E7E3F3C5D3",
        "F5F6E6F4G5E7E3F3C5H6",
        "F5F6E6F4G5E7E3G6D6C4F7",
        "F5F6E6F4G5E7E3G4",
        "F5F6E6F4G5E7F7C5E3F3C4D3C2",
        "F5F6E6F4G6",
        "F5F6E6F4G6D6",
        "F5F6E6F4G6D6G4F7E8F8G8H6H5H3H7",
        "F5F6E6F4G6C6",
        "F5F6E6F4G6C5F3",
        "F5F6E6F4G6C5G4G5H4",
        "F5F6E6F4G6C5G4G5F3E3C4",
        "F5F6E6F4G6C6G4D6C4C5C7",
        "F5F6E6F4G6F7",
        "F5F6E6F4C3",
        "F5F6E6F4C3C4",
        "F5F6E6F4C3D6F3C4C5B4",
        "F5F6E6F4C3D6F3C4C5B4A5A3C6B5A6B6",
        "F5F6E6F4C3D7F3",
        "F5F6E6F4G4",
        "F5F6E6F4G4E7F7G5E3D3H5C6",
        "F5F6E6F4F3",
        "F5F6E6F4F3C5F7",
        "F5F6E6F4D3",
        "F5F6E6D6C7",
        "F5F6E6F4G3F3G4",
        "F5F6E6F4G3D6",
        "F5F4",
        "F5F4D3",
        "F5F4F3",
        "F5F4E3F6C4E2",
        "F5F4E3F2",
        "F5F4E3F6D3E2F3F2",
        "F5F4E3F6D3",
        "F5F4E3F6D3C3C4C5D6F3E6C6",
        "F5F4E3F6D3C3C4F3E6C6D6C5G6H6"
        ].iter().cloned().collect();

    // サーバーに接続するためのTcpStreamを作成
    let address = format!("{}:{}", hostname, port);
    let open_message = format!("OPEN {}\n",name);
    let mut stream = TcpStream::connect(address.clone()).expect("Failed to connect to server");
    stream.write_all(open_message.as_bytes()).expect("Failed to send data to server");

    print!("Sent: {}",open_message);

    let mut status: State = State::WaitMatch;

    let mut board = OthelloBoard::new();

    let mut turn: usize = 0;

    let mut score: Vec<usize> = vec![2,2];

    let mut counter: usize = 4;

    let mut mytime = Some(60000);

    let mut pre_mytime:usize = 60000;

    let mut n:usize = 6;

    let board90 = OthelloBoard::new();

    board90.print(1);

    let mut tezyun: Vec<char> = vec![];

    let mut use_zhoseki = 0;

    let mut zyoseki_mode = 0;

    loop {
        let reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        for line in reader.lines(){
            match line{
                Ok(string) => 
                {
                    println!("Receved: {}",string);
                    let parts: Vec<&str> = string.split(" ").collect();
                    let Some(command) = parts.get(0) else { todo!() };

                    loop{
                        match status{
                            State::WaitMatch => {

                                if *command == "BYE"{
                                    process::exit(0);
                                }
                                else if *command == "START"{
                                    n = 6;

                                    tezyun = vec![];

                                    use_zhoseki = 0;
                                
                                    let Some(color) = parts.get(1) else { todo!() };

                                    board = OthelloBoard::new();

                                    turn = 0;
                                
                                    score = vec![2,2];
                                
                                    counter = 4;

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

                                if *command == "ACK"{
                                    mytime = parts.get(1).and_then(|s| s.parse::<usize>().ok());

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

                                let valid_board = board.valid_pieces(turn);

                                if *command == "MOVE"{
                                    let Some(command2) = parts.get(1) else { todo!() };

                                    if *command2 != "PASS"{

                                        let chars: Vec<char> = (*command2)
                                            .trim()
                                            .chars()
                                            .collect();

                                        if counter < 5{
                                            if chars[0] == 'E' && chars[1] == '6'{
                                                zyoseki_mode = 1;
                                            }else if chars[0] == 'D' && chars[1] == '3'{
                                                zyoseki_mode = 2;
                                            }else if chars[0] == 'C' && chars[1] == '4'{
                                                zyoseki_mode = 3;
                                            }
                                            tezyun = vec!['F','5'];
                                        }else if use_zhoseki == 0{
                                            let henkan: Vec<char>;
                                            let input2 = chars.clone();

                                            if zyoseki_mode == 1{
                                                henkan = match (input2).as_slice() {
                                                    [ch @ 'A'..= 'H', num @ '1'..='8'] => vec![(*num as u8 - b'1' + b'A') as char, (*ch as u8 - b'A' + b'1') as char],
                                                    _ => panic!("Invalid input format"),
                                                };
                                        
                                            } else if zyoseki_mode == 2{
                                        
                                                henkan = match (input2).as_slice() {
                                                    [ch @ 'A'..= 'H', num @ '1'..='8'] => vec![(b'8' - *num as u8 + b'A') as char, (b'H' - *ch as u8 + b'1') as char],
                                                    _ => panic!("Invalid input format"),
                                                };
                                                
                                            }else if zyoseki_mode == 3{
                                        
                                                henkan = match (input2).as_slice() {
                                                    [ch @ 'A'..= 'H', num @ '1'..='8'] => vec![(b'H' - *ch as u8  + b'A') as char, (b'8' - *num as u8 + b'1') as char],
                                                    _ => panic!("Invalid input format"),
                                                };
                                            }else{
                                                henkan = input2;
                                            }
                                            tezyun.extend(henkan);
                                        }

                                        let getf: Vec<usize> = match (chars).as_slice() {
                                            [ch @ 'A'..= 'H', num @ '1'..='8'] => vec![(*ch as u8 - b'A') as usize, (*num as u8 - b'1') as usize],
                                            _ => panic!("Invalid input format"),
                                        };
                                        let bit = getf[0] + getf[1]*8;

                                        score[turn] += valid_board[bit] + 1;
                                        score[1-turn] -= valid_board[bit];
                                        counter += 1;
                    
                                        board.add(bit,&mut turn);
                                    }else{
                                        turn = 1 - turn;
                                    }
                                    
                                }

                                let valid_board = board.valid_pieces(turn);

                                board.print(turn);


                                println!("(turn,B,W):({},{},{})",counter,score[0],score[1]);

                                if turn == 0{
                                    println!("B's turn");
                                }else{
                                    println!("W's turn");
                                }

                                let move_message: String;


                                if valid_board[64] == 0{
                                    turn = 1 - turn;
                                    move_message = format!("MOVE PASS\n");

                                    // writer.write(pass_message.as_bytes()).expect("Failed to send data to server");
                                    // writer.flush().unwrap();
                                    // print!("Sent: {}",pass_message);

                                }
                                else{

                                    // let mut input = String::new();

                                    // io::stdin().read_line(&mut input)
                                    // .expect("Failed to read line");
                                
                                    // let chars: Vec<char> = input
                                    //     .trim()
                                    //     .chars()
                                    //     .collect();

                                    // let getf: Vec<usize> = match chars.as_slice() {
                                    //     [ch @ 'A'..= 'H', num @ '1'..='8'] => vec![(*ch as u8 - b'A') as usize, (*num as u8 - b'1') as usize],
                                    //     _ => panic!("Invalid input format"),
                                    // };
                    
                                    // let bit = getf[0] + getf[1]*8;
                                    let bit:usize;

                                    
                                    if counter < 5{
                                        bit = 37;
                                        tezyun.extend(['F','5']);
                                    }else{
                                        if use_zhoseki != 0{
                                            if let Some(second) = mytime{
                                                bit = board.eval_boards(turn,counter, second, pre_mytime, &mut n);
                                                pre_mytime = second;
                                            }else{
                                                bit = board.eval_boards(turn,counter,60000, 60000, &mut n);
                                            }
                                        }else{
                                            let check: String = tezyun.clone().into_iter().collect();
                                            let mut matches: Vec<&str> = zhoseki
                                                            .iter()
                                                            .filter(|item| item.starts_with(&check))
                                                            .cloned()
                                                            .collect();
                                            
                                            let length = check.len();
                                            matches.sort_by_key(|s| s.len());

                                            if matches.is_empty() || (matches.len() == 1 && matches.pop().unwrap().len() <= length){
                                                use_zhoseki = 1;
                                                if let Some(second) = mytime{
                                                    bit = board.eval_boards(turn,counter, second, pre_mytime, &mut n);
                                                    pre_mytime = second;
                                                }else{
                                                    bit = board.eval_boards(turn,counter,60000, 60000, &mut n);
                                                }
                                            }else{
                                                let utite = matches.pop().unwrap();
                                                let input2: Vec<char> = utite.chars().collect();
                                                let input3 = vec![input2[length],input2[length+1]];
                                                tezyun.extend(input3.clone());

                                                let te;
                                                if zyoseki_mode == 1{

                                                    te = match (input3).as_slice() {
                                                        [ch @ 'A'..= 'H', num @ '1'..='8'] => vec![(*num as u8 - b'1' + b'A') as char, (*ch as u8 - b'A' + b'1') as char],
                                                        _ => panic!("Invalid input format"),
                                                    };
                                            
                                                } else if zyoseki_mode == 2{
                                            
                                                    te = match (input3).as_slice() {
                                                        [ch @ 'A'..= 'H', num @ '1'..='8'] => vec![(b'8' - *num as u8 + b'A') as char, (b'H' - *ch as u8 + b'1') as char],
                                                        _ => panic!("Invalid input format"),
                                                    };
                                                    
                                                }else if zyoseki_mode == 3{
                                            
                                                    te = match (input3).as_slice() {
                                                        [ch @ 'A'..= 'H', num @ '1'..='8'] => vec![(b'H' - *ch as u8  + b'A') as char, (b'8' - *num as u8 + b'1') as char],
                                                        _ => panic!("Invalid input format"),
                                                    };
                                                }else{
                                                    te = input3;
                                                }

                                                let getf: Vec<usize> = match (te).as_slice() {
                                                    [ch @ 'A'..= 'H', num @ '1'..='8'] => vec![(*ch as u8 - b'A') as usize, (*num as u8 - b'1') as usize],
                                                    _ => panic!("Invalid input format"),
                                                };
                                                bit = getf[0] + getf[1]*8;

                                            }
                                        }
                                    }
                                    
                                    score[turn] += valid_board[bit] + 1;
                                    score[1-turn] -= valid_board[bit];
                                    counter += 1;
                    
                                    board.add(bit,&mut turn);

                                    let yoko = bit % 8;
                                    let tate = (bit / 8) + 1;
                                    let mapping = vec!['A','B','C','D','E','F','G','H'];

                                    let getc: Vec<char> = vec![mapping[yoko], (tate as u8 + 48) as char];
                                
                                    move_message = format!("MOVE {}{}\n",getc[0],getc[1]);
                                
                                }

                                writer.write(move_message.as_bytes()).expect("Failed to send data to server");
                                writer.flush().unwrap();
                                println!("Sent: {}",move_message);
                                status = State::WaitAck;
                                break;
                            }
                            State::OppositeTurn => {
                            
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