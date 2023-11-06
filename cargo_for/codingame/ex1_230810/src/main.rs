use std::collections::HashMap;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}


/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/

 use std::time::{Instant, Duration};

 // ビットボードの定義
 type Bitboard = u64;
 
 
 // the number of pieces
 fn count_pieces(n: u64) -> isize {
     let v :i32;
 
     unsafe {
         v = core::arch::x86_64::_popcnt64(n as i64) as i32;
     }
 
     return v as isize;
 }
 
 // オセロ盤の状態を表す構造体
 pub struct OthelloBoard {
     black: Bitboard,   // 黒石の位置を表すビットボード
     white: Bitboard,   // 白石の位置を表すビットボード
 }

 impl Clone for OthelloBoard {
    fn clone(&self) -> OthelloBoard {
        OthelloBoard { black: self.black, white: self.white } 
}}
 
 fn check_right(myboard:u64, opponent:u64, position: u64) -> u64{
 
     let mask = opponent & 0x7e7e7e7e7e7e7e7e;
     let mut result = mask & (position << 1);
     result |= mask & (result << 1);
     result |= mask & (result << 1);
     result |= mask & (result << 1);
     result |= mask & (result << 1);
     result |= mask & (result << 1);
     let mut mypiece:u64 = (result << 1) & myboard;
     mypiece |= mypiece >> 1;
     mypiece |= mypiece >> 2;
     mypiece |= mypiece >> 4;
 
     result &= mypiece;
     result
 }
 
 fn check_left(myboard:u64, opponent:u64, position: u64) -> u64{
 
     let mask = opponent & 0x7e7e7e7e7e7e7e7e;
     let mut result = mask & (position >> 1);
     result |= mask & (result >> 1);
     result |= mask & (result >> 1);
     result |= mask & (result >> 1);
     result |= mask & (result >> 1);
     result |= mask & (result >> 1);
     let mut mypiece:u64 = (result >> 1) & myboard;
     mypiece |= mypiece << 1;
     mypiece |= mypiece << 2;
     mypiece |= mypiece << 4;
 
     result &= mypiece;
     result
 }
 
 fn check_up(myboard:u64, opponent:u64, position: u64) -> u64{
 
     let mask = opponent & 0x00FFFFFFFFFFFF00;
     let mut result = mask & (position >> 8);
     result |= mask & (result >> 8);
     result |= mask & (result >> 8);
     result |= mask & (result >> 8);
     result |= mask & (result >> 8);
     result |= mask & (result >> 8);
     let mut mypiece:u64 = (result >> 8) & myboard;
     mypiece |= mypiece << 8;
     mypiece |= mypiece << 16;
     mypiece |= mypiece << 32;
 
     result &= mypiece;
     result
 }
 
 fn check_down(myboard:u64, opponent:u64, position: u64) -> u64{
 
     let mask = opponent & 0x00FFFFFFFFFFFF00;
     let mut result = mask & (position << 8);
     result |= mask & (result << 8);
     result |= mask & (result << 8);
     result |= mask & (result << 8);
     result |= mask & (result << 8);
     result |= mask & (result << 8);
     let mut mypiece:u64 = (result << 8) & myboard;
     mypiece |= mypiece >> 8;
     mypiece |= mypiece >> 16;
     mypiece |= mypiece >> 32;
 
     result &= mypiece;
     result
 }
 
 fn check_diagonally_upright(myboard:u64, opponent:u64, position: u64) -> u64{
 
     let mask = opponent & 0x007E7E7E7E7E7E00;
     let mut result = mask & (position >> 7);
     result |= mask & (result >> 7);
     result |= mask & (result >> 7);
     result |= mask & (result >> 7);
     result |= mask & (result >> 7);
     result |= mask & (result >> 7);
     let mut mypiece:u64 = (result >> 7) & myboard;
     mypiece |= mypiece << 7;
     mypiece |= mypiece << 14;
     mypiece |= mypiece << 28 ;
 
     result &= mypiece;
     result
 }
 
 fn check_diagonally_upleft(myboard:u64, opponent:u64, position: u64) -> u64{
 
     let mask = opponent & 0x007E7E7E7E7E7E00;
     let mut result = mask & (position >> 9);
     result |= mask & (result >> 9);
     result |= mask & (result >> 9);
     result |= mask & (result >> 9);
     result |= mask & (result >> 9);
     result |= mask & (result >> 9);
     let mut mypiece:u64 = (result >> 9) & myboard;
     mypiece |= mypiece << 9;
     mypiece |= mypiece << 18;
     mypiece |= mypiece << 36;
 
     result &= mypiece;
     result
 }
 
 fn check_diagonally_downright(myboard:u64, opponent:u64, position: u64) -> u64{
 
     let mask = opponent & 0x007E7E7E7E7E7E00;
     let mut result = mask & (position << 9);
     result |= mask & (result << 9);
     result |= mask & (result << 9);
     result |= mask & (result << 9);
     result |= mask & (result << 9);
     result |= mask & (result << 9);
     let mut mypiece:u64 = (result << 9) & myboard;
     mypiece |= mypiece >> 9;
     mypiece |= mypiece >> 18;
     mypiece |= mypiece >> 36;
 
     result &= mypiece;
     result
 }
 
 fn check_diagonally_downleft(myboard:u64, opponent:u64, position: u64) -> u64{
 
     let mask = opponent & 0x007E7E7E7E7E7E00;
     let mut result = mask & (position << 7);
     result |= mask & (result << 7);
     result |= mask & (result << 7);
     result |= mask & (result << 7);
     result |= mask & (result << 7);
     result |= mask & (result << 7);
     let mut mypiece:u64 = (result << 7) & myboard;
     mypiece |= mypiece >> 7;
     mypiece |= mypiece >> 14;
     mypiece |= mypiece >> 28;
 
     result &= mypiece;
     result
 }
 
 fn valids(myboard:u64, opponent:u64) -> u64{
     let akikoma = !(myboard | opponent);
     let mut mask = opponent & 0x7e7e7e7e7e7e7e7e;
     let mut valids:u64 = 0;
     let mut result = mask & (myboard << 1);
     result |= mask & (result << 1);
     result |= mask & (result << 1);
     result |= mask & (result << 1);
     result |= mask & (result << 1);
     result |= mask & (result << 1);
     valids |= result << 1;
 
     result = mask & (myboard >> 1);
     result |= mask & (result >> 1);
     result |= mask & (result >> 1);
     result |= mask & (result >> 1);
     result |= mask & (result >> 1);
     result |= mask & (result >> 1);
     valids |= result >> 1;
 
 
     mask = opponent & 0x00FFFFFFFFFFFF00;
     result = mask & (myboard >> 8);
     result |= mask & (result >> 8);
     result |= mask & (result >> 8);
     result |= mask & (result >> 8);
     result |= mask & (result >> 8);
     result |= mask & (result >> 8);
     valids |= result >> 8;
 
     result = mask & (myboard << 8);
     result |= mask & (result << 8);
     result |= mask & (result << 8);
     result |= mask & (result << 8);
     result |= mask & (result << 8);
     result |= mask & (result << 8);
     valids |= result << 8;
 
     mask = opponent & 0x007E7E7E7E7E7E00;
     result = mask & (myboard >> 7);
     result |= mask & (result >> 7);
     result |= mask & (result >> 7);
     result |= mask & (result >> 7);
     result |= mask & (result >> 7);
     result |= mask & (result >> 7);
     valids |= result >> 7;
 
     result = mask & (myboard >> 9);
     result |= mask & (result >> 9);
     result |= mask & (result >> 9);
     result |= mask & (result >> 9);
     result |= mask & (result >> 9);
     result |= mask & (result >> 9);
     valids |= result >> 9;
 
     result = mask & (myboard << 9);
     result |= mask & (result << 9);
     result |= mask & (result << 9);
     result |= mask & (result << 9);
     result |= mask & (result << 9);
     result |= mask & (result << 9);
     valids |= result << 9;
 
     result = mask & (myboard << 7);
     result |= mask & (result << 7);
     result |= mask & (result << 7);
     result |= mask & (result << 7);
     result |= mask & (result << 7);
     result |= mask & (result << 7);
     valids |= result << 7;
 
     return valids & akikoma;
 }
 
 fn valid_board(myboard:u64, opponent:u64, position: u64) -> u64{
         check_right(myboard, opponent, position) 
         | check_left(myboard, opponent, position) 
         | check_down(myboard, opponent, position)
         | check_up(myboard, opponent, position)
         | check_diagonally_upright(myboard, opponent, position)
         | check_diagonally_upleft(myboard, opponent, position)
         | check_diagonally_downright(myboard, opponent, position)
         | check_diagonally_downleft(myboard, opponent, position)
 }
 
 impl OthelloBoard {
     // 新しい空のオセロ盤を作成する関数
     pub fn new() -> OthelloBoard {
         OthelloBoard {
             black: (1 << 28) | (1 << 35),
             white: (1 << 27) | (1 << 36),
         }
     }
 
     pub fn subtract(&self, color:usize) -> isize{
         let black_number = count_pieces(self.black);
         let white_number = count_pieces(self.white);
         if color == 0{
             // color is black
             black_number - white_number
         }else{
             // color is white
             white_number - black_number
         }
     }
 
     // 盤面の状態を表示する関数
     pub fn print(&self, color: usize) {
         println!("  A B C D E F G H");
 
         let valid = self.valid_pieces(color);
         for row in 0..8 {
             let rows = row + 1;
             print!("{} ",rows);
             for col in 0..8 {
                 let pos = row * 8 + col;
                 let bit = 1 << pos;
 
                 if (self.black & bit) != 0 {
                     print!("B ");
                 } else if (self.white & bit) != 0 {
                     print!("W ");
                 } else {
                     let num = valid[pos as usize];
                     if num == 0{
                         print!("- ");
                     }else{
                         print!("{} ",num);
                     }
                     
                 }
             }
             println!();
         }
     }
 
     fn valid_total(&self, position: u64, color: usize) -> usize{
         if color == 0{
             count_pieces(valid_board(self.black, self.white, position)) as usize
         }else{
             count_pieces(valid_board(self.white, self.black, position)) as usize
         }
     }
 
     pub fn valid_pieces(&self, color: usize) -> Vec<usize>{
          let mut position: u64 = 1;
          let mut pos:usize = 0;
          let mut getting_number: Vec<usize> = Vec::new();
          let mut valid_pos: Vec<usize> = Vec::new();
          let mut arrangements:usize = 0;
         loop {
             if self.black & position != 0 || self.white & position != 0 {
                 getting_number.push(0);
             }
             else{
                 let total = self.valid_total(position, color);
                 if total > 0{
                     arrangements += 1;
                     valid_pos.push(pos);
                 }
                 getting_number.push(total);
             }
 
             position <<= 1;
             pos += 1;
 
             if position == 0 {
                 break;
             }
         }
         getting_number.push(arrangements);
         getting_number.extend(valid_pos);
 
         return getting_number;
     }
 
     fn change_pieces(&mut self, position: u64, color: usize){
         if color == 0{
             let board= valid_board(self.black, self.white, position);
             self.black |= board;
             self.white &= !board;
         }else{
             let board= valid_board(self.white, self.black, position);
             self.white |= board;
             self.black &= !board;
         }
     }
 
     pub fn add(&mut self, bit: usize, turn: &mut usize){
         let position = 1 << bit as u64;
         if *turn == 0 { //black's turn
             self.black |= position;
 
             self.change_pieces(position,*turn);
 
             *turn = 1;
         }else{ // white's turn
             self.white |= position;
 
             self.change_pieces(position,*turn);
             
             *turn = 0;
         }
     }
 
     pub fn add2(&self, position: u64, turn: usize)-> OthelloBoard{
         let mut board2 = OthelloBoard {
             black: self.black,
             white: self.white,
         };
         if turn == 0 { //black's turn
             board2.black |= position;
 
             board2.change_pieces(position,turn);
 
         }else{ // white's turn
             board2.white |= position;
 
             board2.change_pieces(position,turn);
         
         }
         board2
     }
 
     pub fn make_1_next_boards3(&self, color:usize) -> Vec<OthelloBoard>{
         let akikoma = !(self.white | self.black);
         let mut color2 = color;
 
         if akikoma == 0 {
             return vec![];
         }
 
         let mut valid:u64;
 
         if color == 0{
             valid = valids(self.black,self.white);
             if valid == 0 {
                 color2 = 0b1 ^ color;
                 valid = valids(self.white,self.black);
                     if valid == 0{
                         return vec![];
                     }
             }
         }else{
             valid = valids(self.white,self.black);
             if valid == 0 {
                 color2 = 0b1 ^ color;
                 valid = valids(self.black,self.white);
                     if valid == 0{
                         return vec![];
                     }
             }
         }
 
         let mut nextboards_sub:Vec<OthelloBoard> = vec![];
 
         let mut check = valid;
         let mut nokori = valid;
         let k = count_pieces(valid);
 
         for _ in 0..k {
 
             valid = nokori;
             valid >>= 1;
             valid |= valid >> 1;
             valid |= valid >> 2;
             valid |= valid >> 4;
             valid |= valid >> 8;
             valid |= valid >> 16;
             valid |= valid >> 32;
             check &= !(valid);
 
             nextboards_sub.push(self.add2(check, color2));
             nokori &= !(check);
             check = nokori;
         }
 
         nextboards_sub.push(OthelloBoard{ black: 0, white: (0b1 ^ color2) as u64, });
         return  nextboards_sub;
 
     }
 
     pub fn make_1_next_boards2(&self, color: usize) -> Vec<(OthelloBoard,u64,usize,isize)>{
         let board = OthelloBoard {
             black: self.black,
             white: self.white,
         };
 
         let mut valid:u64;
         let myvalid_count:isize;
         let oppvalid_count:isize;
 
         if color == 0{
             valid = valids(self.black,self.white);
             myvalid_count = count_pieces(valid);
             oppvalid_count = count_pieces(valids(self.white,self.black));
             
         }else{
             valid = valids(self.white,self.black);
             myvalid_count = count_pieces(valid);
             oppvalid_count = count_pieces(valids(self.black,self.white));
         }
 
         let mut nextboards:Vec<(OthelloBoard,u64,usize,isize)> = vec![];
 
         let mut check = valid;
         let mut nokori = valid;
         let k = count_pieces(valid);
 
         for _ in 0..k {
 
             valid = nokori;
             valid >>= 1;
             valid |= valid >> 1;
             valid |= valid >> 2;
             valid |= valid >> 4;
             valid |= valid >> 8;
             valid |= valid >> 16;
             valid |= valid >> 32;
             check &= !(valid);
 
             let next_board = board.add2(check, color);
 
             let score = myvalid_count*oppvalid_count;
             //println!("score:{}",score);
             nextboards.push((next_board, check ,color^0b1 , score));
 
             nokori &= !(check);
             check = nokori;
         }
 
         nextboards.sort_by(|a, b| a.3.cmp(&b.3)); //a < b
         return nextboards;
             
     }

     pub fn make_1_next_boards2_5(&self, color: usize) -> Vec<(OthelloBoard,u64,isize)>{
        let board = OthelloBoard {
            black: self.black,
            white: self.white,
        };

        let mut valid:u64;
        let myvalid_count:isize;
        let oppvalid_count:isize;

        if color == 0{
            valid = valids(self.black,self.white);
            myvalid_count = count_pieces(valid);
            oppvalid_count = count_pieces(valids(self.white,self.black));
            
        }else{
            valid = valids(self.white,self.black);
            myvalid_count = count_pieces(valid);
            oppvalid_count = count_pieces(valids(self.black,self.white));
        }

        let mut nextboards:Vec<(OthelloBoard,u64,isize)> = vec![];

        let mut check = valid;
        let mut nokori = valid;
        let k = count_pieces(valid);

        for _ in 0..k {

            valid = nokori;
            valid >>= 1;
            valid |= valid >> 1;
            valid |= valid >> 2;
            valid |= valid >> 4;
            valid |= valid >> 8;
            valid |= valid >> 16;
            valid |= valid >> 32;
            check &= !(valid);

            let next_board = board.add2(check, color);

            let score = myvalid_count*oppvalid_count;
            //println!("score:{}",score);
            nextboards.push((next_board, check ,score));

            nokori &= !(check);
            check = nokori;
        }

        nextboards.sort_by(|a, b| a.2.cmp(&b.2)); //a < b
        return nextboards;
            
    }
 
     pub fn make_1_next_boards1(&self, color:usize) -> Vec<OthelloBoard>{
         let akikoma = !(self.white | self.black);
         let mut color2 = color;
     
         if akikoma == 0 {
             return vec![];
         }
     
         let mut valid:u64;
     
         if color == 0{
             valid = valids(self.black,self.white);
             if valid == 0 {
                 color2 = 0b1 ^ color;
                 valid = valids(self.white,self.black);
                     if valid == 0{
                         return vec![];
                     }
             }
         }else{
             valid = valids(self.white,self.black);
             if valid == 0 {
                 color2 = 0b1 ^ color;
                 valid = valids(self.black,self.white);
                     if valid == 0{
                         return vec![];
                     }
             }
         }
     
         let mut nextboards_sub:Vec<(OthelloBoard,isize)> = vec![];
     
         let mut check = valid;
         let mut nokori = valid;
         let k = count_pieces(valid);
     
         for _ in 0..k {
     
             valid = nokori;
             valid >>= 1;
             valid |= valid >> 1;
             valid |= valid >> 2;
             valid |= valid >> 4;
             valid |= valid >> 8;
             valid |= valid >> 16;
             valid |= valid >> 32;
             check &= !(valid);
     
             let next_board = self.add2(check, color2);
     
             if color2 == 1{
                 valid = valids(next_board.black,next_board.white);
             }else{
                 valid = valids(next_board.white,next_board.black);
             }
     
             nextboards_sub.push((next_board,count_pieces(valid)));
             nokori &= !(check);
             check = nokori;
         }
     
         nextboards_sub.sort_by(|a, b| a.1.cmp(&b.1)); //降順
     
         let mut first_elements: Vec<_> = nextboards_sub.iter().map(|(first, _)| OthelloBoard { black: first.black, white: first.white }).collect();
         first_elements.push(OthelloBoard{ black: 0, white: (0b1 ^ color2) as u64, });
         return  first_elements;
     
     }
 
     pub fn make_1_next_boards1_5(&self, color:usize) -> Vec<OthelloBoard>{
         let akikoma = !(self.white | self.black);
         let mut color2 = color;
 
         if akikoma == 0 {
             return vec![];
         }
 
         let mut valid:u64;
 
         if color == 0{
             valid = valids(self.black,self.white);
             if valid == 0 {
                 color2 = 0b1 ^ color;
                 valid = valids(self.white,self.black);
                     if valid == 0{
                         return vec![];
                     }
             }
         }else{
             valid = valids(self.white,self.black);
             if valid == 0 {
                 color2 = 0b1 ^ color;
                 valid = valids(self.black,self.white);
                     if valid == 0{
                         return vec![];
                     }
             }
         }
 
         let mut nextboards_sub:Vec<(OthelloBoard,isize)> = vec![];
 
         let mut check = valid;
         let mut nokori = valid;
         let k = count_pieces(valid);
 
         for _ in 0..k {
 
             valid = nokori;
             valid >>= 1;
             valid |= valid >> 1;
             valid |= valid >> 2;
             valid |= valid >> 4;
             valid |= valid >> 8;
             valid |= valid >> 16;
             valid |= valid >> 32;
             check &= !(valid);
 
             let next_board = self.add2(check, color2);
 
             if color2 == 1{
                 valid = valids(next_board.black,next_board.white);
             }else{
                 valid = valids(next_board.white,next_board.black);
             }
 
             nextboards_sub.push((next_board,count_pieces(valid)));
             
             
             nokori &= !(check);
             check = nokori;
         }
 
         nextboards_sub.sort_by(|a, b| a.1.cmp(&b.1)); //降順
 
         let mut first_elements: Vec<_> = nextboards_sub.iter().map(|(first, _)| OthelloBoard { black: first.black, white: first.white }).collect();
         first_elements.push(OthelloBoard{ black: 0, white: (0b1 ^ color2) as u64, });
         return  first_elements;
 
     }

     pub fn make_1_next_boards1_5_2(&self, color:usize) -> Vec<(OthelloBoard,u64)>{
        let akikoma = !(self.white | self.black);
        let mut color2 = color;

        if akikoma == 0 {
            return vec![];
        }

        let mut valid:u64;

        if color == 0{
            valid = valids(self.black,self.white);
            if valid == 0 {
                color2 = 0b1 ^ color;
                valid = valids(self.white,self.black);
                    if valid == 0{
                        return vec![];
                    }
            }
        }else{
            valid = valids(self.white,self.black);
            if valid == 0 {
                color2 = 0b1 ^ color;
                valid = valids(self.black,self.white);
                    if valid == 0{
                        return vec![];
                    }
            }
        }

        let mut nextboards_sub:Vec<(OthelloBoard,u64,isize)> = vec![(OthelloBoard{ black: 0, white: (0b1 ^ color2) as u64, },0,isize::MIN)];

        let mut check = valid;
        let mut nokori = valid;
        let k = count_pieces(valid);

        for _ in 0..k {

            valid = nokori;
            valid >>= 1;
            valid |= valid >> 1;
            valid |= valid >> 2;
            valid |= valid >> 4;
            valid |= valid >> 8;
            valid |= valid >> 16;
            valid |= valid >> 32;
            check &= !(valid);

            let next_board = self.add2(check, color2);

            if color2 == 1{
                valid = valids(next_board.black,next_board.white);
            }else{
                valid = valids(next_board.white,next_board.black);
            }

            nextboards_sub.push((next_board,check,count_pieces(valid)));
            
            
            nokori &= !(check);
            check = nokori;
        }

        nextboards_sub.sort_by(|a, b| a.2.cmp(&b.2)); //降順 a<b

        let first_elements: Vec<(_,_)> = nextboards_sub.iter().map(|(first, second, _)| ((OthelloBoard { black: first.black, white: first.white }),*second)).collect();

        return  first_elements;

    }
 
     fn last_mini_max_sub_sub(&self, color: usize, turn: usize, times:&mut u64)->isize{
         *times += 1;
         if *times > 500000000{
             return 0;
         }
 
         let mut boards = self.make_1_next_boards3(turn);
         if boards.is_empty(){
             return self.subtract(color);
         }
 
         let mut return_count:isize = -99;
         let mut now_count:isize = -99;
 
         if let Some(OthelloBoard{ black: _, white: check}) = boards.pop() {
 
             if 1-check == color as u64{
                 for board in boards {
                         now_count = board.last_mini_max_sub_sub(color, 1-color, times);
                         
                         if 0 < now_count {
                             return now_count;
                         }else if now_count == 0{
                             return_count = 0;
                         }
                 }
 
             } else{
                 for board in boards {
                         now_count = board.last_mini_max_sub_sub(color, color, times);
                         
                         if 0 > now_count {
                             return now_count;
                         }else if now_count == 0{
                             return_count = 0;
                         }
                 }
             }
         }
 
         if return_count == -99{
             return now_count;
         }else{
             return 0;
         }
     }
 
     fn last_mini_max_sub(&self, color: usize, turn: usize, times:&mut u64, start_time: Instant, timeout: Duration, count:usize)->isize{
         if count > 54{
             return self.last_mini_max_sub_sub(color, turn, times);
         }
 
         *times += 1;
         if *times > 500000000{
             return 0;
         }
 
         let mut boards = self.make_1_next_boards1(turn);
         if boards.is_empty(){
             return self.subtract(color);
         }
 
         let mut return_count:isize = -99;
         let mut now_count:isize = -99;
 
         if let Some(OthelloBoard{ black: _, white: check}) = boards.pop() {
 
             if 1-check == color as u64{
                 for board in boards {
                     let elapsed_time = start_time.elapsed();
                     if elapsed_time > timeout{
                         now_count = 0;
                     }else{
                         now_count = board.last_mini_max_sub(color, 1-color, times, start_time, timeout, count+1);
                         if 0 < now_count {
                             return now_count;
                         }else if now_count == 0{
                             return_count = 0;
                         }
                     }
                 }
 
             } else{
                 for board in boards {
                     let elapsed_time = start_time.elapsed();
                     if elapsed_time > timeout{
                         now_count = 0;
                     }else{
                         now_count = board.last_mini_max_sub(color, color, times, start_time, timeout, count+1);
                         
                         if 0 > now_count {
                             return now_count;
                         }else if now_count == 0{
                             return_count = 0;
                         }
                     }
                 }
             }
         }
 
         if return_count == -99{
             return now_count;
         }else{
             return 0;
         }
     }
 
     fn last_mini_max(&self, color: usize, start_time: Instant, timeout: Duration, count:usize, n:usize)-> usize{
         let boards = self.make_1_next_boards2(color);
         let mut position:u64 = 0;
         let mut max:isize = isize::MIN;
         let mut now_count ;
         let mut times:u64 = 0;
         for (board, pos, turn,score) in boards {
             let elapsed_time = start_time.elapsed();
             //println!("{:?}",elapsed_time);
             if elapsed_time > timeout{
                 now_count = score;
             }else{
                 if count > 54{
                     now_count = 50000*board.last_mini_max_sub_sub(color, turn , &mut times);
                 }else{
                     now_count = 50000*board.last_mini_max_sub(color, turn, &mut times, start_time, timeout, count);
                 }
                 if now_count == 0{
                     now_count = score;
                 }
             }
 
             if now_count > 99999{
                 position = pos;
                 //println!("certain to win");
                 break;
             }
 
             if max < now_count{
                 max = now_count;
                 // return_pos = pos;
                 position = pos;
             }
             //println!("times:{}",times);
         }
 
        //  if now_count < -99999{
        //      //println!("certain to lose");
        //  }else if now_count < 100000{
        //      //println!("uncertain");
        //  }
 
         let mut position = position >> 1;
         position |= position >> 1;
         position |= position >> 2;
         position |= position >> 4;
         position |= position >> 8;
         position |= position >> 16;
         position |= position >> 32;
 
         return count_pieces(position) as usize;
 
     }
 
     fn eval_board_for_last(&self, color:usize) -> isize{
        let akikoma = !(self.black | self.white);
        let oppcolor = color ^ 0b1;
        let myvalid:u64;
        let oppvalid:u64;
        let myboard:u64;
        let oppboard:u64;
        let mut eval_point:isize = 0;
        let my_kakutei:isize;
        let opp_kakutei:isize;

        if color == 0{
            myboard = self.black;
            oppboard = self.white;
            myvalid = valids(self.black,self.white);
            oppvalid = valids(self.white,self.black);
            my_kakutei = count_pieces((self.add2(akikoma, oppcolor)).add2(akikoma, oppcolor).black);
            opp_kakutei = count_pieces((self.add2(akikoma, color)).add2(akikoma, color).white);
            if myvalid | oppvalid == 0{
                if self.subtract(color) > 0{
                    return 5000;
                }else{
                    return -5000;
                }
            }
        }else{
            myboard = self.white;
            oppboard = self.black;
            myvalid = valids(self.white,self.black);
            oppvalid = valids(self.black,self.white);
            my_kakutei = count_pieces((self.add2(akikoma, oppcolor)).add2(akikoma, oppcolor).white);
            opp_kakutei = count_pieces((self.add2(akikoma, color)).add2(akikoma, color).black);
            if myvalid | oppvalid == 0{
                if self.subtract(color) > 0{
                    return 5000;
                }else{
                    return -5000;
                }
            }
        }

        let kakuteisa = my_kakutei - opp_kakutei;

        if my_kakutei > 32{
            return 5000;
        } else if opp_kakutei < -32{
            return -5000;
        }else if kakuteisa > 15{
            return 3000;
        }else if kakuteisa < -15{
            return  -3000;
        }
         else {
            eval_point = eval_point + kakuteisa*20;
        }

        eval_point = eval_point + count_pieces(myvalid)*10 - count_pieces(oppvalid)*10;

        let kado = myboard & 0x8100000000000081;
        let oppkado = oppboard & 0x8100000000000081;
        let kado_count = count_pieces(kado);
        let kado_oppcount = count_pieces(oppkado);
        let sa = kado_count - kado_oppcount;
        if -1 <= sa && 1 >= sa{
            if kado_count == 0 || kado_oppcount == 0{
                eval_point = eval_point + sa*500;
            }else{
            eval_point = eval_point + sa*100;
            }
        }else{
            eval_point = eval_point + sa*250;
        }
        
        return eval_point;
    }

     fn eval_mini_max_sub2(&self, color: usize, turn: usize, times:&mut u64, n:usize, alpha:isize, memo_eval: &mut HashMap<(u64,u64),Vec<(OthelloBoard,u64,isize)>> ,memo_banmen: &mut HashMap<(u64,u64),Vec<(OthelloBoard,u64)>>,memo_banmen_pre: &HashMap<(u64,u64),Vec<(OthelloBoard,u64)>>)->isize{
        *times += 1;

        let mut memo:Vec<(OthelloBoard,u64,isize)> = vec![];
        let boards: Vec<(OthelloBoard,u64)>;
        let mut return_count:isize;
        let mut now_count:isize;
        let mut beta = isize::MIN;
        let n_1 = n - 1;


        if let Some(value) = (*memo_banmen_pre).get(&(self.black,self.white)){
           if value.is_empty() || n == 0{
               return self.eval_board_for_last(color);
           }
   
           if value[0].0.white != color as u64{
               return_count = isize::MIN;
               for (board,pos) in value.iter().skip(1) {

                       now_count = board.eval_mini_max_sub2(color, color^0b1, times, n_1,beta, memo_eval, memo_banmen,memo_banmen_pre);

                       memo.push((OthelloBoard{black:board.black,white:board.white},*pos,now_count));
                       
                       if return_count < now_count {
                           if alpha == isize::MIN || alpha > now_count{
                               return_count = now_count;  
                               beta = now_count;
                           }else{
                               return_count = now_count;
                               break;
                           }
                       }
                       
               }

           } else{
               return_count = isize::MAX;
               for (board,_) in value.iter().skip(1) {

                       now_count = board.eval_mini_max_sub2(color, color, times, n_1,beta,  memo_eval, memo_banmen,memo_banmen_pre); 

                       if return_count > now_count {
                           if alpha == isize::MIN || alpha < now_count{
                               return_count = now_count;  
                               beta = now_count;
                           }else{
                               return_count = now_count;
                               break;
                           }
                           
                       }
               }

           }
           memo_banmen.insert((self.black,self.white),value.iter().cloned().collect());
           return return_count;
        }
        else{
           boards = self.make_1_next_boards1_5_2(turn);
       
           if boards.is_empty() || n == 0{
               return self.eval_board_for_last(color);
           }
   
           if boards[0].0.white != color as u64{
               return_count = isize::MIN;
               for (board,pos) in boards.iter().skip(1) {

                       now_count = board.eval_mini_max_sub2(color, color^0b1, times, n_1,beta, memo_eval, memo_banmen,memo_banmen_pre);

                       memo.push((OthelloBoard{black:board.black,white:board.white},*pos,now_count));
                       
                       if return_count < now_count {
                           if alpha == isize::MIN || alpha > now_count{
                               return_count = now_count;  
                               beta = now_count;
                           }else{
                               return_count = now_count;
                               break;
                           }
                       }
                       
               }
           } else{
               return_count = isize::MAX;
               for (board,_) in boards.iter().skip(1) {

                       now_count = board.eval_mini_max_sub2(color, color, times, n_1,beta,  memo_eval, memo_banmen,memo_banmen_pre); 

                       if return_count > now_count {
                           if alpha == isize::MIN || alpha < now_count{
                               return_count = now_count;  
                               beta = now_count;
                           }else{
                               return_count = now_count;
                               break;
                           }
                           
                       }
               }
               
           }
           memo_banmen.insert((self.black,self.white),boards);
           return return_count;
           
        }
    }
 
     fn eval_mini_max_sub(&self, color: usize, turn: usize, times:&mut u64, n:usize, alpha:isize, memo_eval: &mut HashMap<(u64,u64),Vec<(OthelloBoard,u64,isize)>> ,memo_banmen: &mut HashMap<(u64,u64),Vec<(OthelloBoard,u64)>>,memo_banmen_pre: &HashMap<(u64,u64),Vec<(OthelloBoard,u64)>>)->isize{
         *times += 1;

         let mut memo:Vec<(OthelloBoard,u64,isize)> = vec![];
         let boards: Vec<(OthelloBoard,u64)>;
         let mut return_count:isize;
         let mut now_count:isize = 0;
         let mut beta = isize::MIN;
         let n_1 = n-1;


         if let Some(value) = (*memo_banmen_pre).get(&(self.black,self.white)){
            if value.is_empty() || n == 0{
                return self.eval_board_for_last(color);
            }
    
            if value[0].0.white != color as u64{
                return_count = isize::MIN;
                let mut alpha_mode = 0;
                for (board,pos) in value.iter().skip(1) {
                    if alpha_mode == 0{
                        now_count = board.eval_mini_max_sub2(color, color^0b1, times, n_1,beta, memo_eval, memo_banmen,memo_banmen_pre);

                        memo.push((OthelloBoard{black:board.black,white:board.white},*pos,now_count));
                        
                        if return_count < now_count {
                            if alpha == isize::MIN || alpha > now_count{
                                return_count = now_count;  
                                beta = now_count;
                            }else{
                                return_count = now_count;
                                alpha_mode = 1;
                                now_count -= 30;
                            }
                        }
                    }else{
                        memo.push((OthelloBoard{black:board.black,white:board.white},*pos,now_count));
                    }
                        
                }
                memo.sort_by(|a, b| b.2.cmp(&a.2)); //a>b
                memo_eval.insert((self.black,self.white), memo);
                return return_count;

            } else{
                return_count = isize::MAX;
                for (board,_) in value.iter().skip(1) {

                        now_count = board.eval_mini_max_sub(color, color, times, n_1,beta,  memo_eval, memo_banmen,memo_banmen_pre); 

                        if return_count > now_count {
                            if alpha == isize::MIN || alpha < now_count{
                                return_count = now_count;  
                                beta = now_count;
                            }else{
                                return now_count;
                            }
                            
                        }
                }
                return return_count;
            }
         }
         else{
            boards = self.make_1_next_boards1_5_2(turn);
        
            if boards.is_empty() || n == 0{
                return self.eval_board_for_last(color);
            }
    
            if boards[0].0.white != color as u64{
                return_count = isize::MIN;
                let mut alpha_mode = 0;
                for (board,pos) in boards.iter().skip(1) {
                    if alpha_mode == 0{
                        now_count = board.eval_mini_max_sub2(color, color^0b1, times, n_1,beta, memo_eval, memo_banmen,memo_banmen_pre);

                        memo.push((OthelloBoard{black:board.black,white:board.white},*pos,now_count));
                        
                        if return_count < now_count {
                            if alpha == isize::MIN || alpha > now_count{
                                return_count = now_count;  
                                beta = now_count;
                            }else{
                                return_count = now_count;
                                alpha_mode = 1;
                                now_count -= 30;
                            }
                        }
                    }else{
                        memo.push((OthelloBoard{black:board.black,white:board.white},*pos,now_count));
                    }
                        
                }
                memo.sort_by(|a, b| b.2.cmp(&a.2)); //a>b
                memo_eval.insert((self.black,self.white), memo);
                memo_banmen.insert((self.black,self.white),boards);
            } else{
                return_count = isize::MAX;
                for (board,_) in boards.iter().skip(1) {

                        now_count = board.eval_mini_max_sub(color, color, times, n_1,beta,  memo_eval, memo_banmen,memo_banmen_pre); 

                        if return_count > now_count {
                            if alpha == isize::MIN || alpha < now_count{
                                return_count = now_count;  
                                beta = now_count;
                            }else{
                                return_count = now_count;
                                break;
                            }
                            
                        }
                }
                
            }
            return return_count;
            
         }
     }
 
     fn eval_mini_max(&self, color: usize, start_time: Instant, timeout: Duration, n:&mut usize, memo_eval: &mut HashMap<(u64,u64),Vec<(OthelloBoard,u64,isize)>> ,memo_eval_pre: &HashMap<(u64,u64),Vec<(OthelloBoard,u64,isize)>> ,memo_banmen: &mut HashMap<(u64,u64),Vec<(OthelloBoard,u64)>>,memo_banmen_pre: &HashMap<(u64,u64),Vec<(OthelloBoard,u64)>>)-> usize{
        let mut return_count:isize = isize::MIN;
        let mut now_count:isize;
        let mut beta = isize::MIN;
        let mut times= 0;
        let aite = color^0b1;
        let n_1 = *n-1;
        let mut return_pos = 0;

        if let Some(value) = (*memo_eval_pre).get(&(self.black,self.white)) {

            for (board,pos,score) in value {
                let elapsed_time = start_time.elapsed();
                if elapsed_time > timeout{
                    now_count = *score;
                }else{
                    now_count = (*board).eval_mini_max_sub(color, aite, &mut times, n_1,beta, memo_eval,memo_banmen,memo_banmen_pre);
                }

                if return_count < now_count {
                        return_count = now_count;  
                        beta = now_count;
                        return_pos = *pos;
                }
            
            }

        } else {
            let boards: Vec<(OthelloBoard, u64, isize)> = self.make_1_next_boards2_5(color);

            for (board,check,_) in boards {
                    let elapsed_time = start_time.elapsed();
                    if elapsed_time > timeout{
                        *n -= 1;
                        break;
                    }else{
                        now_count = board.eval_mini_max_sub(color, aite, &mut times, n_1,beta, memo_eval, memo_banmen,memo_banmen_pre);

                        if return_count < now_count {
                                return_count = now_count;  
                                beta = now_count;
                                return_pos = check;
                        }
                    }
                
            }
        }

         let mut position = return_pos >> 1;
         position |= position >> 1;
         position |= position >> 2;
         position |= position >> 4;
         position |= position >> 8;
         position |= position >> 16;
         position |= position >> 32;
 
         return count_pieces(position) as usize;
     }
 
     pub fn eval_boards(&self, color: usize, count: usize, n: &mut usize, start_time: Instant, timeout: Duration, memo_banmen: &mut HashMap<(u64,u64),Vec<(OthelloBoard,u64)>>, memo_banmen_pre: &HashMap<(u64,u64),Vec<(OthelloBoard,u64)>>, memo_yomikiri: &mut HashMap<(u64,u64),Vec<(OthelloBoard,u64,isize)>>,memo_eval: &mut HashMap<(u64,u64),Vec<(OthelloBoard,u64,isize)>>,memo_eval_pre: &HashMap<(u64,u64),Vec<(OthelloBoard,u64,isize)>>)-> usize{
         if count > 45{//montecalro max 51
                       //a-b max 43 43が1手読める程度
                       //44-45が割れ目か　念のため44-46で計測する必要あり
                       //43-41は不確定切りが必要か
             return self.last_mini_max(color,start_time,timeout,count, *n);
         }else{
            //  if pre_mytime - mytime == 0{
            //      *n = 8;
            //  }else if pre_mytime - mytime < 250{
            //      *n += 1;
            //  }else if pre_mytime - mytime > 10000{
            //      *n -= 2;
            //  }else if pre_mytime - mytime > 2000{
            //      *n -= 1;
            //  }
             return self.eval_mini_max(color, start_time, timeout, n, memo_eval, memo_eval_pre,memo_banmen,memo_banmen_pre);
         }
         
     }

    pub fn make_1_next_boards_tezyun(&self, tezyun: Vec<char>,color:usize) -> Vec<(OthelloBoard,Vec<char>)>{
        let akikoma = !(self.white | self.black);
    
        if akikoma == 0 {
            return vec![];
        }
    
        let mut valid:u64;
    
        if color == 0{
            valid = valids(self.black,self.white);
            if valid == 0 {
                return vec![];
            }
        }else{
            valid = valids(self.white,self.black);
            if valid == 0 {
                return vec![];
            }
        }
    
        let mut nextboards_sub:Vec<(OthelloBoard,Vec<char>)> = vec![];
    
        let mut check = valid;
        let mut nokori = valid;
        let k = count_pieces(valid);

        for _ in 0..k {
            
            valid = nokori;
            valid >>= 1;
            valid |= valid >> 1;
            valid |= valid >> 2;
            valid |= valid >> 4;
            valid |= valid >> 8;
            valid |= valid >> 16;
            valid |= valid >> 32;
            let posi = count_pieces(valid);
            check &= !(valid);
    
            let next_board = self.add2(check, color);

            let yoko = (posi % 8) as usize;
            let tate = ((posi / 8) + 1) as usize;
            let mapping = vec!['A','B','C','D','E','F','G','H'];

            let getc: Vec<char> = vec![mapping[yoko], (tate as u8 + 48) as char];

            let tezyun2 = tezyun.iter().chain(getc.iter()).cloned().collect();

            if color == 1{
                valid = valids(next_board.black,next_board.white);
                if valid == 0{
                    nextboards_sub.extend(next_board.make_1_next_boards_tezyun(tezyun2, color));
                }else{
                    nextboards_sub.push((next_board,tezyun2));
                }
            }else{
                valid = valids(next_board.white,next_board.black);
                if valid == 0{
                    nextboards_sub.extend(next_board.make_1_next_boards_tezyun(tezyun2, color));
                }else{
                    nextboards_sub.push((next_board,tezyun2));
                }
            }

            nokori &= !(check);
            check = nokori;
        }
    
        return  nextboards_sub;
    
    }
 
 }

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let id = parse_input!(input_line, i32); // id of your player.
    let aite = id^0b1;
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let board_size = parse_input!(input_line, i32);
    let mut n = 4;
    let mut memo_eval:HashMap<(u64,u64), Vec<(OthelloBoard,u64,isize)>> = HashMap::new();
    let mut memo_eval_pre:HashMap<(u64,u64), Vec<(OthelloBoard,u64,isize)>> = HashMap::new();
    let mut memo_yomikiri:HashMap<(u64,u64), Vec<(OthelloBoard,u64,isize)>> = HashMap::new();
    let mut memo_banmen:HashMap<(u64,u64), Vec<(OthelloBoard,u64)>> = HashMap::new();
    let mut memo_banmen_pre:HashMap<(u64,u64), Vec<(OthelloBoard,u64)>> = HashMap::new();

    let zhoseki: Vec<&str> = [
        "F5D6C3D3C4F4C5B3C2E3D2C6B4A3G3F3E6G4B5E1D1C1D7H3E2G6G2F6A4F7E7H1G5H5F2E8D8C8A2A6B2A5F8G8C7B6A7G7H2B7H4A1A8B8H8B1H7H6G1F1",
        "F5D6C3D3C4F4C5B3C2E3D2C6B4A3G3F3E6G4B5E1D1C1D7H3E2G6G2F6A4F7E7H1G5H5F2E8D8C8A2A6B2A5F8G8C7B6A7G7H2B7H4F1A8B8G1H6B1A1H8H7",
        "F5D6C3D3C4F4C5B3C2E3D2C6B4A3G3F3E6G4B5E1D1C1D7H3E2G6G2F6A4F7E7H1G5H5F2E8D8C8A2A6B2A5F8G8C7B6A7G7H2B7H4F1A8B8G1H6H8H7B1A1",
        "F5D6C3D3C4F4C5B3C2E3D2C6B4A3G3F3E6G4B5E1D1C1D7H3E2G6G2F6A4F7E7H1G5H5F2E8D8C8A2A6B2A5F8G8C7B6A7G7H2B7H4F1A8B8H8G1H7H6A1B1",
        "F5D6C3D3C4F4C5B3C2E3D2C6B4A3G3F3E6G4B5E1D1C1D7H3E2G6G2F6A4F7E7H1G5H5F2E8D8C8A2A6B2A5F8G8C7B6A7G7H2B7H4F1G1H6A8B8B1A1H8H7",
        "F5D6C3D3C4F4C5B3C2E3D2C6B4A3G3G4F3C1B5A4A6D1E2F2G1F6E6G6F1G5H4E1B1D7H6A5A2H5D8C8E7H3H2G7H7E8B6C7H8F7G2G8B7A7A8B8F8H1B2A1",
        "F5D6C3D3C4F4C5B3C2E3D2C6B4A3G3G4F3C1B5A4A6D1E2F2G1F6E6G6F1G5H4E1B1D7H6A5A2H5D8C7H3E7B7A8F7E8F8G7H8G8H7B6G2C8A7B8B2A1H1H2",
        "F5D6C5F4E3C6D3F6E6D7G4C4G5C3F7D2E7F2C8F3C7D8E8G3E2H5H3G6C1H4H6G2B3B4H1H2G7E1G1A3B6B5A5A4A6A7C2G8F8F1B2B1B7D1H8H7A1B8A8A2",
        "F5F6E6F4E3D6C6C5E7D7C3B5G3E2F7D3C8G4C7F8E8G6H3B3H6G7G5G8B6F3C4A6A4B4F2F1D2C2E1D1B1D8G2H5H4H1H7H2H8G1C1B7B2A1A2A3A5A7A8B8",
        "F5F6E6F4E3C5C6D6E7F3C4B5G6F7G5H6C7B6C3B4F8D3D7C8A3A4B3D2A5A6E2G3F2F1C1D1H3D8E8G8A7G1C2G7H8H7E1H5H1H2B8H4B7G2G4B1A1A8B2A2",
        "F5F6E6F4E3D6C6C5E7F7C3F3D7C8C4C7G6E8D3G4D8H5H6B6F8G8H3H4G5B5A5E2G7G3D2F2B4C1D1E1A7C2B2B3B1A4A2A6A3A1B7A8B8H7H8H2G2F1G1H1",
        "F5F6E6F4E3C5C6D6E7F7C3F3D7C8C4D8G4B6G5G3C7B5E2B3B4A3F8B8A5D3A6F1D2A4E1A7F2H5G6H3H4H6G7C2G1C1E8D1A8B7G2H8A2B2B1A1H2H1H7G8",
        "F5D6C3D3C4F4C5B3C2E3D2C6B4A4B5B6G4E2A3E1D7C1F2A2D1F3F1G1A5A6C7F6G5E7E6G6E8H3F7H5G2G7H8D8H7H1H6H4C8B2H2G3A1B1A7B8A8B7F8G8",
        "F5F6E6F4E3D6G4D3C3H3C4G3G5G6C7C6C5B6D7B5E7F3B4F7H5H4H2D2C2E2F8A4B3G7H8G8F1E1F2G1A3G2A6E8D8A5B2A1D1C1B1A2H1B8H7H6C8A7B7A8",
        "F5F6E6F4E3C5C4D6B5D3C3E2F3G4F1D1H4G3C6G5H5G6E1G1F2H2F7D2H3H6C7D7C1B1C2B6D8E7E8A4B4B2A5A6B3A3G7F8G8H7A7B8H8A8H1G2C8B7A1A2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D1C1B1G6D2A2F2G8H4H5B7A8F8B8C8B2G4A3A1A7H8G7H7H6H3H2E2E1F3G2H1G1F1",
        "F5D6C3D3C4F4C5B3C2E3D2C6B4A3G3G4F3C1B5A4A6D1E2F2G1F6E6G6F1G5H4D7H6E1B1A5A2H5D8C7H3E7B7A8F7E8F8G7H8G8H7B6G2H1H2C8A7B8B2A1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1F3E2H6C8B8G4H4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1F3E2H6G4H4F8F1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1F3E2H6G4H4C8A7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1F3E2H6G4H4C8F1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1F3E2H6G4H4C8B8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1F3E2H6G4F1G6E1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1F3E2H6G4F1G6G7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1F3E2H6G4F1E1H4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1F3E2H6G4F1D1A7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1F3E2H6G6G4F2H3",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1F3E2H6G6G4F2H5",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1G6F3H5H6E2H4E1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1G6F3E2D1F1H6F2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1G6F3E2H6G4H3E1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1G6F3E2H6G4H3A7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1G6F3E2H6G4A2H4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1G6F3A7H6E2B2E1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1G6F3A7H6E2F2G7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1G6F3A7H6E2F8G4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1G6F3A7H6E2F8F2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1G6F3A7H6E2F8E1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1G6H6G4F3E2H3A7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1G6H6G4F3E2F8G8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1G6H6G4F3E2F8A7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1G6H6G4F3E1H5E2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1G6H6G4F3E1H5D1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1G6H6G4F3E1D1B1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1G6H6G4F3E1E2H4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1G6H6E2G4F3F2E1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1G6H6F3G4E1G8F8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2C1A7G6H6F8G8C8B8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2D1H4G6H5G8F3E2G4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2D1H5G6A7G8F3H4H3",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2D1H5G6A7G8F3G4F8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2D1H5G6A7G8F3F2G4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2D1H5G6A7G8F3H6F8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2D1H5G6A7H6H7G8H4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2D1H5G6A7H6H7G8E2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2D1C1G6E1C8H4H5G4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2D1C1G6E1C8G4F3H5",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D2D1C1G6E1C8G4A2B8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D1C1B1G6D2A2F2G8G4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D1C1B1G6D2A2F2G8A7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D1C1B1G6D2A2F2G8H4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D1C1B1G6D2A2A7G8F2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D1C1B1G6D2A2A7G8F3",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D1C1B1G6A7G8H6H5F8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D1C1B1G6H4H6A7A2F3",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D1C1B1G6H4H6A7C8D2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D1C1B1G6H4F8G8H5H6",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D1C1B1G6H4F8A7H5H6",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3F7G5D1C1B1G6H4F8A7A2G8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3D2F7F3F2C1E2F1F8G8B8H3",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3D2F7F3C1E1F2E2F8D1F1G8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3D2F7F3C1E1F2D1F1E2F8G8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3D2D1F7G5H4G6H5G8E2C8B8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3D2D1F7G5H4G6H5G8F2B8E2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3D2D1F7G5H4G6H5G8F2B8F3",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3D2D1F7G5H4G6H5G8F3E2G4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3D2D1F7G5H4G6H5G8F3E2F1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3D2D1F7G5C1G6E1C8H4H5G4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3D2D1F7G5C1G6E1C8G7A3F2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3D2D1F7G5C1G6E1C8G7A3F3",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3D2D1F7G5C1G6E1C8G4F3",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3D2D1F7G5C1G6E1C8G4A2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3D2D1F7G5C1G6G4F3H5B1H2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3D2D1F7G5C1G6G4F3H5F8G8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3D2D1F7G5C1G6G4F8G8F3H5",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3D2G4F7C1G5H5G6F3A7H6E2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3D2G4F7C1G5H5G6F3A7H6G7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3D2G4F7C1G5H5G6F3A7F8E2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3D2G5F7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6D8G3D2C1F7G5",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C8F2G6D1E1F1D8B8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C8F2G6D1E1D8G8F3",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C8F2G6D1D8H4A3A2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C8F2G6D1D8H4E2B8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C8F2G6D8G8F3E1G4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C8F2G6D8G8F3E1F1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C8F2G6D8G8F3E1D1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C8F2G6D8G8F3F1D1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C8F2G6D8G8F3F1H3",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C8F2G6D8G8F3F1G4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C8F2G6D8G8D1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C8G6F3H5G4D1H6H7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C8G6F3H5G4H2H3H4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C8G6F3H5G4H2E2A7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C8G6F3H5G4H2C1A7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C8D1G6F2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C1G6H6G4F3H4H5H7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C1G6H6G4F3A7B2H5",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C1G6H6G4F3A7B2H4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C1G6H6G4F3A7H3G7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C1G6H6G4F3A7H3E1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C1G6H6G4F3E1H3E2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C1G6H6G4F3E1H3A7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C1G6H6E2F3F2E1A7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C1G6H6E2F3F2E1G7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C1G6H6E2F3F2E1H4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C1G6C8H5D8G4F3H2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C1G6F3H5A2G4E2F1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D2C1G6F3H5A2E2F2H3",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D1C8D2G6F2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D1C8D2G6D8G8F2E1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7E8B6F8G3F7G5D1C8D2G6D8G8F2B8F3",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3E2F3H4H3G5E1C8D8E8F7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3E2F3H4H3G5E1C1B1C8D8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3E2F3H4H3G5E1C1B1G1B2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3E2F3H4H3G5E1C1B6B8F8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3E2F3H4H3G5E1C1F8D8B6",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3E2F3G6H5F1G5B1F8E8D8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3E2F3G6H5F1G5B1B6C8B7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3E2F3G6H5F1E1C1G5H6D8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3E2F3G6H5F1E1C1G5H6H7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3E2F3G6H5F1E1C1G5H6F8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3E2F3G6H5F1E1C1G5H6B6",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3E2F3G6H5F1E1C1G5G1B2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3E2F3G6E1C1B1G1B2G5H4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3E2F3G6E1C1H4G5B6B8B1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3E2F3G6E1C1H4G5B6B8H5",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3E2F3G6E1C1H4G5B6F1H5",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E1G5D8E2E8C1F1G1F3",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E1G5D8E2E8G1F1C1F3",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E1G5D8A3A2B7E2F3G6",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E1G5D8A3E8A7H4F8H3",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E1G5D8A3E8A7B2F8F3",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E1G5D8A3E8A7B2F8G8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E1G5D8A3E8A7H3G6H4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E1G5D8A3E8A7H3H4F8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E1G5D8A3H5H6H3G6H4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E2H4B6C8G1F3G5G6G7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E2H4B6C8B7C1B1F1B2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E2H4B6C8B7C1B1F3G2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E2H4D8A3B6A7F3E8H3",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E2H4D8A3B6A7F3E8B7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E2H4D8E8B6F1F3E1G1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E2F3D8E8F8E1H4H5H6",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E2F3D8E8F8E1G8C8B8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E2F3D8E8F8E1B6B8H4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E2F3D8E8E1H3",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E2F3B6C8H5F1E1G2G6",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E2F3B6F1D8B8C8E8E1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4D1F2G3F7E2F3E1H3D8E8B6C8H5",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4F8E8D1E2B6F7D8B8C8G8F1E1F2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4F8E8D1E2B6F7D8A3A2E1G5H6H5",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4F8E8D1E2B6F7D8A3A2E1G5H6G3",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4F8E8D1E2B6F7D8A3F1A7G8C1G6",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4F8E8D1F7D8B8C8G8B6E2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4F8E8D1F7D8E2B6",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4F8E8D1A3A2F2G3E2F3F1G6G5E1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4F8E8D1A3A2F2G3E2F3F1G6G5H6",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G4F8E8D1A3A2F2G3E2F3F1G6G5D8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3F3E1C8D8F7E8F8G6H3G4H4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3F3E1C8F7D8E8G8G4F8F2B6",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3F3E1C8F7D8E8G8G4F8F2G6",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3F3E1C8F7D8E8H3B7F8G4H4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3F3E1C8F7D8E8H3B7F8G4H5",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3F3E1C8F7D8E8F8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3F3E1F8E8H3G4H4F7B6E2F2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3F3E1F8E8H3G4H4F7G5E2F2",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3F3E1F8E8H3G4H4F7D8C8B8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3D1G4G5E2F1B6H3B7H2H5F3",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3D1G4G5E2F1B6H3B7F3G6A7",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3D1G4G5E2F1B6H3D8F7H5H4",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3D1F2F3C1F1G4G6F7H5H3G5",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3D1F2F3C1F1G4G6F7H5H4D8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3D1F2F3C1F1G4G6F7H5G5H3",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3D1F2F3G4F1G2G6F7H4G5H5",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3D1F2F3G4F1G2G6F7H4H3G5",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3D1F2F3G4G6F7H3E2E1F1D8",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3D1F2F3G4G6F7H3E2F1H4H5",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3D1F2F3G4G6F7H3H4H5E2E1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3D1F2F3G4G6F7H3H4H5E2G5",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3D1F2F3G4G6F7H3H4H5H6E1",
        "F5D6C3D3C4F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2G3D1F2F3G4G6F7H3H4H5H6G5",
        "F5D6C4D3C3F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2G3F3F2D1G4H3C1B1E2G6",
        "F5D6C4D3C3F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7F3F2D2",
        "F5D6C4D3C3F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7C8F7D8E8D1G3D2E2B6G4F8G6",
        "F5D6C4D3C3F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D1G3E2D2F3E1F1C1B1F2B6G5G1G4D8",
        "F5D6C4D3C3F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D1C1E2D2F3E1D8F8B6G6E8F2H6F7G5",
        "F5D6C4D3C3F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D1C1E2D2F3E1D8F8B6G6F2F7E8G4F1",
        "F5D6C4D3C3F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D1C1E2D2F3E1D8F8B6G6E8F7F2G4F1",
        "F5D6C4D3C3F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D1C1E2D2F3E1D8F8B6G6E8F7F2",
        "F5D6C4D3C3F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7B6F7E8G3G4G6G5F3D2H5H4H3",
        "F5D6C4D3C3F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7F3A3A2F2D2",
        "F5D6C4D3C3F4C5B3C2B4E3E6C6F6A5A4B5A6D7C7E7D2A3A2F2F3",
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

            let mut use_zhoseki = 0;
        
            let mut zhoseki_2: Vec<&str>;

            if id == 0{
                zhoseki_2 = [
                  "F5D6C3F4F6D3C4G5E6C5F3F7E3G4H5E7H6G3G6H3H4H7B6B4F8C2C7C6D7F2B5A5E2F1D1D2A3A4A6D8C8B8B3B1G1H1C1E1G2B2H2A7H8E8A1B7G8G7A8A2",
                    "F5D6C3D3C4F4F6F3E6E7D7G6G5C5B6C6B5C7F8B4D2A5E3E2A3F2C2A6A4A2B3E1G3C1H6H3E8H5B8G4B2F7G8D8C8H7H4B7A8B1A1A7D1F1G1H8H2G2H1G7",
                    "F5D6C3D3C4F4F6F3E6E7D7G6G5C5C6F7E2C7F8E8C8G8H6D2G3H4H5H3G4E3F2B6C1H7A5A6B7B4D8A4B3A8H8B5G7G2C2G1F1E1A3A2D1B1H2H1B2A7B8A1",
                    "F5D6C3D3C4F4F6G5E6D7E3C5F3E7B6B5C6B4C7B3A6C2D8A5F8E8A4A3A2E2G6F7C8H6F2G3G4H3D1C1F1D2E1B7G7H7H8G8H5A7A8B8H4G1H2B2A1B1H1G2",
                    "F5D6C3D3C4F4F6B4F3E6E3G5G6G4H4H5H6G3H3F7E8C2C5F8D7F2F1E2E7C6G8C8E1G2H1C7D2D1C1G7A4H2D8H8H7B5G1A5A6B6A3B3A2B2B1A1B8A8B7",
                    "F5F6E6F4G5E7F7D6F3G6D8H5E3D7C3D3C4C5E2G3H3B3C8E8G4H4C7H2G8D2F8C6D1F2G1E1F1B2B6C2A1A2B4B5A3A4A5B7H7G2G7H6H1H8A8B8C1B1A6A7",
                    "F5D6C3D3C4F4C5B4C6B5E6B6B3C2A5G6F3E7A6F7E3A4D7D2F6E2F2G3A3G4E8F1F8C7D8G5H6C8H5G8H4H3G7H8H7B7A2B2A1B1C1E1B8A8A7D1G2H1G1H2",
                    "F5D6C5F4D3G5E6F6G3F3E3E2G4F2C7H4D2C1E1H3C2B5C6C4H5H6B6C8E7C3B3A3A6A5A4A7B4B2G1F1A1F7D1H1B1E8D7A2G6H7D8F8A8G7B8G2B7H2H8G8",
                    "F5D6C5F4E3C6D3E2F3C3C4D2D1G4E6G3E7F1B6G6G5H6H4F6H5H3F7F8F2E1G1E8D7C8C1C2B4B2G2B3B5H1H2C7A2A3A5A1B1A6G7A4G8H8H7A7A8D8B8B7",
                    "F5D6C5F4E3C6D3F3E6F6G4G3G5H5H4H3G6H6D7C8E7F7C7B6E8F8D8C4F2C2D2B7A6A7A8B8A5B5A4C3B3F1C1B4A3B2A1E1A2B1E2D1G1G2H1H2H7G7H8G8",
                    "F5D6C3D3C4F4C5B3C2B4E3E6C6F6D7B5G3C7A5A4A6B6A3C8E7F3E8B7G4G5H5F2D2C1A8B8E1G6D1F1E2B1G2D8F8F7A7H1G1H2H6G7H4H3H7H8G8B2A2A1",
                    "F5D6C5F4E3C6D3F6E6C3C4D7E7C7D8C8F7F8G3D2G4G6H6E8G5F3B6B3B4A6C2A5D1G7B7B5F2E2E1C1B1G1A3H7H8G8B8H5H4G2A4A8A7A2B2F1H1A1H3H2",
                    "F5D6C5F4E3C6D3F6E6D7E7C7C4F3D8C8B8B4B5A5F7E8F8B3B6C2D2C3G6H7H5G8H8H6C1B2E2E1D1F1G1A7A6A4G3G4H4G2G7G5F2H3A8B1A1B7H2H1A3A2",
                    "F5D6C5F4E3C6D3F6E6D7E7C7C4F3D8C8B8E2B6E8F8F7G4G5H5H6H7C3G8H4H3G6G3B4B5A5F2D2E1F1C1D1G1A6B3C2A3H2H1G2A4A2B1B2A1B7A8G7H8A7",
                    "F5D6C3D3C4F4F6G5E3F3G3E2G4F2C6B6E6C5H4H6D7F7H5E7B5G6C7C8A6D2H7B3B4A3F8H2A4E8C1C2G8A5A2B2G7H3H1B8G1F1D1E1G2H8D8A7A8B7B1A1",
                    "F5F6E6F4G5E7E3F3C5H6G4C6D6B5D7G6C4H3H5C3H4D3F8F7F2G3A5B6B3A6C2D2B4F1A7C8D8A2C7A4B8E8G2E2D1H1G7H8H7G8G1E1H2C1B7A8A3B2A1B1",
                    "F5D6C3D3C4F4F6G5E3F3G3F2E6C6G6C5E2E7F7F8H5H3G4H6F1H4D7C8B4C7G7B5A6A5A4D2E1C1G8D1B1G2D8E8H1A3B6A7C2G1H2A1B7B2B8H8A2A8B3H7",
                    "F5D6C3D3C4F4F6F3E6E7D7C5B6B5E3G5G3F7G4D8H6D2G6C2F8H5H4A7C6E2E1F1G1F2D1H2A5C7H3H7C8E8A6A4B7C1B1B2G2G7H8G8H1B8A2A1A3A8B4B3",
                    "F5D6C3D3C4F4E3F3G4E2D2G6G3F2F1C5G5E1D1H6C6B5D7B4B3E7F6B6C7H3H5H4F8E6G1C2C1C8E8B7A6A4A3A2G7B8H7H8A8D8A7A5F7G8A1B1B2H1G2H2",
                    "F5F6E6F4E3C5G5G3G4F3C4D6E2H5H3D2C7D3H6H7F2C3E7D7G6E1C8E8D8B8C6F8F7B5C1C2B4B3F1G1B6G8A5A4B7D1H1A8H4G2H8H2G7A7A3A2A1B2B1A6",
                    "F5D6C5F4D3E3G4G3E6E7F6C6E2G6F3H3G5H4F7E8H6B5F8G8H5H7C7C4D8C8D7C2D2E1D1C1F2C3G2G7B6A6H8H1H2G1F1B8B7A8A7A5A4A3B4B3B2A1B1A2",
                    "F5D6C3D3C4F4E3F6C6B6G5H4H6G3E6F3G4H5F7E2D2B4C5E7E8D7G6D8C8F8G8F2E1C1C2B3B5A5H3F1D1G7A3A4A6C7H7G2G1H1B8B7H8H2B2A2A7A8B1A1",
                    "F5D6C3D3C4F4C5B3C2E3D2C6F2E6B4A3F7F6G4G3H4F8A5B5B6A6A4E2F3D1C1F1G5G6H7H2G7B1E1H5A1A2B2H3G1G2H6H8C7E7D8D7G8C8E8B7A7A8B8H1",
                    "F5F6E6F4G5D6E7G6C5C4F3H5F7G4H6H7D3C3E3F2H3E8F8D2G2G7H4B6G3C2H8H1D8D7C8H2G1G8F1E2E1C7C6D1A6A7A8B8B7B5B3B2B4A5A4A3B1A1A2C1",
                    "F5D6C3D3C4F4C5B4B3C2D7C6E6B5A5A3B6A7C7C8E2F7E3D2F3D8E7F8A4F6F2G6G5E1A2H4H5H6C1G4D1F1G1G2H1B2E8A6A8G3H3H2B8B7G8G7H7H8B1A1",
                    "F5D6C4D3C3F4F6F3E6E7F7C5E3E2G4D2F2G6C7G5G3C6H6B5B4H4F1C2H5H7D8D1D7H3B3E1C1A4A6A5A3F8G1B6A7B1G2C8B8H1H2B2H8E8G8G7A1A2B7A8",
                    "F5F6E6F4G5E7E3G6F3D6C5C4C6D3F7H5C7E8D8B5B6D7F8G4D2C8B8C2B3C3A5A3E2B4G3A4A6A7H6G7H4D1H8G8H7E1B2A8B7H2H3A1C1F2H1G2A2B1G1F1",
                    "F5D6C3D3C4F4C5B3C2E3D2B4C6E6B5C1E1A5A6B6A3A7F6G4F2E2F7D1B1G1D7E8D8E7C7C8F8G8A4A2B7G6F3G3G2B2A1F1H3H4H5G5H2B8A8H1H8H7H6G7",
                ].iter().cloned().collect();

                zhoseki_2.extend(zhoseki.clone());
            }else{
                zhoseki_2 = [
                    "F5D6C5F4D7F6F3E3G3G5D3E2F2G4F1H3G6H6H5H4H2H1F7G2H7H8G1E1C6C7G7F8E6E7E8D8D2B6",                    "F5D6C5F4D3E3G4G3E6C4B4F6E7F3E2F2G6G5F1D1E1G1H3H5H4H2D2C7D7H6G2C3C2H1B8C8D8C6B6B2A1H7B1F7G7A4B3H8A3B5A5G8F8E8A8B7C1A2A6A7",                    
                    "F5D6C5F4E3C6F6D3F3C4B4C3C2D2B3E2E6B5D7A4C1E1A3B6C7A2F1D1A6F2B1A1B2G1G2H2H1H3H4G3G4F7G6A5F8G7G8H7H5H8H6E7E8D8B8G5C8A7A8B7",                    
                    "F5D6C3D3E3F3C5D2C2F4E2B5E6F2C6E1C4F6G3G4G5E7F1G1E8H5G6H4A5D1H3H2G2B6C7F7G7H1B4A6B3C1A4B1A7F8G8C8D7B7D8H6B2A1B8A3A2A8H7H8",                    
                    "F5D6C5F4E3C6E6F7F3C4E7F8D7E8B5D3B3B6D8C8C7F6A5A7B7B8G6G4H4E2C2D2C3A8C1A6D1A4B4H6G7G5H7H5G8H8B2A2F1E1A3G1A1H3B1F2G2H1",                    
                    "F5D6C4B3C6B6D7E8B4B5A4E6C5A3E7A5C8F7C7F8C3D8F6B8A7D2C2D1E1F1C1B1B2A2G8H8D3G7E3E2A1F2A6F4G1G2H2B7A8H7F3G4H4G3H3G5H6H5G6H1",                    
                    "F5F6E6F4G5G6G4E7F7F8H6H4H5H7E8D8H3H2G3F2F3E2F1E3D2D1C5C4E1G1D3C2C3C6",                    
                    "F5F6E6F4E3C5G5F3G4G6E7H3C4H6H5H4C3F7C6D8G8F8E8H8G3D7C8F2E2B8F1C7G2D6G7H7D1D2B5A5H2D3H1G1E1B6A6A7C1B4B3C2B2A3A4A1B1B7A8A2",                    
                    "F5F6C4G5G6F4E6F7E8H6H5H4G3F3G4E3F2F1E1D1E2H3G7H2G2D3D2H1G1C1C2B2B1A1A2C3B3A3C5B4A4H7H8D6F8G8D8",                    
                    "F5F6E6F4E3C5C6D6E7D7C3B5B6B4C7F3A3E2G5D3F2C4D2A4A6A5B3F8E8B8C8D8A7A8B7B2A1A2B1C1C2F7G6H7G4G3H5H6H8G7H4G8H3H2G1F1H1G2E1D1",                    
                    "F5F6D3G5G6F4G4E3D6E6D7F3G3H5H6H7E7H4H3H2G2H1F2E1F1G1G7E2D2C1D1C2B1A1B2C3C4B3C5B4A2H8G8F8F7A3C7",                    
                    "F5F6E6F4G5G6G4E7E3F3G3D6F7H5H6H7D8H4C3C4C7C6B3C5B6D3H3H2C2F8G7E2E1E8G8B4A5H8B5C8F2D2D1A4A3B1C1F1G2D7A7B7B8A8A6H1G1B2A1A2",                    
                    "F5D6C3D3C4F4F6F3E6E7D7G6D8F7G5E8D2H4H5H6F8G4E3C5B4E2E1C2H3H2G3C8C6G8G7H7B1C1D1F2F1B5A6H8B6A4A5A7B3C7B7G2H1G1B8A8A1A3B2A2",                    
                    "F5D6C5F4D7F6G3E6G5C6C3F3C7D3G4G6E7H4H6H3F2E2C4B3D2E3H5C1D1H7G2B4C2B5A3A4A6H2B1F7A5H1F8G1F1E1A1B2A2A7B6B7A8B8D8C8G7G8E8H8",                    
                    "F5D6C3G5F6F4E6E7G3F7G6G4H6E3D8F3D2E2C6C4H3C7F2C5E8H5H4D7B3B4A5A3B5C8B8D3C2G8A2A1G7C1E1D1B1B6A7F1B7B2H8H7A4A6A8G1F8G2H2H1",                    
                    "F5D6C3D3C5F4F6C4F3E6E3B5A6D2C6B6A7B4C2D7E1D1C1B3A4A5D8A8A3A2B2G4G5A1B1F1H5E2F2G1B7B8C7G3G2H1H2H4H3H6G6C8E7F7E8",                    
                    "F5D6C3D3C4F4C5B3C2B4E3E6C6G4A5A4B5F6D7C7G5C8D8A6B6E8A3A2B7E7H4C1G6F7F8G8G7H8H7H5H6D2D1E1B1A1E2G3F2H3F3F1G2G1B2B8H2H1A7A8",                    
                    "F5D6C5F4D3E3F2C6E6F7G5B5B6C4E7F8D7E8C8C7B3A6B4A3A5A4B8D8G8F6A7A8B7H8G7H7H5H4H3G6F3G3H6G4F1E2D1E1H2G2H1G1D2C3C1B1C2B2A1A2",                    
                    "F5F6F7E3F3F4D3C4C5E6D6C3B3B5A6B4A5A3C6E2F2C2E1D2D1G5H5G3H4H3H2G4G2H6H7F1G1C1B1H1G6A1B2A2A4A7B6B7C7C8B8A8D8H8G7E8F8G8D7E7",                    
                    "F5F6E6F4G6C5E3D6F3G5G4E7C7C6C4B6A6H5E8H3H4G3G2H6H7H8H1F2H2G1E1F1E2D3D1C1B1G7F7F8G8C2D2C3B7B8D7B5D8C8A8A7A5",                    
                    "F5D6C5F4F3E3C7C6C3D7E7E6F6D8C8F8C4D3C2D2B5B4C1A5B6E1D1B1A3A4B3A2F7B8B7G8B2A8G7E8A7H7H8A6H6G3E2G6G5F1F2A1H3H5H4G4G1G2H2H1",                    
                    "F5D6C6F4E6C5C4B5F3E3B4D3B6A6B3C7A4A3C3A5C8D7F6E7D8A2A7A8B7B8F7E8F8G8G7H8G5G6H6H4H5H7G4G3F2H3H2F1E2E1D2C2C1D1G1B1A1B2G2H1",                    
                    "F5D6C3D3C4F4F6G5G4E3D2C5E6C6B5B4G3E2F3F2D1C1E1F1C2G1A5A3H6H3D7H4B3B1G2H5B6A2H2H1G6A7A6A4B2H7G7G8H8F8E8F7C8D8E7B8C7B7A8A1",                    
                    "F5F6E6F4F3C5C4E3D3D6G4H3H5G3G5E2C6H6G6H4D1F2G1H7G7H8E7F7F8D8D7C8C7G8H2F1E1E8G2H1C1D2C3B8B3B4A4C2B2A2B5B6A7B7A8B1A1A6A5A3",                    
                    "F5D6C3D3E3F3G3E6F6F4G5G6D7H5G4H4H7E7E8C5H3H6F7G8C6H8C4G7F8C7B8D8C8A8B7B6A7A6B5B4A5A4H2B3",                    
                    "F5F6E6F4E3C5C4E7C6D2G5G4G6D6G3H3F2F3D7C8H4H5E8H6C1E1F7D3E2D1G1F1G2C7B7B8H7H1H2B1H8C2C3B3A2A3A4G8F8D8G7B4B6A6A7A8B5A5B2A1",                    
                    "F5D6C4D3C5F6E3C3E6B5B4A5C6C7A3D7E7B6C8A4A6E8F7F8B8G8C2B3G7H8G5D8F4A8H7A7A2B7A1H6B2G6H5H4G4F3H3B1C1D1E2D2E1F1G1F2G3G2",                    
                    "F5D6C5F4E3C6D3G5E6F6G3C4B4A3B5A4A6B6D7F3E7C7G4B3C3H4H6H5F2H7G6E8F8G8A7E2H3H2D1C2D2F1F7D8B8C1A5A8C8B7E1G1H8G7H1G2B1B2A2A1",                    
                    "F5D6C4D3C5F4E3C6E6F3D7E7F6E8C7B8C2G4F7G5D8C8F8G8G6H5H6H7H3B3B4G3A4B6C3D2B5A5A3A2B2D1C1B1A1G7H2G2H1F2H4G1E1E2H8F1A8A6B7A7",                    
                    "F5D6C4D3C5B6B5F4E3F3D2F6D7C1G6G5E2D1F1F2C6C2E6F7E1G1C3H5A6B4A3E7F8C8D8B7C7A4A5A7B3E8A8G8G7H6H8B8H7G4H4H3G3G2H1H2B1B2A2A1",                    
                    "F5D6C4F4C5D3E6B5E3C6D7G5F6C7B4C3G6A4D2F2E2D8F3E7F7G4E8F8B3E1C2C1F1D1B1A3H6G3A6H4H5H3A5A7B6G2H1G1H2A1B7G7B8C8A8B2A2H7H8G8",                    
                    "F5D6C5F4E3C4E6C6D3F3G4F7E7D2G3F8C2F6C1E2D1G5H6F2E1G1C3H5H4H2G6B3D8D7B2H3F1H7H1G2H8A2B1B4A5B5A1G7A3G8E8C7B6A7B7A8B8C8A6A4",                    
                    "F5D6C5F4E3C6D3F3E6F7G4C4E7F8D7G5F6H5E8G6C7D8C8B8H4H3G3H2G7D2H6H8H1F2G2H7G8B6B5A5B4A8A6A7B7A3A4B3C2B2A2C1D1G1C3A1B1E2E1",                    
                    "F5D6C5F4D7F6G5E6G6C6B6E7F8G4H3E8F7C8C7G8G3H6F3D8E3B8G7F2E2H8E1H7C3C4H5C2A6B7H4B5F1G1A8A4A5G2A7D3H2H1A3B4D2B2C1A1D1B1A2",                    
                    "F5F6E6F4G5E7F7D6C5G6C3C6D8H6H4D3F3H5H7E3B6F8C4F2E2C2D2B5A4E1C1E8G8D7G4G3D1H3H2C7F1A6B4A5A7B2B3A3A2A1B1A8B7B8C8G2G1H8G7H1",                    
                    "F5D6C5F4E3C6E6F7E7F6D7D8F8C8G3B5B6C7A4A6G5G6G8H4H6B4A3G4A5E8B8A2B7H5H3C3B3C2D2D3B1C4B2D1C1A1G7H7H8A8A7F3E2E1F2F1G2H2G1H1",                                
                ].iter().cloned().collect();

                zhoseki_2.extend(zhoseki.clone());
            }

            let mut bitboard:OthelloBoard = OthelloBoard{
                black: 0,
                white: 0,
            };

            let mut ikkomae:OthelloBoard = OthelloBoard::new();
            let mut tedaze:Vec<char>;
            let mut tezyun: Vec<char>;
            let mut zyoseki_mode = 0;


            for _ in 0..board_size as usize {
                let mut input_line = String::new();
                io::stdin().read_line(&mut input_line).unwrap();
                let line = input_line.trim().to_string();
                 // rows from top to bottom (viewer perspective).
    
                 let chars: Vec<char> = line
                 .trim()
                 .chars()
                 .collect();
    
                 for i in chars.iter(){
                    bitboard.black >>= 1;
                    bitboard.white >>= 1;
                    if *i == '0' {
                        bitboard.black |= 0x8000000000000000;
                    }else if *i == '1'{
                        bitboard.white |= 0x8000000000000000;
                    }
    
                 }
            }
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let action_count = parse_input!(input_line, i32); // number of legal actions for this turn.
            for _ in 0..action_count as usize {
                let mut input_line = String::new();
                io::stdin().read_line(&mut input_line).unwrap();
                //let action = input_line.trim().to_string(); // the action
            }

            if id == 0{
                println!("F5");
                tezyun = vec!['F','5'];
                ikkomae = ikkomae.add2(1<<37, id as usize);
                
            }else{

                let tugi = ikkomae.make_1_next_boards_tezyun(vec![],aite as usize);

                if let Some(matching_second_arg) = tugi.iter().find(|(first_arg, _)| (first_arg.white == bitboard.white) && (first_arg.black == bitboard.black)) {
                    tedaze = matching_second_arg.1.clone();
                } else {
                    use_zhoseki = 1;
                    tedaze = vec![];
                }

                if tedaze[0] == 'E' && tedaze[1] == '6'{
                    zyoseki_mode = 1;
                }else if tedaze[0] == 'D' && tedaze[1] == '3'{
                    zyoseki_mode = 2;
                }else if tedaze[0] == 'C' && tedaze[1] == '4'{
                    zyoseki_mode = 3;
                }else{
                    zyoseki_mode = 0;
                }
                tezyun = vec!['F','5'];

                let check: String = tezyun.clone().into_iter().collect();
                let mut matches: Vec<&str> = zhoseki_2
                            .iter()
                            .filter(|item| item.starts_with(&check))
                            .cloned()
                            .collect();
            
                let length = check.len();
                matches.sort_by_key(|s| s.len());

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

                // println!("{}{}",te[0],te[1]);

                let getf: Vec<usize> = match (te).as_slice() {
                    [ch @ 'A'..= 'H', num @ '1'..='8'] => vec![(*ch as u8 - b'A') as usize, (*num as u8 - b'1') as usize],
                    _ => panic!("Invalid input format"),
                };
                let bit = getf[0] + getf[1]*8;

                let yoko = bit % 8;
                let tate = (bit / 8) + 1;
                let mapping = vec!['a','b','c','d','e','f','g','h'];
        
                let getc: Vec<String> = vec![mapping[yoko].to_string(), tate.to_string()];
                // Write an action using println!("message...");
                // To debug: eprintln!("Debug message...");
        
                println!("{}{}",getc[0],getc[1]); // a-h1-8

                ikkomae = bitboard.add2(1 << bit, id as usize);
            }

    let mut tugi = ikkomae.make_1_next_boards_tezyun(vec![],aite as usize);

    // game loop
    loop {

        let mut bitboard:OthelloBoard = OthelloBoard{
            black: 0,
            white: 0,
        };
        for _ in 0..board_size as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let line = input_line.trim().to_string();
             // rows from top to bottom (viewer perspective).

             let chars: Vec<char> = line
             .trim()
             .chars()
             .collect();

             for i in chars.iter(){
                bitboard.black >>= 1;
                bitboard.white >>= 1;
                if *i == '0' {
                    bitboard.black |= 0x8000000000000000;
                }else if *i == '1'{
                    bitboard.white |= 0x8000000000000000;
                }

             }
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let action_count = parse_input!(input_line, i32); // number of legal actions for this turn.
        for _ in 0..action_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            //let action = input_line.trim().to_string(); // the action
        }

        let start_time = Instant::now();
        let timeout_for_final = Duration::from_millis(140 as u64);

        let count = count_pieces(bitboard.black | bitboard.white);

        let bit:usize;

        if use_zhoseki != 0{
            bit = bitboard.eval_boards(id as usize, count as usize, &mut n, start_time, timeout_for_final, &mut memo_banmen,&memo_banmen_pre,&mut memo_yomikiri, &mut memo_eval, &memo_eval_pre);
        }else{
            if let Some(matching_second_arg) = tugi.iter().find(|(first_arg, _)| (first_arg.white == bitboard.white) && (first_arg.black == bitboard.black)) {
                tedaze = matching_second_arg.1.clone();
            } else {
                use_zhoseki = 1;
                tedaze = vec![];
            }
    
            for chunk in tedaze.chunks(2){
                let henkan: Vec<char>;
                let chunk2 = chunk.to_vec();

                if zyoseki_mode == 1{
                    henkan = match (chunk2).as_slice() {
                        [ch @ 'A'..= 'H', num @ '1'..='8'] => vec![(*num as u8 - b'1' + b'A') as char, (*ch as u8 - b'A' + b'1') as char],
                        _ => panic!("Invalid input format"),
                    };
            
                } else if zyoseki_mode == 2{
            
                    henkan = match (chunk2).as_slice() {
                        [ch @ 'A'..= 'H', num @ '1'..='8'] => vec![(b'8' - *num as u8 + b'A') as char, (b'H' - *ch as u8 + b'1') as char],
                        _ => panic!("Invalid input format"),
                    };
                    
                }else if zyoseki_mode == 3{
            
                    henkan = match (chunk2).as_slice() {
                        [ch @ 'A'..= 'H', num @ '1'..='8'] => vec![(b'H' - *ch as u8  + b'A') as char, (b'8' - *num as u8 + b'1') as char],
                        _ => panic!("Invalid input format"),
                    };
                }else{
                    henkan = chunk2;
                }
                tezyun.extend(henkan);
            }

            let check: String = tezyun.clone().into_iter().collect();
            let mut matches: Vec<&str> = zhoseki_2
                            .iter()
                            .filter(|item| item.starts_with(&check))
                            .cloned()
                            .collect();
            
            let length = check.len();
            matches.sort_by_key(|s| s.len());

            if matches.is_empty() || (matches.len() == 1 && matches[0].len() <= length){
                use_zhoseki = 1;
                bit = bitboard.eval_boards(id as usize, count as usize, &mut n, start_time, timeout_for_final,&mut memo_banmen,&memo_banmen_pre,&mut memo_yomikiri, &mut memo_eval, &memo_eval_pre);
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
                ikkomae = bitboard.add2(1 << bit, id as usize);
                tugi = ikkomae.make_1_next_boards_tezyun(vec![], aite as usize);
            }
        }

        // println!("{}",count);
        //println!("{}",bitboard.black);
         //println!("{}",bitboard.white);
        let yoko = bit % 8;
        let tate = (bit / 8) + 1;
        let mapping = vec!['a','b','c','d','e','f','g','h'];

        let getc: Vec<String> = vec![mapping[yoko].to_string(), tate.to_string()];
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("{}{}",getc[0],getc[1]); // a-h1-8

        memo_banmen_pre = memo_banmen;
        memo_eval_pre = memo_eval;
        memo_banmen = HashMap::new();
        memo_eval = HashMap::new();
        memo_yomikiri = HashMap::new();
    }
}
