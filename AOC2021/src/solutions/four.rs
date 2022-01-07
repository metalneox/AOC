use std::{io::BufRead, vec};

//aoc completamente triste è da riscrivere
fn check_last(vec: Vec<Card> ) -> bool { 
    let winner_size = vec.iter().filter(|x| x.win == true).count();
    if vec.len()-1 == winner_size{
        return true;
    }

    false
}


//ultima a vincere
#[derive(Debug,Clone)]
struct Card {
    rows: Vec<Vec<(i32, bool)>>,
    win: bool,
}

impl Card {
    fn new() -> Card {
        Card { rows: vec![] ,win: false}
    }
    //add_row , sign_number ,check and win line
    fn add_row(&mut self, current_row: Vec<(i32, bool)>) {
        self.rows.push(current_row);
    }

    fn sign(&mut self, n: i32) {
        for a in self.rows.as_mut_slice() {
            for b in a.as_mut_slice() {
                //println!("n = {} b.0 = {}",n,b.0);
                if b.0 == n {
                    b.1 = true;
                }
            }
        }
    }
    fn check_row(&mut self) -> bool{
        let rows_consumer = self.rows.iter();

        for a in rows_consumer {
            if a.is_empty() {
                return false;
            }
            if a.iter().all(|p| p.1 == true) {
                self.win = true;
                return true;
            }
        }

        false
    }

    //soluzione veramente triste da riscrivere
    fn check_col(&mut self) -> bool{
        for i in 0..=4 {
            if self.rows[0][i].1 && self.rows[1][i].1 && self.rows[2][i].1 && self.rows[3][i].1 && self.rows[4][i].1 {
                self.win = true;
                return true
            }

        }
        false
    }
    //check solution 1
    fn check(&mut self) -> bool {
        //check row
        //check col
        if self.check_row() == true {
            return true;
        }
        if self.check_col() == true {
            return true;
        }
        false
    }
  
    fn winner(&mut self) -> i32 {
        let rows_consumer = self.rows.iter();
        let mut result = 0;
        //println!("{:#?}",rows_consumer);
        for a in rows_consumer {
            //println!("{:#?}",a);
            result += a
                .iter()
                .filter(|p| p.1 == false)
                .fold(0, |acc, x| acc + x.0);
            //println!("{}",result);
        }
        result
    }


}


pub(crate) fn solution1n2() {
    let test = include_str!("../solutions/input/four");
    //let mut card = vec![vec![(0, 0); 5]; 5];
    let mut card: Vec<Card> = Vec::new();
    let mut input: Vec<i32> = vec![];
    let mut vec_temp: Vec<Vec<(i32, bool)>> = vec![];


    for (id, value) in test.lines().enumerate() {
        if id == 0 {
            input = value
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
        } else {
            if value != "\n\n" {
                let temp: Vec<(i32, bool)> = value
                    .split_whitespace()
                    .to_owned()
                    .map(|x| (x.parse::<i32>().unwrap(), false))
                    .collect::<Vec<(i32, bool)>>();
                
                if !temp.is_empty(){
                    vec_temp.push(temp);
                }
            }
        }
    }

    let len = vec_temp.len() / 5;
    let mut iter = vec_temp.into_iter();
    for _ in 0..len {
        let mut temp2 = Card::new();
        for _ in 0..5 {
            if let Some(val) = iter.next() {
                temp2.add_row(val);
            }
        }
        card.push(temp2);
    }


    //Solution 1
    //'extraction: for i in input {
    //    for a in card.iter_mut() {
    //        a.sign(i);
    //        let flag = a.check();
    //        
    //        if flag == true {
    //            //piglio tutti i numeri non true e moltiplico per ultimo i
    //            //println!("Solution 1 {}",a.winner()*i);
    //            println!("Solution 2 {}",a.winner()*i);
    //            break 'extraction;
    //        }
    //        //win row se si ritorna last number che è 1
    //    }
    //}

    //Solution 2
    let mut flag_winner = false;
    'extraction: for i in input {
        //prende la prima carta dopo aver completata ultima
        flag_winner = check_last(card.clone());
        for a in card.iter_mut() {
            let mut last = false;
             
            if a.win == false {
                last = true;
            }

            a.sign(i);

            //let flag = a.check();
            let flag = a.check() && flag_winner && last;
            

            if flag == true {
                //println!("value = {} ,number = {}",a.winner(),i);
                println!("Solution 2 {}",a.winner()*i);
                break 'extraction;
            }
        }
    }
    //println!("-------");
    //println!("{:#?}",card);
    //println!("-------");
}
