struct Card {
    num: usize,
    win_num: Vec<usize>,
    extr_num: Vec<usize>,
}

pub(crate) fn solution1() {
    let text = include_str!("../solutions/input/four_test");

    let mut games: Vec<Card> = vec![];

    //init struct
    for line in text.lines() {
        let mut current_card: Card;

        let row = line.split_once(":").unwrap();

        let num = row.0.split_once(" ").unwrap().1.parse::<usize>().unwrap();

        let card_val = row.1.split_once("|").unwrap();

        let card1: Vec<usize> = card_val
            .0
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let card2: Vec<usize> = card_val
            .1
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        //println!("Num {} =>", num);
        //println!("Card Val {:#?} =>", card1);
        //println!("Card Val1 {:#?} =>", card2);

        let current_struct = Card {
            num: num,
            win_num: card1,
            extr_num: card2,
        };

        games.push(current_struct);

        //
    }

    let result = 0;
    println!("Day 4 solution 1 => {}", result);
}

pub(crate) fn solution2() {
    //println!("Day 4 solution 2 => {}", result);
}
