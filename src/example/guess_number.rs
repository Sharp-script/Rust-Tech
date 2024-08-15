use std::cmp::Ordering;
use std::io;
use rand::Rng;

pub fn guess_number() {
    println!("Let's guess the number!");

    let rand_num: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess_number = String::new();

        println!("Please enter your guess number: ");
        io::stdin()
            .read_line(&mut guess_number)
            .expect("Failed to guess");

        let guess_number : u32 = match guess_number.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess_number.cmp(&rand_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("❀❀❀❀❀ Win ❀❀❀❀❀");
                break;
            }
        }
    }
}