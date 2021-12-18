enum status{
    Incremented,
    Decremented,
}

pub(crate) fn solution1() {
    //include_str!("src/solutions/input/one_test");
    let test: Vec<_> = std::fs::read_to_string("src/solutions/input/one")
        .expect("file not found!")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut result = 0;
    let mut check = status::Incremented;

    for a in test.windows(2) {
        if a[0] < a[1] {
           check = status::Incremented;
        }else{
            check = status::Decremented;
        }

        match check {
            status::Incremented => result += 1,
            status::Decremented => (), 
        }
    }    
}

pub(crate) fn solution2() {
    //include_str!("src/solutions/input/one_test");
    let test: Vec<_> = std::fs::read_to_string("src/solutions/input/one")
        .expect("file not found!")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut result = 0;
    let mut check = status::Incremented;
    let mut temp = test[0]+test[1]+test[2];
    for a in test.windows(3) {
        if a.iter().sum::<i32>() > temp{
           check = status::Incremented;
                }else{
            check = status::Decremented;
        }

        temp = a.iter().sum::<i32>();

        match check {
            status::Incremented => result += 1,
            status::Decremented => (), 
        }
    }    
    println!("{}",result);
}
