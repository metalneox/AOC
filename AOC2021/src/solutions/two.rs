pub(crate) fn solution1() {
    //let test = include_str!(".././solutions/input/two_test");
    let test:Vec<(String,u32)> = std::fs::read_to_string("src/solutions/input/two")
        .expect("file not found!")
        .lines()
        .map(|x| {
            let mut splitto = x.split(" ");
            let tuplo = (splitto.next().unwrap().to_string(), splitto.next().unwrap().parse::<u32>().unwrap());  
            return tuplo;            
        })
        .collect::<Vec<(String,u32)>>();
    
    let mut depth = 0;
    let mut xpos = 0;

    for (cmd,value) in test{
        match &*cmd {
            "forward" => xpos += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => ()
        }        
    }
    println!("{}",xpos*depth);
}


pub(crate) fn solution2() {
    //let test = include_str!(".././solutions/input/two_test");
    let test:Vec<(String,u32)> = std::fs::read_to_string("src/solutions/input/two")
        .expect("file not found!")
        .lines()
        .map(|x| {
            let mut splitto = x.split(" ");
            let tuplo = (splitto.next().unwrap().to_string(), splitto.next().unwrap().parse::<u32>().unwrap());  
            return tuplo;            
        })
        .collect::<Vec<(String,u32)>>();
    
    let mut depth = 0;
    let mut xpos = 0;
    let mut aim = 0;

    for (cmd,value) in test{
        match &*cmd {
            "forward" => {
                if aim == 0 {
                    xpos += value;
                }else{
                    xpos += value;
                    depth += aim*value;
                }
            },
            "down" => aim += value,
            "up" => aim -= value,
            _ => ()
        }        
    }
    println!("{}",xpos*depth);
}