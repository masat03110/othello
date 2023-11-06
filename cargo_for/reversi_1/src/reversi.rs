// use std::io;
// use std::process;
// use std::env;

// use core::num;

const FIRST:usize = 35;
const SECOND:usize = 45;

// ビットボードの定義
type Bitboard = u64;

// オセロ盤の状態を表す構造体
pub struct OthelloBoard {
    black: Bitboard,   // 黒石の位置を表すビットボード
    white: Bitboard,   // 白石の位置を表すビットボード
}

impl OthelloBoard {
    // 新しい空のオセロ盤を作成する関数
    pub fn new() -> OthelloBoard {
        OthelloBoard {
            black: (1 << 28) | (1 << 35),
            white: (1 << 27) | (1 << 36),
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

    fn check_right(&self, pos: usize, color: usize) -> usize{
        let mut pos_check = pos;
        let boards: Vec<u64> = vec![self.black, self.white];
        let mut get_pieces: usize = 0;

        loop {
            if (pos_check % 8) == 7 {
                return 0;
            }
            pos_check += 1;
            if (boards[1-color] & 1 << pos_check) != 0 {
                get_pieces += 1;
            }
            else if (boards[color] & 1 << pos_check) != 0 {
                return get_pieces;
            }
            else{
                return 0;
            }  
        }
    }

    fn check_left(&self, pos: usize, color: usize) -> usize{
        let mut pos_check = pos;
        let boards: Vec<u64> = vec![self.black, self.white];
        let mut get_pieces: usize = 0;

        loop {
            if (pos_check % 8) == 0 {
                return 0;
            }

            pos_check -= 1;
            if (boards[1-color] & 1 << pos_check) != 0 {
                get_pieces += 1;
            }
            else if (boards[color] & 1 << pos_check) != 0 {
                return get_pieces;
            }
            else{
                return 0;
            }
        }
    }

    fn check_up(&self, pos: usize, color: usize) -> usize{
        let mut pos_check = pos;
        let boards: Vec<u64> = vec![self.black, self.white];
        let mut get_pieces: usize = 0;

        loop {
            if pos_check < 8 {
                return 0;
            }

            pos_check -= 8;
            if (boards[1-color] & 1 << pos_check) != 0 {
                get_pieces += 1;
            }
            else if (boards[color] & 1 << pos_check) != 0 {
                return get_pieces;
            }
            else{
                return 0;
            }
        }
    }

    fn check_down(&self, pos: usize, color: usize) -> usize{
        let mut pos_check = pos;
        let boards: Vec<u64> = vec![self.black, self.white];
        let mut get_pieces: usize = 0;

        loop {
            if pos_check > 55 {
                return 0;
            }

            pos_check += 8;
            if (boards[1-color] & 1 << pos_check) != 0 {
                get_pieces += 1;
            }
            else if (boards[color] & 1 << pos_check) != 0 {
                return get_pieces;
            }
            else{
                return 0;
            }
        }
    }

    fn check_diagonally_upright(&self, pos: usize, color: usize) -> usize{
        let mut pos_check = pos;
        let boards: Vec<u64> = vec![self.black, self.white];
        let mut get_pieces: usize = 0;

        loop {
            if (pos_check % 8) == 7 || pos_check < 8{
                return 0;
            }

            pos_check -= 7;
            if (boards[1-color] & 1 << pos_check) != 0 {
                get_pieces += 1;
            }
            else if (boards[color] & 1 << pos_check) != 0 {
                return get_pieces;
            }
            else{
                return 0;
            }
        }
    }

    fn check_diagonally_upleft(&self, pos: usize, color: usize) -> usize{
        let mut pos_check = pos;
        let boards: Vec<u64> = vec![self.black, self.white];
        let mut get_pieces: usize = 0;

        loop {
            if (pos_check % 8) == 0 || pos_check < 8{
                return 0;
            }

            pos_check -= 9;
            if (boards[1-color] & 1 << pos_check) != 0 {
                get_pieces += 1;
            }
            else if (boards[color] & 1 << pos_check) != 0 {
                return get_pieces;
            }
            else{
                return 0;
            }
        }
    }

    fn check_diagonally_downright(&self, pos: usize, color: usize) -> usize{
        let mut pos_check = pos;
        let boards: Vec<u64> = vec![self.black, self.white];
        let mut get_pieces: usize = 0;

        loop {
            if (pos_check % 8) == 7 || pos_check > 55{
                return 0;
            }

            pos_check += 9;
            if (boards[1-color] & 1 << pos_check) != 0 {
                get_pieces += 1;
            }
            else if (boards[color] & 1 << pos_check) != 0 {
                return get_pieces;
            }
            else{
                return 0;
            }
        }
    }

    fn check_diagonally_downleft(&self, pos: usize, color: usize) -> usize{
        let mut pos_check = pos;
        let boards: Vec<u64> = vec![self.black, self.white];
        let mut get_pieces: usize = 0;

        loop {
            if (pos_check % 8) == 0 || pos_check > 55{
                return 0;
            }

            pos_check += 7;
            if (boards[1-color] & 1 << pos_check) != 0 {
                get_pieces += 1;
            }
            else if (boards[color] & 1 << pos_check) != 0 {
                return get_pieces;
            }
            else{
                return 0;
            }
        }
    }

    fn valid_total(&self, pos: usize, color: usize) -> usize{
        let total = self.check_right(pos, color) +
                    self.check_left(pos, color) +
                    self.check_up(pos, color) +
                    self.check_down(pos, color) +
                    self.check_diagonally_downleft(pos, color) +
                    self.check_diagonally_downright(pos, color) +
                    self.check_diagonally_upleft(pos, color) +
                    self.check_diagonally_upright(pos, color);
        return total;
    }

    pub fn valid_pieces(&self, color: usize) -> Vec<usize>{
         let mut pos: usize = 0;
         let mut getting_number: Vec<usize> = Vec::new();
         let mut valid_pos: Vec<usize> = Vec::new();
         let mut arrangements:usize = 0;
        loop {
            if self.black & (1 << pos) != 0 || self.white & (1 << pos) != 0 {
                getting_number.push(0);
            }
            else{
                let total = self.valid_total(pos, color);
                if total > 0{
                    arrangements += 1;
                    valid_pos.push(pos);
                }
                getting_number.push(total);
            }

            pos += 1;

            if pos > 63 {
                break;
            }
        }
        getting_number.push(arrangements);
        getting_number.extend(valid_pos);

        return getting_number;
    }

    fn change_pieces(&mut self, pos: usize, color: usize, mode: isize){
        // mode: 0-left 1-right 2-upleft 3-up 4-upright 5-downleft 6-down 7-downright
        let mut change_pos = pos; 
        let x = if mode < 2 {2*mode - 1} else if mode < 5 {mode - 11} else {mode + 2};
        if color == 0{
            loop {
                change_pos = ((change_pos as isize) + x) as usize;
                if (self.white & 1 << change_pos) != 0 {
                    self.white &= !(1 << change_pos);
                    self.black |= 1 << change_pos;
                }
                else{
                    break;
                }
            }
        }else{
            loop {
                change_pos = ((change_pos as isize) + x) as usize;
                if (self.black & 1 << change_pos) != 0 {
                    self.black &= !(1 << change_pos);
                    self.white |= 1 << change_pos;
                }
                else{
                    break;
                }
            }
        }
    }

    fn check_and_change(&mut self, bit: usize, color: usize){
        if self.check_left(bit,color) > 0{
            self.change_pieces(bit,color,0);
            }
        if self.check_right(bit,color) > 0{
            self.change_pieces(bit,color,1);
            }
        if self.check_diagonally_upleft(bit,color) > 0{
            self.change_pieces(bit,color,2);
            }
        if self.check_up(bit,color) > 0{
            self.change_pieces(bit,color,3);
            }
        if self.check_diagonally_upright(bit,color) > 0{
            self.change_pieces(bit,color,4);
            }
        if self.check_diagonally_downleft(bit,color) > 0{
            self.change_pieces(bit,color,5);
            }
        if self.check_down(bit,color) > 0{
            self.change_pieces(bit,color,6);
            }
        if self.check_diagonally_downright(bit,color) > 0{
            self.change_pieces(bit,color,7);
            }
    }

    pub fn add(&mut self, bit: usize, turn: &mut usize){
        if *turn == 0 { //black's turn
            self.black |= 1 << bit;

            self.check_and_change(bit,*turn);

            *turn = 1;
        }else{ // white's turn
            self.white |= 1 << bit;

            self.check_and_change(bit,*turn);
            
            *turn = 0;
        }
    }

    



    pub fn which_pos_mini_available(&self, color: usize, count: usize) -> usize{
        let mut turn = color;
        let board2 = self.clone();
        let valids_now = board2.valid_pieces(color);
        let slice = &valids_now[65..];
        let valid_pos: Vec<usize> = slice.to_vec();
        let mut mini_num:usize = 99999;
        let mut return_pos = valids_now[65];
    
        for item in valid_pos{
            let mut board3 = OthelloBoard {
                black: board2.black,
                white: board2.white,
            };
            board3.add(item, &mut turn);
            let valids = board3.valid_pieces(turn);
            let mut eval_num:usize;

            if valids[64] != 0{
                let slice = &valids[65..];
                let valid_pos: Vec<usize> = slice.to_vec();
                let mut max = 65;
                let mut min = 65;

                for item in valid_pos{
                    if valids[max] < valids[item]{
                        max = item;
                    }else if valids[min] > valids[item]{
                        min = item;
                    }
                }

                if count < FIRST{
                    eval_num = valids[64]*valids_now[item]/(valids[min]+valids[max]);

                    if item == 0 || item == 7 || item == 63 || item == 56{
                        eval_num /= 100;
                    }else if ((item % 8) < 2 && ((item < 16) || (item > 47))) || ((item % 8) > 5 && ((item < 16) || (item > 47))){
                        eval_num += 100;

                    }

                 }else if count < SECOND{

                    eval_num = valids[64]*valids_now[item]/(valids[min]+valids[max]);

                    if item == 0 || item == 7 || item == 63 || item == 56{
                        eval_num /= 100;
                    }else if ((item % 8) < 2 && ((item < 16) || (item > 47))) || ((item % 8) > 5 && ((item < 16) || (item > 47))){
                        eval_num += 100;

                    }
                }
                else{
                    if valids_now[item] <= valids[max]{
                        eval_num = 2 + valids[max] - valids_now[item];
                        if item == 0 || item == 7 || item == 63 || item == 56{
                        eval_num /= 100;
                        }else if ((item % 8) < 2 && ((item < 16) || (item > 47))) || ((item % 8) > 5 && ((item < 16) || (item > 47))){
                            eval_num += 100;

                        }

                    }else{
                        eval_num = 1/(valids_now[item] - valids[max]);

                        if item == 0 || item == 7 || item == 63 || item == 56{
                            eval_num /= 100;
                        }else if ((item % 8) < 2 && ((item < 16) || (item > 47))) || ((item % 8) > 5 && ((item < 16) || (item > 47))){
                            eval_num += 100;
    
                        }

                    }
                }
            }else{
                
                eval_num = (1/10000)*(1/valids_now[item]);
            }
            if eval_num < mini_num{
                mini_num = eval_num;
                return_pos = item;
            }
        }

        return_pos

    }


}

// pub fn eval_fun_for_start(&self, color: usize, count: usize, yomite: usize) -> usize{
//     let board2 = self.clone();
//     let valids = board2.valid_pieces(color);
//     let slice = &valids[65..];
//     let valid_pos: Vec<usize> = slice.to_vec();
//     let mut max_pos = valids[65];
//     let mut min_pos = valids[65];

//     for item in valid_pos{
//         if valids[max_pos] < valids[item]{
//             max_pos = item;
//         }else if valids[min_pos] > valids[item]{
//             min_pos = item;
//         }
//     }

//     if count > SWITCH{
//         max_pos
//     }else{
//         min_pos
//     }
// }

// fn print_vec_for_validboard(vec: &Vec<usize>)
//     {
//         let mut count:usize = 0;
//         for item in vec {
//             let x = count / 8;
//             if (count % 8) == 0{
//                 print!("{}:",x);
//             }
//             print!("{:?} ", item);

//             count += 1;

//             if (count % 8) == 0{
//                 println!();
//             }


//         }
//     }



// fn do_reversi(cpu:usize) {

//     let mut board = OthelloBoard::new();

//     let mut turn: usize = 0;

//     let mut passed: usize = 0;

//     let mut score: Vec<usize> = vec![2,2];

//     let mut counter: usize = 4;

//     loop {

//         board.print(turn);
//         let valid_board = board.valid_pieces(turn);

//         println!("(B,W):({},{})",score[0],score[1]);

//         if counter > 63{
//             if score[0] > score[1]{
//                 println!("Black win !!");
//             }else if score[0] < score[1]{
//                 println!("White win !!");
//             }else{
//                 println!("Draw");
//             }
//             process::exit(0);
//         }

//         if turn == 0{
//             println!("B's turn");
//         }else{
//             println!("W's turn");
//         }

//         loop{
//             if valid_board[64] == 0{
//                 println!("passed");
//                 turn = 1 - turn;
//                 passed += 1;
//                 if passed > 1{
//                     if score[0] > score[1]{
//                         println!("Black win!");
//                     }else if score[0] < score[1]{
//                         println!("White win!");
//                     }else{
//                         println!("Draw");
//                     }
//                     process::exit(0);
//                 }
//                 break;
//             }
//             passed = 0;

//             let bit;

//             if cpu == turn{
//                 let mut input = String::new();

//                 io::stdin().read_line(&mut input)
//                 .expect("Failed to read line");

//                 let chars: Vec<char> = input
//                     .trim()
//                     .chars()
//                     .collect();
//                     // .filter_map(|c| c.to_digit(10))
//                     // .filter_map(|c| Some(c as usize))
//                     // .collect();

//                 let getf: Vec<usize> = match chars.as_slice() {
//                     [ch @ 'a'..= 'h', num @ '1'..='8'] => vec![(ch.to_digit(36).unwrap() - 10) as usize, (num.to_digit(10).unwrap() - 1) as usize],
//                     _ => panic!("Invalid input format"),
//                 };

//                 bit = getf[0] + getf[1]*8;
//             }
//             else{
//                 bit = eval_fun(&valid_board, counter);
//             }

//             if valid_board[bit] == 0 {
//                 println!("you can't put a piece in the place");
//             }else{
//                 score[turn] += valid_board[bit] + 1;
//                 score[1-turn] -= valid_board[bit];
//                 counter += 1;

//                 board.add(bit,&mut turn);

//                 break;
//             }
//         }
//     }

// }

// fn main(){
//     let args: Vec<String> = env::args().collect();

//     if args.len() < 2 {
//         println!("Usage: ./reversi <1 or 2> (1:black,2:white)");
//         return;
//     }

//     let player: usize = match args[1].parse() {
//         Ok(n) => n,
//         Err(_) => {
//             println!("Invalid input");
//             return;
//         }
//     };

//     do_reversi(player - 1);

// }