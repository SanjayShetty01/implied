use colored::*;
mod implied;
mod prob_functions;
mod utils;
use figlet_rs;
use std::io;

fn main() {
    utils::print_separator('═', 56);
    let standard_font = figlet_rs::FIGfont::standard().unwrap();
    if let Some(banner) = standard_font.convert("Implied") {
        print!("{}", format!("{banner}").cyan().bold());
    }
    println!(
        "  {}  {}",
        "⚡".yellow(),
        "Betting Odds → Implied Probability Calculator"
            .bright_white()
            .bold()
    );
    utils::print_separator('═', 56);
    println!();

    let mut wager = utils::get_wager();

    loop {
        println!();
        utils::display_main_menu();

        let mut num = String::new();
        print!(
            "  {} ",
            "▶ Your choice:".bright_cyan().bold()
        );
        utils::flush_stdout();
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read the number");

        let parsed_num = num.trim().parse::<i32>();

        match parsed_num {
            Ok(value) => {
                println!();
                utils::which_calc_decider(value, wager);
            }
            Err(_) => {
                println!(
                    "  {} {}",
                    "✗".red().bold(),
                    "Invalid option! Please enter 1, 2, or 3.".red()
                );
            }
        };

        println!();
        utils::print_separator('─', 56);
        println!(
            "  {}  {}   {}   {}",
            "↩".bright_yellow(),
            "[Enter] Continue".bright_white(),
            "[C] Change wager".bright_yellow(),
            "[X] Exit".bright_red()
        );
        utils::print_separator('─', 56);
        print!(
            "  {} ",
            "▶".bright_cyan().bold()
        );
        utils::flush_stdout();

        let mut final_call = String::new();
        io::stdin()
            .read_line(&mut final_call)
            .expect("Failed to read input");

        if final_call.trim().eq_ignore_ascii_case("x") {
            println!();
            utils::print_separator('═', 56);
            println!(
                "  {}  {}",
                "👋",
                "Thanks for using Implied! Good luck on your bets."
                    .bright_green()
                    .bold()
            );
            utils::print_separator('═', 56);
            break;
        } else if final_call.trim().eq_ignore_ascii_case("c") {
            wager = utils::get_wager();
        }
    }
}
