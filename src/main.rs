use rand::random; // 0.8.4

fn main() {
    //let mut graves = Vec::new();
    gamble_sim(100000, 96);
}

pub fn gamble_sim(to_gen: usize, bar_len: usize) {
    let mut winner = Gambler::new(0, 0);

    for i in 1..=to_gen {
        let mut gamer = Gambler::new(1000, 1);
        gamer.gamble();
        // graves.push(gamer);
        if gamer.peak_money > winner.peak_money {
            winner = gamer;
            print!("\x1b[1B\nCurrent leader:\n{}\nGeneration:\n", winner);
            update(i, to_gen, bar_len, true)
        } else {
            update(i, to_gen, bar_len, false)
        }
    }

    println!("\x1b[1B\nWinnerrrr:\n{}", winner);
}

#[derive(Debug)]
struct Gambler {
    money: usize,
    start_bet: usize,
    bet: usize,
    flips: usize,
    peak_money: usize,
    successses: usize,
    failures: usize,
}

impl Gambler {
    fn new(money: usize, start_bet: usize) -> Self {
        Self {
            money,
            start_bet,
            bet: start_bet,
            flips: 0,
            peak_money: money,
            successses: 0,
            failures: 0,
        }
    }

    fn flip(&mut self) -> bool {
        if self.bet > self.money {
            self.money = 0;
            return false;
        }

        let successs: bool = random();

        if successs {
            self.money += self.bet;
            self.successses += 1;
        } else {
            self.money -= self.bet;
            self.failures += 1;
        }

        successs
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
            "Money: {}
Start bet: {}
Killer bet: {}
Flips survived: {}
Successful flips: {}
Unsuccessful flips: {}
Peak money: {}",
            self.money,
            self.start_bet,
            self.bet,
            self.flips,
            self.successses,
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
    if new_bar != gen_bar(value - 1, old_top, bar_len) || refresh {
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
