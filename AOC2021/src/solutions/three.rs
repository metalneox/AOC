pub(crate) fn solution1() {
    //let test = include_str!(".././solutions/input/two_test");
    let matrix: Vec<Vec<u32>> = std::fs::read_to_string("src/solutions/input/three")
        .expect("file not found!")
        .lines()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();

    //println!("{:#?}",matrix[0][0]);

    let mut gamma_rate = "".to_string(); // valore delle colonne piu frequente

    //todo magia
    for i in 0..matrix[0].len() {
        let mut temp: Vec<u32> = vec![];
        for j in 0..matrix.len() {
            //println!("{}",matrix[j][i]);
            temp.push(matrix[j][i]);
        }

        let one = temp.iter().filter(|x| x == &&1u32).count();
        let zero = temp.iter().filter(|x| x == &&0u32).count();

        if one >= zero {
            gamma_rate.push('1');
        } else {
            gamma_rate.push('0');
        }
    }

    let epsilon_rate = gamma_rate
        .chars()
        .map(|x| match x {
            '0' => '1',
            '1' => '0',
            _ => '0',
        })
        .collect::<String>();

    let gamma = u32::from_str_radix(&gamma_rate, 2).unwrap();
    println!("{}", gamma);
    let epsilon = u32::from_str_radix(&epsilon_rate, 2).unwrap();
    println!("{}", epsilon);
    println!("{}", gamma * epsilon);
}

fn delete_matrix(matrix_temp:&mut Vec<Vec<u32>>,value:u32,position:usize)-> Vec<Vec<u32>>{
        if matrix_temp.len() <= 1 {
            return matrix_temp.to_vec();
        }
        matrix_temp.retain(|x| x[position] != value);
        println!("{:#?}",matrix_temp);
        matrix_temp.to_vec()
}


pub(crate) fn solution2() {
    //let test = include_str!(".././solutions/input/two_test");
    let mut matrix: Vec<Vec<u32>> = std::fs::read_to_string("src/solutions/input/three")
        .expect("file not found!")
        .lines()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();

    let mut matrix2 = matrix.clone();

    let mut pos = 0;

    while matrix.len() > 1 {
        for i in 0..matrix[0].len() {
            let mut temp: Vec<u32> = vec![];
            for j in 0..matrix.len() {
                //println!("{}",matrix[j][i]);
                temp.push(matrix[j][i]);
            }

            let one = temp.iter().filter(|x| x == &&1u32).count();
            let zero = temp.iter().filter(|x| x == &&0u32).count();

            if one >= zero {
                delete_matrix(&mut matrix,0,pos);
                pos += 1;
            } else { 
                delete_matrix(&mut matrix,1,pos);
                pos += 1;
            }
        }
    }

    let mut pos2 = 0;
    while matrix2.len() > 1 {
        for i in 0..matrix2[0].len() {
            let mut temp: Vec<u32> = vec![];
            for j in 0..matrix2.len() {
                //println!("{}",matrix[j][i]);
                temp.push(matrix2[j][i]);
            }

            let one = temp.iter().filter(|x| x == &&1u32).count();
            let zero = temp.iter().filter(|x| x == &&0u32).count();

            if one < zero {
                delete_matrix(&mut matrix2,0,pos2);
                pos2 += 1;
            } else { 
                delete_matrix(&mut matrix2,1,pos2);
                pos2 += 1;
            }
        }
    }
    let oxygen_arr = matrix[0].iter().map(|x| x.to_string()).collect::<String>();
    println!("{:#?}",matrix2);
    let c02_arr =  matrix2[0].iter().map(|x| x.to_string()).collect::<String>();

    let oxygen = u32::from_str_radix(&oxygen_arr, 2).unwrap();
    let c02 = u32::from_str_radix(&c02_arr, 2).unwrap();
    println!("{}", oxygen*c02);
    
    //let epsilon = u32::from_str_radix(&epsilon_rate, 2).unwrap();
    //println!("{}", epsilon);
}