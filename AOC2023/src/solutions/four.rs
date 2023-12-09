#[derive(Debug)]
struct Card {
    num: usize,
    win_num: Vec<usize>,
    extr_num: Vec<usize>,
}

pub(crate) fn solution1() {
    let text = include_str!("../solutions/input/four");

    let mut games: Vec<Card> = vec![];

    //init struct
    for line in text.lines() {
        let mut current_card: Card;

        let row = line.split_once(":").unwrap();

        println!("{:#?}", row);
        ////TODO: Ricontrollare che sia tutto ok ho fatto unwrap_or per un overfloww
        let tuple_num = row.0.split_once(" ").unwrap();
        //.1.parse::<usize>().unwrap();

        let num = tuple_num.1.trim().parse::<usize>().unwrap();

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

        let current_struct = Card {
            num: num,
            win_num: card1,
            extr_num: card2,
        };

        games.push(current_struct);
    }

    let mut result = 0;
    for game in games {
        //let mut points = 0;

        let points = game
            .extr_num
            .iter()
            .filter(|x| game.win_num.contains(x))
            .count();

        if points > 0 {
            result += (1..points).fold(1, |a, _| a * 2);
        }
    }
    println!("Day 4 solution 1 => {}", result);
}

pub(crate) fn solution2() {
    let text = include_str!("../solutions/input/four");

    let mut games: Vec<Card> = vec![];
    let mut scratchcards = vec![1; text.lines().count()];

    //init struct
    for line in text.lines() {
        let mut current_card: Card;

        let row = line.split_once(":").unwrap();

        println!("{:#?}", row);
        ////TODO: Ricontrollare che sia tutto ok ho fatto unwrap_or per un overfloww
        let tuple_num = row.0.split_once(" ").unwrap();
        //.1.parse::<usize>().unwrap();

        let num = tuple_num.1.trim().parse::<usize>().unwrap();

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

        let current_struct = Card {
            num: num,
            win_num: card1,
            extr_num: card2,
        };

        games.push(current_struct);
    }

    for game in games {
        //let mut points = 0;

        let points = game
            .extr_num
            .iter()
            .filter(|x| game.win_num.contains(x))
            .count();

        let num2 = game.num - 1; //index 0 array
                                 //
        for _ in 0..scratchcards[num2] {
            for i in (num2 + 1)..(num2 + 1 + points) {
                if i >= scratchcards.len() {
                    break;
                }
                scratchcards[i] += 1;
            }
        }
    }
    println!("{:#?}", scratchcards);
    let result: i32 = scratchcards.iter().sum();
    println!("Day 4 solution 2 0 => {}", result);
}
