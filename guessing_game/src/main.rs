use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Make a guess!");

        let mut guess = String::new();

        println!(">>");
        io::stdin().read_line(&mut guess).expect("❌ Failure.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Low."),
            Ordering::Greater => println!("Too High"),
            Ordering::Equal => {
                println!("🏁 Win!");
                break;
            }
        }
    }
}
