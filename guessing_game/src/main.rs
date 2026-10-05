use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("welcome to the guessing game");
    loop{
    println!("guess a number");
    let secret_number=rand::thread_rng().gen_range(1,101);
    println!("the secret number is:{}",secret_number);
    let mut guess= String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("please input correctly");
    println!("you guess:{}",guess);
    let guess:u32 = guess.trim().parse().expect("enter a number");
    match guess.cmp(&secret_number){
        Ordering::Less =>println!("Too small!"),
        Ordering::Greater =>println!("Too Big"),
        Ordering::Equal => {
            println!("You Win");
            break;
        }
        }
    }
}
