#![warn(clippy::all, clippy::pedantic)]

use rand::random; // 0.8.4
use scottish_names::{Sex, first_name, surname}; // 0.2.2
use titlecase::titlecase; // 1.0

use std::sync::mpsc;
use std::thread;

fn main() {
    //let mut graves = Vec::new();
    // gamble_sim(10_000_000, 190, 500, false);

    let args: Vec<String> = std::env::args().collect();
    let args: (usize, usize, usize, bool) = (
        // to generate
        args.get(1).map_or(10000, |gens| gens.parse().expect("could not parse amount to generate!")),
        // bar length (-2 :S)
        args.get(2).map_or(190, |bar_len| bar_len.parse().expect("could not parse amount to generate!")),
        // threads to spawn
        args.get(3).map_or(10, |threads| threads.parse().expect("could not parse amount to generate!")),
        // whether to show cool progress bar and updating generation count
        args.get(4).map_or(true, |logging| logging != "false")
    );

    gamble_sim(args.0, args.1, args.2, args.3);
}

/// # Panics
/// stop
pub fn gamble_sim(to_gen: usize, bar_len: usize, threads: usize, show_progress: bool) {
    let (tx, rx) = mpsc::channel();

    for _ in 0..threads {
        let thread_tx = tx.clone();

        thread::spawn(move || {
            for _ in 1..=(to_gen / threads) {
                let mut gamer = Gambler::new(1000, 1);
                gamer.gamble();
                // graves.push(gamer);
                thread_tx.send(gamer).unwrap();
            }
            drop(thread_tx);
        });
    }
    drop(tx);

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx2.send(val).unwrap();
    // });

    let mut winner = Gambler::new(0, 0);

    let mut generation = 0;
    while let Ok(gamer) = rx.recv() {
        generation += 1;
        if gamer.peak_money > winner.peak_money {
            winner = gamer;
            print!("\x1b[1B\nCurrent leader:\n{}\nGeneration:\n", winner);
            update(generation, to_gen, bar_len, true);
        } else if show_progress && generation % threads == 0 {
            update(generation, to_gen, bar_len, false);
        }
    }

    println!("\x1b[1B\nWinnerrrr:\n{}", winner);
}

#[derive(Debug)]
struct Gambler {
    name: String,
    money: usize,
    start_bet: usize,
    bet: usize,
    flips: usize,
    peak_money: usize,
    successes: usize,
    failures: usize,
}

impl Gambler {
    fn new(money: usize, start_bet: usize) -> Self {
        Self {
            name: format!("{} {}", first_name(if random() {
                Sex::Male
            } else {
                Sex::Female
            }), titlecase(surname())),
            // name: String::from(""),
            money,
            start_bet,
            bet: start_bet,
            flips: 0,
            peak_money: money,
            successes: 0,
            failures: 0,
        }
    }

    fn flip(&mut self) -> bool {
        if self.bet > self.money {
            self.money = 0;
            return false;
        }

        let success: bool = random();

        if success {
            self.money += self.bet;
            self.successes += 1;
        } else {
            self.money -= self.bet;
            self.failures += 1;
        }

        success
    }

    fn gamble(&mut self) {
        while self.money > 0 {
            self.flips += 1;
            if self.flip() {
                self.bet = self.start_bet;
                self.peak_money = self.money;
            } else {
                self.bet *= 2;
            }
        }
    }
}

impl std::fmt::Display for Gambler {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Name: {}
Money: {}
Start bet: {}
Killer bet: {}
Flips survived: {}
Successful flips: {}
Unsuccessful flips: {}
Peak money: {}",
            self.name,
            self.money,
            self.start_bet,
            self.bet,
            self.flips,
            self.successes,
            self.failures,
            self.peak_money
        )
    }
}
// impl flip for Gambler {
//     println!("hi");
// }

fn gen_bar(value: usize, old_top: usize, bar_len: usize) -> String {
    format!(
        "[{:.<bar_len$}]",
        "#".repeat(value / (old_top / bar_len)),
        bar_len = bar_len
    )
}

fn update_bar(value: usize, old_top: usize, bar_len: usize, refresh: bool) -> bool {
    let new_bar = gen_bar(value, old_top, bar_len);
    if refresh || new_bar != gen_bar(value - 1, old_top, bar_len) {
        if !refresh {
            print!("\x1b[1B");
        }
        print!("\x1b[0G\x1b[K{}", new_bar);
        return true;
    }
    false
}

fn update(value: usize, old_top: usize, bar_len: usize, refresh: bool) {
    let bar_updated = update_bar(value, old_top, bar_len, refresh);
    if bar_updated {
        print!("\x1b[1A");
    }
    print!(
        "\x1b[13G{:0max_len$}/{}",
        value,
        old_top,
        max_len = &old_top.to_string().len()
    );
}
