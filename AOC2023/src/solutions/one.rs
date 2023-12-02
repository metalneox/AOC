use std::collections::HashMap;

pub(crate) fn solution1() {
    let text = include_str!("../solutions/input/one");
    let mut result = 0;

    for a in text.lines() {
        let prova: Vec<_> = a.chars().filter(|x| x.is_numeric()).collect();
        let primo = prova.first().unwrap();
        let ultimo = prova.last().unwrap();
        let temp_res = format!("{}{}", primo, ultimo).parse::<i32>().unwrap();

        result += temp_res;
    }

    println!("Solutions 1 =>{}", result);
}

//soluzione alternativa interessante sostituisci dalle stringhe i nomi con i numeri e poi riusare
//la solution 1
pub(crate) fn solution2() {
    let text = include_str!("../solutions/input/one");

    let map = vec![
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ];

    let mut result = 0;

    for a in text.lines() {
        let mut myvec: Vec<_> = Vec::new();
        let mut string_temp = "".to_string();

        for curr_char in a.chars() {
            if curr_char.is_numeric() {
                myvec.push(curr_char.to_string().parse::<i32>().unwrap());
                string_temp = "".to_string();
            } else {
                string_temp.push(curr_char);

                for (search, value) in map.clone() {
                    if string_temp.contains(&search) {
                        myvec.push(value);
                        string_temp = "".to_string();
                        string_temp.push(curr_char);
                        break;
                    }
                }
            }
        }
        println!("{:#?}", myvec);
        let primo = myvec.first().unwrap();
        let ultimo = myvec.last().unwrap();

        let temp_res = format!("{}{}", primo, ultimo).parse::<i32>().unwrap();
        result += temp_res;
    }

    println!("Solutions 2 =>{}", result);
}
