use std::{collections::HashMap};

pub(crate) fn solution1() {
    let binding =
        std::fs::read_to_string("src/solutions/input/eight").expect("file not found!");
    let vec_data: Vec<_> = binding.lines().map(|x| x.to_string()).collect();

    let mut result = 0;

    for data in vec_data {
        let out = data
            .split("|")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let output = out[1]
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        for a in output {
            let vec_char: Vec<char> = a.chars().collect();

            match vec_char.len() {
                2 | 3 | 4 | 7 => result += 1,
                _ => (),
            }
        }
    }

    println!("{}", result);
}

fn mapping(segmenti:&str) -> i32{
    match segmenti {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" =>2,
        "acdfg" =>3,
        "bcdf" =>4,
        "abdfg" =>5,
        "abdefg" =>6,
        "acf" =>7,
        "abcdefg" =>8,
        "abcdfg" =>9,
        _ => -1,
    }
}

//TODO funzione che rimuove il codice duplicato
fn extract(n1:String,n2:String) -> String{
    let temp = n1.chars().map(|x| {
                                                            if !n2.contains(x){
                                                                return x;
                                                            }else{
                                                                return ' ';
                                                            } 
                                                        }).collect::<String>();

    let temp = temp.replace(" ", "");
    temp
}

fn trasform(vet:Vec<String>) -> HashMap<i32,String>{

    let mut result = HashMap::new();
    let mut vec5:Vec<String> = Vec::new();
    let mut vec6:Vec<String> = Vec::new();

    for a in vet {
        match a.len() {
            2 => result.insert(1, a),
            3 => result.insert(7, a),
            4 => result.insert(4, a),
            5 => {vec5.push(a);None},
            6 => {vec6.push(a);None},
            7 => result.insert(8, a),
            _ => None,
        };
    }

    let mut zero = "".to_string();

    let one = result.get(&1).unwrap().clone();
    let one_vec:Vec<char> = one.chars().collect();


    let four = result.get(&4).unwrap();
    
    let mut six = "".to_string();

    let seven = result.get(&7).unwrap().clone();
    let eight = result.get(&8).unwrap().clone();
    
    let mut nine = "".to_string();

    let segm_a = extract(seven.clone(),one.clone());

    let temp = format!("{}{}",segm_a,four); 
    let segm_g_e = extract(eight.clone(),temp);


    let temp = format!("{}{}{}",segm_a,one,segm_g_e); 
    let segm_b_d = extract(eight.clone(),temp);

    let segm_g_e:Vec<char> = segm_g_e.chars().collect::<Vec<char>>();
    let segm_b_d:Vec<char> = segm_b_d.chars().collect::<Vec<char>>();

    for data in vec6 {
        if data.find(segm_g_e[0]) == None || data.find(segm_g_e[1]) == None{
            nine = data;
        }
        else if data.clone().find(segm_b_d[0]) == None || data.find(segm_b_d[1]) == None{
            zero = data;
        }  
        else if data.find(one_vec[0]) == None || data.find(one_vec[1]) == None{
            six = data;
        }
    }
    
   
    let segm_d = extract(eight.clone(),zero.clone());

    let segm_c = extract(eight.clone(),six.clone());

    let segm_f = extract(one.clone(),segm_c.clone());

    let segm_e = extract(eight.clone(),nine.clone());

    let segm_b = segm_b_d.iter().map(|&x| {
                                                        if !segm_d.contains(x){
                                                            return x;
                                                        }else{
                                                            return ' ';
                                                        } 
                                                    }).collect::<String>();
                                                
    let segm_b = segm_b.replace(" ", "");

    let temp = format!("{}{}",segm_c,segm_e);

    let five = extract(eight.clone(),temp);

    let temp = format!("{}{}",segm_b,segm_f);

    let two = extract(eight.clone(),temp);

    let temp = format!("{}{}",segm_b,segm_e); 

    let three = extract(eight.clone(),temp);

    result.insert(0, zero);
    result.insert(2, two);
    result.insert(3, three);
    result.insert(5, five);
    result.insert(6, six);
    result.insert(9, nine);
 
    result
}


pub(crate) fn solution2(){
    // prima parte mapping
    // seconda parte risultato e poi sommarlo

    let binding =
        std::fs::read_to_string("src/solutions/input/eight").expect("file not found!");
    let vec_data: Vec<_> = binding.lines().map(|x| x.to_string()).collect();

    let mut result = 0;

    for data in vec_data {
        let out = data
            .split("|")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        // Per Mapping
        let mapping = out[0]
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        //println!("Mapping = {:#?}",mapping);
        // 
        let output = out[1]
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        
        let mut temp_result:String = String::new(); 
        let temp = trasform(mapping.clone());

        for numers in output {

            let mut temp_num:Vec<char> = numers.chars().collect();
            temp_num.sort();

            for a in temp.clone().into_iter() {
                let mut temp_h:Vec<char> = a.1.chars().collect();
                temp_h.sort();
 
                if temp_h == temp_num{
                    temp_result.push_str(&a.0.to_string());
                }

            }
           
        }

        result += temp_result.parse::<i32>().unwrap();
    }

    println!("{}", result);

}
