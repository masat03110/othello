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

    pub fn eval_b_pos(&self, color:usize) -> usize{
        let mycolor:u64;
        let oppcolor:u64;
        let mut eval_points:usize = 100;
        if color == 0{
            // color is black
            mycolor = self.black;
            oppcolor = self.white;            
        }else{
            // color is white
            mycolor = self.white;
            oppcolor = self.black;
        }

        eval_points -= 25*count_pieces(oppcolor & 0x8100000000000081) as usize;
        eval_points += 25*count_pieces(mycolor & 0x8100000000000081) as usize;

        if ((mycolor & 0xFF) != 0) && (oppcolor & 0xFF) == 0{
            eval_points += 10;
        }  

        if ((mycolor & 0x8080808080808080) != 0) && (oppcolor & 0x8080808080808080) == 0{
            eval_points += 10;
        }

        if ((mycolor & 0x0101010101010101) != 0) && (oppcolor & 0x0101010101010101) == 0{
            eval_points += 10;
        }

        if ((mycolor & 0xFF00000000000000) != 0) && (oppcolor & 0xFF00000000000000) == 0{
            eval_points += 10;
        }

        return eval_points;

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

    pub fn add2(&mut self, bit: usize, turn: usize){
        if turn == 0 { //black's turn
            self.black |= 1 << bit;

            self.check_and_change(bit,turn);

        }else{ // white's turn
            self.white |= 1 << bit;

            self.check_and_change(bit,turn);
        
        }
    }

    pub fn make_1_next_boards(&self, color: usize, _count: usize) -> Vec<(OthelloBoard,usize,usize)>{
        let mut turn = color;
        let board2 = self.clone();
        let mut valids_now = board2.valid_pieces(turn);
        
        if valids_now[64] == 0{
            turn = 1 - turn;
            valids_now = board2.valid_pieces(turn);
            if valids_now[64] == 0{
                let board = OthelloBoard {
                    black: board2.black,
                    white: board2.white,
                };
                return vec![(board,99,color)]
                //99 means no availeble position
            }
        }
        
        let slice = &valids_now[65..];
        let valid_pos: Vec<usize> = slice.to_vec();
        let mut next_boards:Vec<(OthelloBoard,usize,usize)> = Vec::new();
    
        for item in valid_pos{
            let mut board3 = OthelloBoard {
                black: board2.black,
                white: board2.white,
            };

            board3.add2(item, turn);
            next_boards.push((board3,item,1-turn));
        }   
        return next_boards;
    }

    fn last_mini_max_sub(&self, color: usize, turn: usize, count:usize)->isize{
        let boards = self.make_1_next_boards(turn,count);
        let mut return_count:isize = -99;
        let mut now_count:isize;

        for (board, pos, now_turn) in boards {
            if pos == 99{
                return self.subtract(color);
            }
            else if 1-now_turn == color{
                now_count = board.last_mini_max_sub(color, now_turn,count);
                if return_count == -99{
                    return_count = now_count;
                }else if return_count < now_count {
                    return_count = now_count;
                }   
            }else{
                now_count = board.last_mini_max_sub(color, now_turn,count);
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
        let boards = self.make_1_next_boards(color,count);
        let mut return_pos:usize = 99;
        let mut max:isize = -999;
        let mut now_count;
        for (board, pos, turn) in boards {
            now_count = board.last_mini_max_sub(color, turn ,count);
            if now_count > 0{
                return pos;
            }
            if max < now_count{
                max = now_count;
                return_pos = pos;
            }
        }
        return return_pos;
    }


    pub fn eval_boards_mid(&self, color: usize, count: usize) -> (usize,usize){
        let mut turn = color;
        let board2 = self.clone();
        let valids_now = board2.valid_pieces(color);
        if valids_now[64] == 0{
            return (0,99);
        }
        let slice = &valids_now[65..];
        let valid_pos: Vec<usize> = slice.to_vec();
        let mut max_num:usize = 0;
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

                eval_num = 0;

                if count < 20{
                    eval_num += 100*(valids[min]+valids[max])/(valids[64]*valids_now[item]);

                 }else if count < 30{

                    eval_num += 100*(valids[min]+valids[max])/(valids[64]*valids_now[item]);
                }
                else{
                    eval_num +=  100*(valids_now[item]*valids_now[item])/(valids[64]*(valids[max]));
                }
            }else{
                
                eval_num = 10000*valids_now[item];
            }

            eval_num += self.eval_b_pos(color);

            if eval_num > max_num{
                max_num = eval_num;
                return_pos = item;
            }
        }

        (max_num,return_pos)

    }

    fn mid_mini_max_sub(&self, color: usize, turn: usize, count:usize, n:usize)->isize{
        
        let boards = self.make_1_next_boards(turn,count);
        let mut return_count:isize = -999999;
        let mut now_count:isize;

        if n == 0 {
            if turn == color{
                let (points,_) = self.eval_boards_mid(turn, count);
                return points as isize;
            }else{
                let (points,_) = self.eval_boards_mid(turn, count);
                return (-1)*(points as isize);
            }
        }
        for (board, pos, now_turn) in boards {
            if pos == 99{
                let points = board.eval_b_pos(color);
                return points as isize;
            }
            else if now_turn == color{
                now_count = board.mid_mini_max_sub(color, now_turn,count,n-1);
                if return_count == -999999{
                    return_count = now_count;
                }else if return_count > now_count {
                    return_count = now_count;
                }   
            }else{
                now_count = board.mid_mini_max_sub(color, now_turn,count
                ,n-1);
                if return_count == -999999{
                    return_count = now_count;
                }else if return_count < now_count {
                    return_count = now_count;
                }
            }
        }

        return return_count;
    }

    fn mid_mini_max(&self, color: usize, count: usize, n:usize)-> usize{
        let boards = self.make_1_next_boards(color,count);
        let mut return_pos:usize = 99;
        let mut max:isize = -999;
        let mut now_count ;
        let oppboard:u64;
        let myboard:u64;
        let mut oppboard2:u64;
        let mut myboard2:u64;

        if color == 0{
            myboard = self.black;
            oppboard = self.white;
        }else{
            myboard = self.white;
            oppboard = self.black;
        }


        for (board, pos, turn) in boards {
            now_count = 0;
            now_count += board.mid_mini_max_sub(color, turn ,count,n);

            if color == 0{
                myboard2 = board.black;
                oppboard2 = board.white;
            }else{
                myboard2 = board.white;
                oppboard2 = board.black;
            }

            print!("{}:",pos);

            let position = (1 as u64) << pos;
            if position & 0x8100000000000081 != 0{
                now_count += 99999;
            }else if ((myboard ^ myboard2) & 0x0042000000004200) & (myboard2) != 0{
                now_count -= 99999;
            }else if position & 0x0042000000004200 != 0{
                now_count -= 99999;
            }else if position & 0xFF000000000000FF != 0 {
                if ((position << 1) & oppboard2 != 0) && ((position >> 1) & oppboard2) != 0{
                    now_count += 99999;
                }
                else if ((position << 1) | (position >> 1)) & oppboard2 != 0{
                    now_count -= 99999;
                }else if (position << 1)  & myboard2 == 0{
                    if ((position << 2) & 0xFF000000000000FF) & myboard2 != 0{
                    now_count -= 99999;
                    }
                }else if (position >> 1)  & myboard2 == 0{
                    if ((position >> 2) & 0xFF000000000000FF) & myboard2 != 0{
                    now_count -= 99999;
                    }
                }
            }else if position & 0x0081818181818100 != 0 {
                if ((position << 8) & oppboard2 != 0) && (position >> 8) & oppboard2 != 0{
                    now_count += 99999;
                }
                else if ((position << 8) | (position >> 8)) & oppboard2 != 0{
                    now_count -= 99999;
                }else if (position << 8)  & myboard2 == 0{
                    if ((position << 16) & 0x0081818181818100) & myboard2 != 0{
                    now_count -= 99999;
                    }
                }else if (position >> 8)  & myboard2 == 0{
                    if ((position >> 16) & 0x0081818181818100) & myboard2 != 0{
                    now_count -= 99999;
                    }
                }
            }

            if ((self.white | self.black) & 0xFFC3C3C3C3FF) == 0{
                if position & 0xFFC3C3C3C3FF != 0{
                    now_count -= 500;
                }
            }
            
            if (position & 0xFF818181818181FF != 0) && (((self.white | self.black) & 0xFF818181818181FF) != 0){
                now_count += 500;
            }

            if (oppboard ^ oppboard2) & 0xFF818181818181FF != 0{
                now_count += 9999999;
            }

            if ((myboard ^ myboard2) & 0x0042000000004200) & (myboard2) != 0{
                now_count -= 99999;
            }

            if return_pos == 99{
                max = now_count;
                return_pos = pos;
            }        
            if max < now_count{
                max = now_count;
                return_pos = pos;
            }
            println!("{}",now_count);
        }

        return return_pos;
    }

        pub fn eval_boards_fst(&self, color: usize, count: usize) -> (usize,usize){
        let mut turn = color;
        let board2 = self.clone();
        let valids_now = board2.valid_pieces(color);
        if valids_now[64] == 0{
            return (0,99);
        }
        let slice = &valids_now[65..];
        let valid_pos: Vec<usize> = slice.to_vec();
        let mut max_num:usize = 0;
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

                eval_num = 0;

                if count < 20{
                    eval_num += 100*(valids[min]+valids[max])/(valids[64]*valids_now[item]);

                 }else if count < 30{

                    eval_num += 100*(valids[min]+valids[max])/(valids[64]*valids_now[item]);
                }
                else{
                    eval_num +=  100*(valids_now[item]*valids_now[item])/(valids[64]*(valids[max]));
                }
            }else{
                
                eval_num = 10000*valids_now[item];
            }

            eval_num += self.eval_b_pos(color);

            if eval_num > max_num{
                max_num = eval_num;
                return_pos = item;
            }
        }

        (max_num,return_pos)

    }

    fn fst_mini_max_sub(&self, color: usize, turn: usize, count:usize, n:usize)->isize{
        
        let boards = self.make_1_next_boards(turn,count);
        let mut return_count:isize = -999999;
        let mut now_count:isize;

        if n == 0 {
            if turn == color{
                let (points,_) = self.eval_boards_fst(turn, count);
                return points as isize;
            }else{
                let (points,_) = self.eval_boards_fst(turn, count);
                return (-1)*(points as isize);
            }
        }
        for (board, pos, now_turn) in boards {
            if pos == 99{
                let points = board.eval_b_pos(color);
                return points as isize;
            }
            else if now_turn == color{
                now_count = board.fst_mini_max_sub(color, now_turn,count,n-1);
                if return_count == -999999{
                    return_count = now_count;
                }else if return_count > now_count {
                    return_count = now_count;
                }   
            }else{
                now_count = board.fst_mini_max_sub(color, now_turn,count
                ,n-1);
                if return_count == -999999{
                    return_count = now_count;
                }else if return_count < now_count {
                    return_count = now_count;
                }
            }
        }

        return return_count;
    }

    fn fst_mini_max(&self, color: usize, count: usize, n:usize)-> usize{
        let boards = self.make_1_next_boards(color,count);
        let mut return_pos:usize = 99;
        let mut max:isize = -999;
        let mut now_count;
        let oppboard:u64;
        let myboard:u64;
        let mut oppboard2:u64;
        let mut myboard2:u64;

        if color == 0{
            myboard = self.black;
            oppboard = self.white;
        }else{
            myboard = self.white;
            oppboard = self.black;
        }

        for (board, pos, turn) in boards {
            now_count = 0;
            now_count += board.fst_mini_max_sub(color, turn ,count,n);
            if color == 0{
                myboard2 = board.black;
                oppboard2 = board.white;
            }else{
                myboard2 = board.white;
                oppboard2 = board.black;
            }

            print!("{}:",pos);

            let position = (1 as u64) << pos;
            if position & 0x8100000000000081 != 0{
                now_count += 99999;
            }else if ((myboard ^ myboard2) & 0x0042000000004200) & (myboard2) != 0{
                now_count -= 99999;
            }
            else if position & 0x0042000000004200 != 0{
                now_count -= 99999;
            }else if position & 0xFF000000000000FF != 0 {
                if ((position << 1) & oppboard2 != 0) && ((position >> 1) & oppboard2) != 0{
                    now_count += 99999;
                }
                else if ((position << 1) | (position >> 1)) & oppboard2 != 0{
                    now_count -= 99999;
                }else if (position << 1)  & myboard2 == 0{
                    if ((position << 2) & 0xFF000000000000FF) & myboard2 != 0{
                    now_count -= 99999;
                    }
                }else if (position >> 1)  & myboard2 == 0{
                    if ((position >> 2) & 0xFF000000000000FF) & myboard2 != 0{
                    now_count -= 99999;
                    }
                }
            }else if position & 0x0081818181818100 != 0 {
                if ((position << 8) & oppboard2 != 0) && (position >> 8) & oppboard2 != 0{
                    now_count += 99999;
                }
                else if ((position << 8) | (position >> 8)) & oppboard2 != 0{
                    now_count -= 99999;
                }else if (position << 8)  & myboard2 == 0{
                    if ((position << 16) & 0x0081818181818100) & myboard2 != 0{
                    now_count -= 99999;
                    }
                }else if (position >> 8)  & myboard2 == 0{
                    if ((position >> 16) & 0x0081818181818100) & myboard2 != 0{
                    now_count -= 99999;
                    }
                }
            }

            

            if ((self.white | self.black) & 0xFFC3C3C3C3FF) == 0{
                if position & 0xFFC3C3C3C3FF != 0{
                    now_count -= 500;
                }
            }
            
            if (position & 0xFF818181818181FF != 0) && (((self.white | self.black) & 0xFF818181818181FF) != 0){
                now_count += 5000;
            }

            if (oppboard ^ oppboard2) & 0xFF818181818181FF != 0{
                now_count += 9999999;
            }

            if return_pos == 99{
                max = now_count;
                return_pos = pos;
            }        
            if max < now_count{
                max = now_count;
                return_pos = pos;
            }
            println!("{}",now_count);
        }

        
        return return_pos;
    }

    pub fn eval_boards(&self, color: usize, count: usize) -> usize{
        if count > 51{
            return self.last_mini_max(color, count);
        }else if count > 30{
            return self.mid_mini_max(color, count,3);
        }else{
            return self.fst_mini_max(color, count, 3);
        }
    }

}