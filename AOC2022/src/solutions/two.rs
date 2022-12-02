#[derive(Clone,Copy,Debug,PartialEq)]
enum status{
    Rock,
    Paper,
    Scissors,
    Null,
}

pub(crate) fn solution1() {
    let text = include_str!("../solutions/input/two");

    let games = text.split("\n").collect::<Vec<_>>();

    let mut result = 0;

    // 1 for Rock, 2 for Paper, and 3 for Scissors valgono sempre
    for value in games{
        let temp:Vec<&str> = value.split(" ").collect();
        let player1 = temp[0];
        //Me
        let player2 = temp[1];

        let p1_status = match player1 {
                            "A" => status::Rock,
                            "B" => status::Paper,
                            "C" => status::Scissors,
                            _ => status::Null};

        let p2_status = match player2 {
                            "X" => status::Rock,
                            "Y" => status::Paper,
                            "Z" => status::Scissors,
                            _ => status::Null};

        let game = (p1_status,p2_status);

        match game {
            game if game.0 == game.1 => result+=3,
            (status::Rock,status::Paper) => result+=6,
            (status::Rock,status::Scissors) => (),
            (status::Paper,status::Rock) => (),
            (status::Paper,status::Scissors) => result+=6,
            (status::Scissors,status::Rock) => result+=6,
            (status::Scissors,status::Paper) => (),
            _ =>(),
        }

        match player2 {
            "X" => result+=1,
            "Y" => result+=2,
            "Z" => result+=3,
            _ => ()
        } 
    }

    println!("Result => {}",result);

}


fn win(value:status)->status{
    match value {
        status::Scissors=> status::Rock,
        status::Paper => status::Scissors,
        status::Rock => status::Paper,
        _ =>status::Null,
    }
}

fn lost(value:status)->status{
    match value {
        status::Scissors=> status::Paper,
        status::Paper => status::Rock,
        status::Rock => status::Scissors,
        _ =>status::Null,
    }
}

pub(crate) fn solution2() {
    let text = include_str!("../solutions/input/two");

    let games = text.split("\n").collect::<Vec<_>>();

    let mut result = 0;

    // 1 for Rock, 2 for Paper, and 3 for Scissors valgono sempre
    for value in games{
        let temp:Vec<&str> = value.split(" ").collect();
        let player1 = temp[0];
        //Me
        let game_status = temp[1];

        let p1_status = match player1 {
                            "A" => status::Rock,
                            "B" => status::Paper,
                            "C" => status::Scissors,
                            _ => status::Null};

        //println!("{}",game_status);
        //println!("{:#?}",p1_status);
        let game_lost = lost(p1_status);
        let game_win = win(p1_status);
        
        //println!("lost{:#?}-win{:#?}",game_lost,game_win);

        //Mi prende sempre lo stesso valore
        let p2_status = match game_status {
                            "X" =>game_lost,//perdere
                            "Y" => p1_status,//pareggio
                            "Z" => game_win,//vincere
                            _ => status::Null};

        println!("{:#?}-{:#?}",p1_status,p2_status);

        let game = (p1_status,p2_status);

        match game {
            game if game.0 == game.1 => result+=3,
            (status::Rock,status::Paper) => result+=6,
            //(status::Rock,status::Scissors) => (),
            //(status::Paper,status::Rock) => (),
            (status::Paper,status::Scissors) => result+=6,
            (status::Scissors,status::Rock) => result+=6,
            //(status::Scissors,status::Paper) => (),
            _ =>(),
        }

        match p2_status {
            status::Rock => result+=1,
            status::Paper => result+=2,
            status::Scissors => result+=3,
            _ => ()
        } 
    }

    println!("Result solution 2 => {}",result);

}

