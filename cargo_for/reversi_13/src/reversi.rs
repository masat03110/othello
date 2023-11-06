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
    valids |= akikoma & (result << 1);

    result = mask & (myboard >> 1);
    result |= mask & (result >> 1);
    result |= mask & (result >> 1);
    result |= mask & (result >> 1);
    result |= mask & (result >> 1);
    result |= mask & (result >> 1);
    valids |= akikoma & (result >> 1);


    mask = opponent & 0x00FFFFFFFFFFFF00;
    result = mask & (myboard >> 8);
    result |= mask & (result >> 8);
    result |= mask & (result >> 8);
    result |= mask & (result >> 8);
    result |= mask & (result >> 8);
    result |= mask & (result >> 8);
    valids |= akikoma & (result >> 8);

    result = mask & (myboard << 8);
    result |= mask & (result << 8);
    result |= mask & (result << 8);
    result |= mask & (result << 8);
    result |= mask & (result << 8);
    result |= mask & (result << 8);
    valids |= akikoma & (result << 8);

    mask = opponent & 0x007E7E7E7E7E7E00;
    result = mask & (myboard >> 7);
    result |= mask & (result >> 7);
    result |= mask & (result >> 7);
    result |= mask & (result >> 7);
    result |= mask & (result >> 7);
    result |= mask & (result >> 7);
    valids |= akikoma & (result >> 7);

    result = mask & (myboard >> 9);
    result |= mask & (result >> 9);
    result |= mask & (result >> 9);
    result |= mask & (result >> 9);
    result |= mask & (result >> 9);
    result |= mask & (result >> 9);
    valids |= akikoma & (result >> 9);

    result = mask & (myboard << 9);
    result |= mask & (result << 9);
    result |= mask & (result << 9);
    result |= mask & (result << 9);
    result |= mask & (result << 9);
    result |= mask & (result << 9);
    valids |= akikoma & (result << 9);

    result = mask & (myboard << 7);
    result |= mask & (result << 7);
    result |= mask & (result << 7);
    result |= mask & (result << 7);
    result |= mask & (result << 7);
    result |= mask & (result << 7);
    valids |= akikoma & (result << 7);

    valids
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

    // pub fn make_1_next_boards3_2(&self, color: usize, akikoma: u64) -> Vec<OthelloBoard>{

    //     let board = OthelloBoard {
    //         black: self.black,
    //         white: self.white,
    //     };

    //     let mut position:u64 = 1;
    //     let mut nextboards_sub:Vec<OthelloBoard> = vec![];

    //     loop{
    //         while position & akikoma == 0 {
    //             if position == 0 {
    //                 break;
    //             }
    //             position <<= 1;
    //         }

    //         if position == 0 {
    //             break;
    //         }

    //         let total = self.valid_total(position, color);
    //         // let opptotal = self.valid_total(position, 1-color);
            
    //         if total != 0{
    //             // nextboards_sub.push((board.add2(position, color),total+opptotal));
    //             nextboards_sub.push(board.add2(position, color));
    //         }
    //         position <<= 1;
    //     }

    //     if nextboards_sub.is_empty(){

    //         return vec![];

    //     }else{
    //         // nextboards_sub.sort_by(|a, b| b.1.cmp(&a.1));

    //         // let mut first_elements: Vec<OthelloBoard> = nextboards_sub.iter().map(|(first, _)| OthelloBoard { black: first.black, white: first.white, }).collect();

    //         // first_elements.push(OthelloBoard{ black: 0, white: (1-color) as u64, });

    //         // return first_elements;
    //         nextboards_sub.push(OthelloBoard{ black: 0, white: (1-color) as u64, });
    //         return  nextboards_sub;
    //     }

    // }

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

    pub fn make_1_next_boards2(&self, color: usize, flag: u8) -> Vec<(OthelloBoard,u64,usize,usize)>{
        let board = OthelloBoard {
            black: self.black,
            white: self.white,
        };

        let akikoma = !(self.white | self.black);

        if akikoma == 0 {
            return vec![(board,0,color,0)];
        }

        let mut position:u64 = 1;
        let mut nextboards:Vec<(OthelloBoard,u64,usize,usize)> = vec![];
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

            let total = self.valid_total(position, color);
            let opptotal = self.valid_total(position, 1-color);
            
            if total != 0{
                flag2 = 1;
                nextboards.push((board.add2(position, color),position,1-color,total+opptotal));
            }
            position <<= 1;
        }

        if flag2 != 0{
            nextboards.sort_by(|a, b| b.3.cmp(&a.3));
            return nextboards;
        }else if flag != 0{
            return vec![(board,0,color,0)];
        }else{
            return board.make_1_next_boards2(1-color, 1);
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
            next_boards.push((board2.add2(1 << item, turn),item,1-turn));
        }   
        return next_boards;
    }

    fn last_mini_max_sub(&self, color: usize, turn: usize, times:&mut u64)->isize{
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
                        now_count = board.last_mini_max_sub(color, 1-color, times);
                        
                        if 0 < now_count {
                            return now_count;
                        }else if now_count == 0{
                            return_count = 0;
                        }
                }

            } else{
                for board in boards {
                        now_count = board.last_mini_max_sub(color, color, times);
                        
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

    fn last_mini_max(&self, color: usize)-> usize{
        let boards = self.make_1_next_boards2(color,0);
        let mut position:u64 = 0;
        let mut return_pos:usize = 0;
        let mut max:isize = -999;
        let mut now_count;
        let mut times:u64 = 0;
        for (board, pos, turn,_) in boards {
            now_count = board.last_mini_max_sub(color, turn , &mut times);

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
        if count > 42{//montecalro max 51
                      //a-b max 43 43が1手読める程度
            return self.last_mini_max(color);
        }else if count > 30{
            return self.mid_mini_max(color, count,3);
        }else{
            return self.fst_mini_max(color, count, 3);
        }
    }

}