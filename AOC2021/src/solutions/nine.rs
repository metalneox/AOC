pub(crate) fn solution1() {
    let binding =
        std::fs::read_to_string("src/solutions/input/_test").expect("file not found!");
    let vec_data: Vec<_> = binding.lines().map(|x| x).collect();


    //let matrix: Vec<Vec<u32>> = std::fs::read_to_string("src/solutions/input/three")
    //    .expect("file not found!")
    //    .lines()
    //    .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
    //    .collect::<Vec<Vec<u32>>>();




    println!("{:#?}",vec_data);
}