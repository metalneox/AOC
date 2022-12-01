pub(crate) fn solution1() {
    let text = include_str!("../solutions/input/one");
    let calories = text.split("\n\n").collect::<Vec<_>>();

    let mut max = 0;

    for value in calories {
        let temp = value.split("\n").map(|x| x.to_string()).collect::<Vec<String>>();  
        let prova = temp[0].parse::<i32>();
        let current:i32 = temp.iter().map(|x| x.parse::<i32>().unwrap()).sum();

        if max < current {
            max = current;
        }
    }

    println!("{}",max);
}

pub(crate) fn solution2() {
    let text = include_str!("../solutions/input/one");
    let calories = text.split("\n\n").collect::<Vec<_>>();

    let mut vet = vec![];

    for value in calories {
        let temp = value.split("\n").map(|x| x.to_string()).collect::<Vec<String>>();  
        let current:i32 = temp.iter().map(|x| x.parse::<i32>().unwrap()).sum();

        vet.push(current);

    }

    vet.sort();
    let siz = vet.len();
    let result = vet[siz-1]+vet[siz-2]+vet[siz-3];

    println!("{}",result);
}
