#[warn(dead_code)]
#[derive(Debug)]
struct Position {
    num: i32,
    pos_init: (usize, usize),
    pos_end: (usize, usize),
}

fn check_neighbors(current_pos: (usize, usize), grid: &[[char; 140]; 140]) -> bool {
    let x = current_pos.0 as i32;
    let y = current_pos.1 as i32;
    //println!("{:#?}",grid);
    let neighbors: Vec<(i32, i32)> = vec![
        (-1, 0),
        (-1, -1),
        (0, -1),
        (-1, 1),
        (1, 0),
        (1, 1),
        (0, 1),
        (1, -1),
    ];

    for neighbor in neighbors {
        if x + neighbor.0 < grid.len() as i32
            && (x + neighbor.0) >= 0
            && y + neighbor.1 < grid.len() as i32
            && (y + neighbor.1) >= 0
        {
            let current_x = (x + neighbor.0) as usize;
            let current_y = (y + neighbor.1) as usize;

            if grid[current_x][current_y] != '.' && !grid[current_x][current_y].is_numeric() {
                return true;
            }
        }
    }

    false
}

//horizontal
fn resolve_p1(num_data: Vec<Position>, grid: &[[char; 140]; 140]) -> i32 {
    let mut result = 0;
    for current in num_data {
        let range_pos = current.pos_init.1..current.pos_end.1;

        for a in range_pos {
            let temp = (current.pos_init.0, a);
            let valid = check_neighbors(temp, grid);

            //println!("current {}--{}", current.num, valid);
            if valid {
                //println!("current {}", current.num);
                result += current.num;
                //println!("Somma {}", result);
                break;
            }
        }
    }

    result
}

pub(crate) fn solution1() {
    //let text = include_str!("../solutions/input/three_test");
    let text = include_str!("../solutions/input/three");
    //Grid size test
    //let mut grid = [['.'; 10]; 10];
    //Grid Size full data
    let mut grid = [['.'; 140]; 140];

    //make grid
    for line in text.lines().enumerate() {
        for init in line.1.chars().enumerate() {
            grid[line.0][init.0] = init.1;
        }
    }

    let mut num_pos: Vec<Position> = vec![];

    let mut pos_init = (0, 0);
    //let mut pos_end = (0, 0);

    for row_char in grid.iter().enumerate() {
        let mut temp = "".to_string();
        for current_char in row_char.1.iter().enumerate() {
            if current_char.1.is_numeric() {
                if temp.is_empty() {
                    pos_init = (row_char.0, current_char.0);
                }
                temp.push(*current_char.1);
            } else if !temp.is_empty() {
                let num = temp.parse::<i32>().unwrap();
                //println!("{}", num);
                let pos_end = (row_char.0, current_char.0);
                num_pos.push(Position {
                    num: num,
                    pos_init: pos_init,
                    pos_end: pos_end,
                });
                temp.clear();
            }
        }
    }

    //println!("{:#?}", num_pos);
    let result = resolve_p1(num_pos, &grid);
    println!("Day 3 solution 1 => {}", result);
}

pub(crate) fn _solution2() {}
