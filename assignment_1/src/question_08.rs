pub fn run(){
    let result = multiply(5.6,2.4,10.2);
    println!("{}", result);
}

fn multiply (a: f32, b: f32, c: f32) -> f32{
    a*b*c
}