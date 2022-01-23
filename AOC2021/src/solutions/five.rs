use std::collections::HashMap;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}
#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

#[derive(Debug)]
struct Canvas {
    table: HashMap<(u32, u32), u32>,
}

impl Canvas {
    fn new() -> Canvas {
        Canvas {
            table: HashMap::new(),
        }
    }
    //settaggio delle colonne orizzontali o verticali e sicuramente da riscrivere codice duplicato e soluzione triste
    fn set_line(&mut self, current_line: Line) {
        if current_line.p1.x == current_line.p2.x {
            //setto linea orizzontale
            if current_line.p1.y <= current_line.p2.y {
                for i in current_line.p1.y..=current_line.p2.y {
                    //check exist
                    let k = (current_line.p1.x, i);

                    if !self.table.contains_key(&k) {
                        self.table.insert(k, 1);
                    } else {
                        let mut tmp = self.table.get(&k).unwrap().clone();
                        tmp += 1;
                        self.table.insert(k, tmp);
                    }
                    //println!("{}",i);
                }
            } else {
                for i in current_line.p2.y..=current_line.p1.y {
                    //check exist
                    let k = (current_line.p1.x, i);

                    if !self.table.contains_key(&k) {
                        self.table.insert(k, 1);
                    } else {
                        let mut tmp = self.table.get(&k).unwrap().clone();
                        tmp += 1;
                        self.table.insert(k, tmp);
                    }
                    //println!("{}",i);
                }
            }
            //controllare anche p1.y > p2.y e uguale con altro secondo if
        } else if current_line.p1.y == current_line.p2.y {
            //setto linea verticale
            if current_line.p1.x <= current_line.p2.x {
                for i in current_line.p1.x..=current_line.p2.x {
                    //check exist
                    let k = (i, current_line.p1.y);

                    if !self.table.contains_key(&k) {
                        self.table.insert(k, 1);
                    } else {
                        let mut tmp = self.table.get(&k).unwrap().clone();
                        tmp += 1;
                        self.table.insert(k, tmp);
                    }
                    //println!("{}",i);
                }
            } else {
                for i in current_line.p2.x..=current_line.p1.x {
                    //check exist
                    let k = (i, current_line.p1.y);

                    if !self.table.contains_key(&k) {
                        self.table.insert(k, 1);
                    } else {
                        let mut tmp = self.table.get(&k).unwrap().clone();
                        tmp += 1;
                        self.table.insert(k, tmp);
                    }
                    //println!("{}",i);
                }
            }
        }else{
            //check diagonale
            #[inline]
            fn diff(a: u32, b: u32) -> u32 {
                if b > a {
                    b - a
                } else {
                    a - b
                }
            }
            //println!("a.x {} a.y {} -> b.x {} b.y {}",current_line.p1.x,current_line.p1.y,current_line.p2.x,current_line.p2.y);
            if diff(current_line.p1.x, current_line.p2.x) == diff(current_line.p1.y, current_line.p2.y) {
                // setto i valori delle diagonale nella canvas
                if current_line.p1.x > current_line.p2.x && current_line.p1.y > current_line.p2.y {
                    //diagonale in basso a sinistra
                    //fix for
                    let mut one_x = current_line.p1.x;
                    let mut one_y = current_line.p1.y;
                    loop{
                        //check exist
                        let k = (one_x, one_y);

                        if !self.table.contains_key(&k) {
                            self.table.insert(k, 1);
                        } else {
                            let mut tmp = self.table.get(&k).unwrap().clone();
                            tmp += 1;
                            self.table.insert(k, tmp);
                        }
                        if one_x == current_line.p2.x && one_y == current_line.p2.y {break;}
                        one_x -= 1;
                        one_y -= 1;
                        //println!("------{:#?}",k);
                    }

                }else if current_line.p1.x < current_line.p2.x && current_line.p1.y < current_line.p2.y {
                    let mut one_x = current_line.p1.x;
                    let mut one_y = current_line.p1.y;
                    loop{
                        //check exist
                        let k = (one_x, one_y);

                        if !self.table.contains_key(&k) {
                            self.table.insert(k, 1);
                        } else {
                            let mut tmp = self.table.get(&k).unwrap().clone();
                            tmp += 1;
                            self.table.insert(k, tmp);
                        }

                        if one_x == current_line.p2.x && one_y == current_line.p2.y {break;}
                        one_x += 1;
                        one_y += 1;
                        //println!("{:#?}",k);
                    }
                }else if current_line.p1.x < current_line.p2.x && current_line.p1.y > current_line.p2.y {
                    //diagonale in alto a sinistra
                    let mut one_x = current_line.p1.x;
                    let mut one_y = current_line.p1.y;
                    loop{
                        //check exist
                        let k = (one_x, one_y);

                        if !self.table.contains_key(&k) {
                            self.table.insert(k, 1);
                        } else {
                            let mut tmp = self.table.get(&k).unwrap().clone();
                            tmp += 1;
                            self.table.insert(k, tmp);
                        }

                        if one_x == current_line.p2.x && one_y == current_line.p2.y {break;}
                        one_x += 1;
                        one_y -= 1;
                    }
                }else if current_line.p1.x > current_line.p2.x && current_line.p1.y < current_line.p2.y {
                    let mut one_x = current_line.p1.x;
                    let mut one_y = current_line.p1.y;
                    loop{
                        //check exist
                        let k = (one_x, one_y);

                        if !self.table.contains_key(&k) {
                            self.table.insert(k, 1);
                        } else {
                            let mut tmp = self.table.get(&k).unwrap().clone();
                            tmp += 1;
                            self.table.insert(k, tmp);
                        }
                        if one_x == current_line.p2.x && one_y == current_line.p2.y {break;}
                        one_x -= 1;
                        one_y += 1;
                    }
                }
 
                //println!("a.x {} a.y {} -> b.x {} b.y {}",current_line.p1.x,current_line.p1.y,current_line.p2.x,current_line.p2.y);
            }


        }
    }
    fn result(&mut self) -> u32 {
        self.table.iter().filter(|x| *x.1 >= 2).count() as u32
    }
}

fn read_point(temp_point: &str) -> Point {
    let split_point = temp_point.split(",").map(|x| x).collect::<Vec<&str>>();
    let result = Point {
        x: split_point[0].parse::<u32>().unwrap(),
        y: split_point[1].parse::<u32>().unwrap(),
    };
    result
}

fn read_line(temp_line: &str) -> Line {
    let split_line = temp_line
        .split("->")
        .map(|x| x.trim())
        .collect::<Vec<&str>>();

    let mut one = read_point(split_line[0]);
    let mut two = read_point(split_line[1]);

    let linea = Line { p1: one, p2: two };
    return linea;
}

//solution 1 and 2 
pub(crate) fn solution1() {
    //Make vector of point tuple
    let stream = include_str!("../solutions/input/five");

    let lines = stream.split("\n").map(|x| x).collect::<Vec<&str>>();

    let mut input: Vec<Line> = vec![];
    let mut canvas_result = Canvas::new();
    //let mut cava:Canvas = Canvas::new();

    for a in lines {
        let tmp_line = read_line(a);
        input.push(tmp_line);
    }

    //dati input strutturati e stampati
    //println!("{:#?}",input);

    for i in input {
        //println!("{:#?}", i);
        canvas_result.set_line(i);
    }

    //println!("{:#?}", canvas_result);
    println!("{:#?}", canvas_result.result());
}
