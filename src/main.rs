use rand::seq::SliceRandom;

use std::{
    fmt::Display,
    fs::{self, read_to_string},
    io::{self, Write},
    process::exit,
};

fn input() -> String {
    let mut ret = String::new();
    io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read a line of input");
    ret
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Card {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
}


impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Card::Ace => write!(f, "ace"),
            Card::Two => write!(f, "two"),
            Card::Three => write!(f, "three"),
            Card::Four => write!(f, "four"),
            Card::Five => write!(f, "five"),
            Card::Six => write!(f, "six"),
            Card::Seven => write!(f, "seven"),
            Card::Eight => write!(f, "eight"),
            Card::Nine => write!(f, "nine"),
            Card::Ten => write!(f, "king"),
        }
    }
}

struct RandCardGenerator {
    cards: Vec<Card>,
}

impl RandCardGenerator {
    fn new() -> RandCardGenerator {
        let mut rng = rand::thread_rng();

        let mut cards: Vec<Card> = vec![
            Card::Ace,
            Card::Two,
            Card::Three,
            Card::Four,
            Card::Five,
            Card::Six,
            Card::Seven,
            Card::Eight,
            Card::Nine,
            Card::Ten, // tens
            Card::Ten, // king
            Card::Ten, // queen
            Card::Ten, // jack
        ].repeat(4);

        cards.shuffle(&mut rng);

        RandCardGenerator {
            cards,
        }
    }

    pub fn get_card(&mut self) -> Card {
        self.cards.pop().unwrap() // UNWRAP SAFETY: Its impossible to pull 52 cards
    }
}

fn get_vals(cards: &Vec<Card>) -> usize {
    let mut ret = 0;
    let mut has_ace = false;
    for card in cards {
        ret += *card as usize;
        has_ace = (*card == Card::Ace) | has_ace;
    }
    if ret+10 < 21 && has_ace && ret < 21{
        ret += 10;
    }
    ret
}

fn main() {
    match fs::File::open("money.txt") {
        Ok(_) => (),
        Err(_) => {
            fs::File::create("money.txt").unwrap();
        }
    }
    let money = read_to_string("money.txt").unwrap();
    let money_int;

    match money.trim().parse::<usize>() {
        Ok(a) => money_int = a,
        Err(_) => {
            println!("reset");
            fs::write("money.txt", b"100").unwrap();
            money_int = 100;
        }
    }

    println!("You have {}\n\nPlace your bet: ", money_int);
    let bet: usize;
    loop {
        match input().trim().parse() {
            Ok(n) => {
                if n <= money_int {
                    bet = n;
                } else {
                    println!("You cannot bet more than you have.");
                    continue;
                }
                break;
            }
            Err(_) => {
                println!("Enter a valid number.");
            }
        }
        println!("Place your bet: ");
    }
    let mut rng = RandCardGenerator::new();
    let mut dealer_cards = Vec::new();
    for _ in 0..5 {
        let card = rng.get_card();
        if card as usize + get_vals(&dealer_cards) < 21 {
            dealer_cards.push(card);
        }
    }

    let dealer_val = get_vals(&dealer_cards);
    let mut user_cards = Vec::new();
    for _ in 0..2 {
        let card = rng.get_card();
        user_cards.push(card);
    }
    loop {
        print!(
            "Your cards are: {:?} with a value of {}\nPick up? (y/n) ",
            user_cards,
            get_vals(&user_cards)
        );
        io::stdout().flush().unwrap();
        let card = rng.get_card();
        match input().to_lowercase().trim() {
            "y" => user_cards.push(card),
            "n" => break,
            _ => continue,
        }
        let val = get_vals(&user_cards);
        if val > 21 {
            println!(
                "You went over at {}! You now have ${}",
                get_vals(&user_cards),
                money_int - bet
            );
            fs::write("money.txt", format!("{}\n", money_int - bet).as_bytes()).unwrap();
            exit(0);
        } else if val == 21 {
            println!("You hit 21! You now have ${}", money_int + bet);
            fs::write("money.txt", format!("{}\n", money_int + bet).as_bytes()).unwrap();
            exit(0);
        }
    }

    if get_vals(&user_cards) > dealer_val || get_vals(&user_cards) == 21 {
        println!("You win! You now have ${}", money_int + bet);
        fs::write("money.txt", format!("{}\n", money_int + bet).as_bytes()).unwrap();
    } else if dealer_val == get_vals(&user_cards) {
        println!("Tie at {}. You lost no money", dealer_val);
    } else {
        println!(
            "You loose! The dealer had {} and you had {}.\n You now have ${}",
            dealer_val,
            get_vals(&user_cards),
            money_int - bet
        );
        fs::write("money.txt", format!("{}\n", money_int - bet).as_bytes()).unwrap();
    }
}
