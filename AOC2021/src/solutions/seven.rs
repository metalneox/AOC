fn median(numbers: &mut [i64]) -> i64 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

// Gli step del piccolo gauss ===========>    (n* (n+1)/2)
fn step_calculator(value: i64, step: i64, current: &mut i64) -> i64 {
    if step <= 0 {
        return *current;
    }

    *current += value + 1i64;
    let step = step - 1;
    step_calculator(value + 1i64, step, current)
}

pub(crate) fn solution1() {
    let stream = include_str!("../solutions/input/seven");
    let series = stream
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut lines = series;

    let result = median(&mut lines);

    let result: i64 = lines.iter().map(|x| (x - result).abs()).sum();

    println!("{}", result);
}

pub(crate) fn solution2() {
    let stream = include_str!("../solutions/input/seven");
    let series = stream
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut lines = series.clone();

    let mut result = 0;

    let min_value = series.iter().min().unwrap();
    let max_value = series.iter().max().unwrap();

    for val in *min_value..*max_value {
        //let step_cost = step_calculator(  0 , 4,&mut 0 );

        // risolvere il problema qui
        let temp_result: i64 = lines
            .iter()
            .map(|x| step_calculator(0, (x - val).abs(), &mut 0))
            .sum();

        //

        if temp_result < result || result <= 0 {
            result = temp_result;
        }
    }

    println!("{}", result);
}
