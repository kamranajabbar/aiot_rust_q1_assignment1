pub fn run(){
    let marks = 62;

    if marks >= 80 {
        println!("Grade of student A+");
    } else if marks >= 70 && marks < 80 {
        println!("Grade of student A");
    } else if marks >= 60 && marks < 70 {
        println!("Grade of student B");
    } else if marks >= 50 && marks < 60 {
        println!("Grade of student C");
    } else if marks >= 40 && marks < 50 {
        println!("Grade of student D");
    } else {
        println!("Grade of student F");
    }
}


// When marks = 95;       Output = Grade of student A+
// When marks = 32;       Output = Grade of student F
// When marks = 62;       Output = Grade of student B