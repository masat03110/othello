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
    result &= (0 - count_pieces((result << 1) & myboard)) as u64;
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
    result &= (0 - count_pieces((result >> 1) & myboard)) as u64;
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
    result &= (0 - count_pieces((result >> 8) & myboard)) as u64;
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
    result &= (0 - count_pieces((result << 8) & myboard)) as u64;
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
    result &= (0 - count_pieces((result >> 7) & myboard)) as u64;
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
    result &= (0 - count_pieces((result >> 9) & myboard)) as u64;
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
    result &= (0 - count_pieces((result << 9) & myboard)) as u64;
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
    result &= (0 - count_pieces((result << 7) & myboard)) as u64;
    result
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

    pub fn eval_good_pos(&self, color:usize) -> usize{
        let myboard:u64;
        let opponent:u64;
        let mut eval_points:usize = 100;
        if color == 0{
            // color is black
            myboard = self.black;
            opponent = self.white;            
        }else{
            // color is white
            myboard = self.white;
            opponent = self.black;
        }

        let akikoma = !(myboard | opponent);

        count_pieces(opponent & !valid_board(myboard, opponent, akikoma));

        count_pieces(myboard & !valid_board(myboard, opponent, akikoma));

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
        //for humans
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

    pub fn make_1_next_boards_for_last(&self, color: usize, flag: u8) -> Vec<(OthelloBoard,u64,usize)>{
        let board = OthelloBoard {
            black: self.black,
            white: self.white,
        };

        let akikoma = !(self.white | self.black);

        if akikoma == 0 {
            return vec![(board,0,color)];
        }

        let mut position:u64 = 1;
        let mut nextboards:Vec<(OthelloBoard,u64,usize)> = vec![];
        let mut flag2:u8 = 0;
        loop{
            while position & akikoma == 0 {
                if position == 0 {
                    break;
                }
                position <<= 1;
            }

            if position == 0 {
                break;
            }
            
            if self.valid_total(position, color) != 0{
                flag2 = 1;
                nextboards.push((board.add2(position, color),position,1-color));
            }
            position <<= 1;
        }

        if flag2 != 0{
            return nextboards;
        }else if flag != 0{
            return vec![(board,0,color)];
        }else{
            return board.make_1_next_boards_for_last(1-color, 1);
        }
    }

    fn last_mini_max_sub(&self, color: usize, turn: usize, count:usize, times:&mut u64)->isize{
        *times += 1;
        if *times > 10000000 {
            return -30;
        }
        let boards = self.make_1_next_boards_for_last(turn,0);
        let mut return_count:isize = -99;
        let mut now_count:isize;

        for (board, position, now_turn) in boards {
            if position == 0 {
                return self.subtract(color);
            }
            else if 1-now_turn == color{
                now_count = board.last_mini_max_sub(color, now_turn,count, times);
                if return_count == -99{
                    return_count = now_count;
                }else if return_count < now_count {
                    return_count = now_count;
                }   
            }else{
                now_count = board.last_mini_max_sub(color, now_turn,count, times);
                if return_count == -99{
                    return_count = now_count;
                }
                else if return_count > now_count {
                    return_count = now_count;
                }
            }
        }

        return return_count;
    }

    fn last_mini_max(&self, color: usize, count: usize)-> usize{
        let boards = self.make_1_next_boards_for_last(color,0);
        let mut position:u64 = 0;
        let mut return_pos:usize = 0;
        let mut max:isize = -999;
        let mut now_count;
        let mut times:u64 = 0;
        for (board, pos, turn) in boards {
            now_count = board.last_mini_max_sub(color, turn ,count, &mut times);
            if now_count > 0{
                position = pos;
                break;
            }
            if max < now_count{
                max = now_count;
                // return_pos = pos;
                position = pos;
            }
            println!("times:{}",times);
        }

        while position != 1{
            return_pos += 1;
            position >>= 1;
        }
        return return_pos;
    }



    pub fn eval_boards(&self, color: usize, count: usize) -> usize{
        if count > 39{//montecalro max 51
            return self.last_mini_max(color, count);
        }else if count > 30{
            return self.mid_mini_max(color, count,3);
        }else{
            return self.fst_mini_max(color, count, 3);
        }
    }

}