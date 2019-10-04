pub fn run(){
    let year = 2020;

    if year%4 == 0 {
        println!("{} is leap year.", year);
    } else {
        println!("{} is not leap year.", year);
    }
}