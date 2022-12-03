pub(crate) fn solution1() {
    let text = include_str!("../solutions/input/three"); 
    let bags = text.split("\n").collect::<Vec<_>>();
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut result = 0;

    for current in bags{
        let (split1, split2) = current.split_at(current.len()/2);

        let vet1 = split1.chars().collect::<Vec<char>>();
        let vet2 = split2.chars().collect::<Vec<char>>();

        let letter = vet1.iter()
                         .filter(|x| vet2.contains(x)) 
                         .map(|x| *x)
                         .collect::<Vec<char>>();

        let index_opt = alphabet.iter().position(|x| *x == letter[0]);
        let mut index = 0;

        if index_opt.is_some(){
           index = index_opt.unwrap()+1;
        }
        result += index;
    }
    
    println!("Day 3 solution 1 => {}",result);

}

pub(crate) fn solution2() {
    let text = include_str!("../solutions/input/three"); 
    let bags = text.split("\n").collect::<Vec<_>>();
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut result = 0;

    let three = bags.chunks(3).map(|x| x.to_vec()).collect::<Vec<Vec<&str>>>();
    

    for group in three{

        let compare = group[0].chars().filter(|x| group[1].contains(*x)).map(|x| x).collect::<Vec<char>>();

        let letter = compare.iter().filter(|&&x| group[2].contains(x)).map(|x| *x).collect::<Vec<char>>();
 
        let index_opt = alphabet.iter().position(|x| *x == letter[0]);
        let mut index = 0;
    
        if index_opt.is_some(){
           index = index_opt.unwrap()+1;
        }
        result += index;
    }   
      
    println!("Day 3 solution 2 => {}",result);

}