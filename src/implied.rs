use crate::{prob_functions, utils};
use colored::*;
use std::io;

pub fn decimal_prob_calc(wager: f64) {
    print!(
        "  {} {} ",
        "▶".bright_cyan().bold(),
        "Enter the Decimal Odds:".bright_white()
    );
    utils::flush_stdout();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read Decimal Odds");

    match input.trim().parse::<f64>() {
        Ok(value) if value > 1.0 => {
            print_calculating();
            let prob = prob_functions::decimal_prob(value);
            let payout = prob_functions::calculate_payout(prob, wager);
            let return_on_bet = prob_functions::calculate_percentage_return(payout, wager);
            utils::display_metrics_table(prob, payout, return_on_bet);
        }
        Ok(value) if value > 0.0 => {
            println!(
                "  {} {}",
                "⚠".yellow().bold(),
                "Decimal odds must be greater than 1.0.".yellow()
            );
        }
        Ok(_) | Err(_) => {
            println!(
                "  {} {}",
                "✗".red().bold(),
                "Please enter a valid numeric value for Decimal Odds.".red()
            );
        }
    }
}

pub fn money_prob_calc(wager: f64) {
    print!(
        "  {} {} ",
        "▶".bright_cyan().bold(),
        "Enter the Moneyline (e.g. +150 or -200):".bright_white()
    );
    utils::flush_stdout();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read Moneyline");

    match input.trim().parse::<f64>() {
        Ok(value) if value != 0.0 => {
            print_calculating();
            let prob = prob_functions::moneyline_prob(value);
            let payout = prob_functions::calculate_payout(prob, wager);
            let return_on_bet = prob_functions::calculate_percentage_return(payout, wager);
            utils::display_metrics_table(prob, payout, return_on_bet);
        }
        Ok(_) => {
            println!(
                "  {} {}",
                "⚠".yellow().bold(),
                "Moneyline cannot be zero.".yellow()
            );
        }
        Err(_) => {
            println!(
                "  {} {}",
                "✗".red().bold(),
                "Please enter a valid integer for Moneyline Odds.".red()
            );
        }
    }
}

pub fn fraction_prob_calc(wager: f64) {
    print!(
        "  {} {} ",
        "▶".bright_cyan().bold(),
        "Enter the Fractional Odds (e.g. 3/4):".bright_white()
    );
    utils::flush_stdout();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read Fractional Odds");

    let trimmed = input.trim();
    let parts: Vec<&str> = trimmed.split('/').collect();

    if parts.len() != 2 {
        println!(
            "  {} {}",
            "✗".red().bold(),
            "Invalid format. Please enter as numerator/denominator (e.g. 3/4).".red()
        );
        return;
    }

    match (
        parts[0].trim().parse::<f64>(),
        parts[1].trim().parse::<f64>(),
    ) {
        (Ok(num), Ok(den)) if den != 0.0 && num >= 0.0 && den > 0.0 => {
            print_calculating();
            let value = num / den;
            let prob = prob_functions::fraction_prob(value);
            let payout = prob_functions::calculate_payout(prob, wager);
            let return_on_bet = prob_functions::calculate_percentage_return(payout, wager);
            utils::display_metrics_table(prob, payout, return_on_bet);
        }
        (Ok(_), Ok(_)) => {
            println!(
                "  {} {}",
                "⚠".yellow().bold(),
                "Both numerator and denominator must be positive, and denominator cannot be zero."
                    .yellow()
            );
        }
        _ => {
            println!(
                "  {} {}",
                "✗".red().bold(),
                "Invalid fraction values. Please enter numeric values.".red()
            );
        }
    }
}

fn print_calculating() {
    println!();
    println!(
        "  {} {}",
        "⏳",
        "Calculating...".bright_green().bold()
    );
    println!();
}
