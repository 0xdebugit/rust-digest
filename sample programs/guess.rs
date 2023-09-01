use rand::prelude::*; // 0.8.5
use std::io;

fn main() {
    let rnumber = thread_rng().gen_range(1..20);    
    let mut unumber = String::new();
    
    loop {
        io::stdin().read_line(&mut unumber);
        
        let pnumber: i32 = unumber.trim().parse().unwrap();
        
        if pnumber > rnumber {
            println!("Too High! {}", pnumber);
        }else if pnumber < rnumber {
            println!("Too Low! {}", pnumber);
        }else{
            println!("You guessed it right {}", pnumber);
            break;
        }
    }
    
}