pub(crate) fn solution1() {
    let text = include_str!("../solutions/input/four");
    let jobs = text.split("\n").collect::<Vec<_>>();

    let mut result = 0;

    for line in jobs {
        let current_line = line.trim().split(",").collect::<Vec<_>>();

        //il min e max
        let first = current_line[0].split("-").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
        let second = current_line[1].split("-").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();

         
   
        if first[0] <= second[0] && first[1]>= second[1] || first[0] >= second[0] && first[1] <= second[1] {
            result += 1;
        }
        
    }
    
    println!("Day 4 solution 1 => {}",result);
}

pub(crate) fn solution2() {
    let text = include_str!("../solutions/input/four");
    let jobs = text.split("\n").collect::<Vec<_>>();

    let mut result = 0;

    for line in jobs {
        let current_line = line.trim().split(",").collect::<Vec<_>>();

        let first = current_line[0].split("-").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
        let second = current_line[1].split("-").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();

        if !(first[1] < second[0] || first[0] > second[1]){
            result += 1;
        }
        
    }
    
    println!("Day 4 solution 2 => {}",result);

}
