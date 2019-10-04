pub fn run() {
    for x in 1..20 {
        if x % 2 == 0 { continue; }

        println!("{}", x);
    }
}