use std::net::TcpStream;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::env;
//use std::io;
use std::process;
use crate::reversi::OthelloBoard;
mod reversi;


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

    let mut board = OthelloBoard::new();

    let mut turn: usize = 0;

    let mut score: Vec<usize> = vec![2,2];

    let mut counter: usize = 4;

    let mut mytime = Some(60000);

    let mut pre_mytime:usize = 60000;

    let mut n:usize = 7;

    let board90 = OthelloBoard::new();

    board90.print(1);

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
                                    n = 7;
                                
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
                                        bit = valid_board[65];
                                    }else{
                                        if let Some(second) = mytime{
                                            bit = board.eval_boards(turn,counter, second, pre_mytime, &mut n);
                                            pre_mytime = second;
                                        }else{
                                            bit = board.eval_boards(turn,counter,60000, 60000, &mut n);
                                        }
                                    }
                                    
                                    score[turn] += valid_board[bit] + 1;
                                    score[1-turn] -= valid_board[bit];
                                    counter += 1;
                    
                                    board.add(bit,&mut turn);

                                    let yoko = bit % 8;
                                    let tate = (bit / 8) + 1;
                                    let mapping = vec!["A","B","C","D","E","F","G","H"];

                                    let getc: Vec<String> = vec![(mapping[yoko].to_string()), tate.to_string()];
                                
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