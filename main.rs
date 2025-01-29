use std::io;
fn main(){
    println!("Hithesh");
    let x="5";

    let mut guess =String:: new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    let guess= guess.trim();

    if (x==guess){
        println!("You guessed it right");
    }
    else{
        println!("You guessed it wrong");
    }

    println!("You guessed: {}", guess);
}