use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guessing Game");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    loop {
    println!("Please input your guessed string ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");
    let guess: u32 = match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue,
    };
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Tooo smallllll"),
        Ordering::Equal =>{ 
                    println!("Equal, You win");
                    break;
                }
        Ordering::Greater => println!("Tooooooo biggggggg"),
    };
    }
}
