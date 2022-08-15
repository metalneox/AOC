use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Lanternfish {
    fish: HashMap<u64, u64>,
}

impl Lanternfish {
    fn new() -> Lanternfish {
        Lanternfish {
            fish: HashMap::new(),
        }
    }
}

// da 0 a 8
pub(crate) fn solution1() {
    let stream = include_str!("../solutions/input/six");

    let lines = stream
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    //init input to Laternfish struct

    //conto i numeri da 1 a 8 e il numero presente
    //nuovo starta da 8
    //se arriva a 0 starta da 6
    //80 day

    //init lanterfish
    let mut input = Lanternfish::new();
    for a in 0..=8 {
        let temp = lines.iter().filter(|&&x| x == a).count() as u64;
        //println!("{} -> {}",a,temp);
        input.fish.insert(a, temp);
    }

    for _i in 0..256 {
        let temp = input.clone();

        //println!("{:#?}",temp);

        let zero = temp.fish.get(&0).unwrap().clone() as u64;
        //input.fish.insert(0,0);

        //aggiungo tutti gli 0 diventano 6 + nuovi 8
        let six = temp.fish.get(&7).unwrap().clone() as u64 + zero as u64;

        for j in 1..=8 {
            let val = temp.fish.get(&j);
            match val {
                Some(val) => Some(input.fish.insert(j - 1, *val)),
                _ => None,
            };

            input.fish.insert(6, six);
            input.fish.insert(8, zero);
        }
        /*****************************************************/

        /*****************************************************/
    }

    //test = 5934
    println!("{:#?}", input.fish);
    let sum = input.fish.iter().fold(0, |acc, x| acc + x.1);
    println!("{:#?}", sum);
}
