use std::collections::HashMap;

pub(crate) fn solution1() {
    let text = include_str!("../solutions/input/two");

    let mut result = 0;

    let mut colors_max: HashMap<&str, u32> = HashMap::new();
    colors_max.insert("red", 12);
    colors_max.insert("green", 13);
    colors_max.insert("blue", 14);

    for line in text.lines() {
        //togliere collect
        let splited = line.split(':').collect::<Vec<&str>>();

        let id = splited[0].strip_prefix("Game ").unwrap_or("");
        let mut game_valid = true;

        for bags in splited[1].split(|c| matches!(c, ',' | ';')) {
            let mut current_bag = bags.split_whitespace();

            let value_bag: u32 = current_bag
                .next()
                .expect("No numeric part")
                .parse()
                .expect("Failed to parse numeric part");

            let string_bag = current_bag.next().expect("No string part");

            if value_bag > *colors_max.get(string_bag).unwrap() {
                game_valid = false;
                break;
            }
        }
        //12 red cubes, 13 green cubes, and 14 blue cubes?
        if game_valid {
            result += id.trim().parse::<i32>().unwrap();
        }
    }

    println!("Result solution 1 ==> {}", result); //2285
}

pub(crate) fn solution2() {
    let text = include_str!("../solutions/input/two");

    let mut result = 0;

    let mut colors_max: HashMap<&str, u32> = HashMap::new();
    for line in text.lines() {
        colors_max.insert("red", 0);
        colors_max.insert("green", 0);
        colors_max.insert("blue", 0);

        let splited = line.split(':').collect::<Vec<&str>>();

        for bags in splited[1].split(|c| matches!(c, ',' | ';')) {
            let mut current_bag = bags.split_whitespace();

            let value_bag: u32 = current_bag
                .next()
                .expect("No numeric part")
                .parse()
                .expect("Failed to parse numeric part");

            let string_bag = current_bag.next().expect("No string part");

            let curr_max = colors_max.get_mut(string_bag).unwrap();

            if value_bag > *curr_max {
                *curr_max = value_bag;
            }
        }
        result += colors_max.get("red").unwrap()
            * colors_max.get("green").unwrap()
            * colors_max.get("blue").unwrap();
    }

    println!("Result solution 2 ==> {}", result); //77021
}
