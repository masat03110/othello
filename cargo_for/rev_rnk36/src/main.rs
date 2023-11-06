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
 
     pub fn make_1_next_boards2(&self, color: usize, n:usize) -> Vec<(OthelloBoard,u64,usize,isize)>{
         let board = OthelloBoard {
             black: self.black,
             white: self.white,
         };
 
         let mut valid:u64;
 
         if color == 0{
             valid = valids(self.black,self.white);
         }else{
             valid = valids(self.white,self.black);
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
 
             let mut times = 0;
             let next_board = board.add2(check, color);
 
             let score = next_board.eval_mini_max_sub(color, 1-color, &mut times, n,1);
             //println!("score:{}",score);
             nextboards.push((next_board, check ,1-color , score));
 
             nokori &= !(check);
             check = nokori;
         }
 
         nextboards.sort_by(|a, b| b.3.cmp(&a.3));
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
         let mut nextboards_sub2:Vec<(OthelloBoard,isize)> = vec![];
 
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
 
             if valid & 0x8100000000000081 != 0{
                 nextboards_sub2.push((next_board,count_pieces(valid)));
             }else{
                 nextboards_sub.push((next_board,count_pieces(valid)));
             }
             
             nokori &= !(check);
             check = nokori;
         }
 
         if nextboards_sub.is_empty(){
             nextboards_sub = nextboards_sub2;
         }
 
         nextboards_sub.sort_by(|a, b| a.1.cmp(&b.1)); //降順
 
         let mut first_elements: Vec<_> = nextboards_sub.iter().map(|(first, _)| OthelloBoard { black: first.black, white: first.white }).collect();
         first_elements.push(OthelloBoard{ black: 0, white: (0b1 ^ color2) as u64, });
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
         let boards = self.make_1_next_boards2(color, n);
         let mut position:u64 = 0;
         let mut max:isize = isize::MIN;
         let mut now_count = 0;
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
 
     fn eval_mini_max_sub(&self, color: usize, turn: usize, times:&mut u64, n:usize, alpha:isize)->isize{
         *times += 1;
         if *times > 500000000{
             return 0;
         }
 
         let mut boards = self.make_1_next_boards1_5(turn);
         if boards.is_empty() || n == 0{
             return self.eval_board_for_last(color);
         }
 
         let mut return_count:isize = 0;
         let mut now_count:isize;
         let mut beta = 1;
 
         if let Some(OthelloBoard{ black: _, white: check}) = boards.pop() {
 
             if 1-check == color as u64{
                 return_count = isize::MIN;
                 for board in boards {
                         now_count = board.eval_mini_max_sub(color, 1-color, times, n-1,beta);
                         
                         if return_count < now_count {
                             if alpha == 1 || alpha > now_count{
                                 return_count = now_count;  
                                 beta = now_count;
                             }else{
                                 return now_count;
                             }
                         }
                        
                 }
 
             } else{
                 return_count = isize::MAX;
                 for board in boards {
                         now_count = board.eval_mini_max_sub(color, color, times, n-1,beta);
                         
                         if return_count > now_count {
                             if alpha == 1 || alpha < now_count{
                                 return_count = now_count;  
                                 beta = now_count;
                             }else{
                                 return now_count;
                             }
                              
                         }
                 }
             }
         }
 
         return_count
     }
 
     fn eval_mini_max(&self, color: usize, n:usize)-> usize{
         let boards = self.make_1_next_boards2(color, n);
 
         let (_,pos,_,_) = boards[0];
         let mut position = pos >> 1;
         position |= position >> 1;
         position |= position >> 2;
         position |= position >> 4;
         position |= position >> 8;
         position |= position >> 16;
         position |= position >> 32;
 
         return count_pieces(position) as usize;
     }
 
     pub fn eval_boards(&self, color: usize, count: usize, n: &mut usize) -> usize{
         let start_time = Instant::now();
         let timeout_for_final = Duration::from_millis(150 as u64);
         if count > 45{//montecalro max 51
                       //a-b max 43 43が1手読める程度
                       //44-45が割れ目か　念のため44-46で計測する必要あり
                       //43-41は不確定切りが必要か
             return self.last_mini_max(color,start_time,timeout_for_final,count, *n);
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
             return self.eval_mini_max(color, *n);
         }
         
     }
 
 }

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let id = parse_input!(input_line, i32); // id of your player.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let board_size = parse_input!(input_line, i32);
    let mut n = 5;

    // game loop
    loop {
        let mut bitboard:OthelloBoard = OthelloBoard{
            black: 0,
            white: 0,
        };
        for i in 0..board_size as usize {
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
        for i in 0..action_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let action = input_line.trim().to_string(); // the action
        }

        let count = count_pieces(bitboard.black | bitboard.white);

        // println!("{}",count);
        //println!("{}",bitboard.black);
         //println!("{}",bitboard.white);

        let bit = bitboard.eval_boards(id as usize, count as usize, &mut n);

        let yoko = bit % 8;
        let tate = (bit / 8) + 1;
        let mapping = vec!["a","b","c","d","e","f","g","h"];

        let getc: Vec<String> = vec![(mapping[yoko].to_string()), tate.to_string()];

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("{}{}",getc[0],getc[1]); // a-h1-8
    }
}
