use crate::implied;
use colored::*;
use std::io::{self, Write};
use tabled::{
    settings::{style::Style, themes::Theme, Color},
    Table, Tabled,
};

/// Flush stdout so `print!` (no newline) appears immediately.
pub fn flush_stdout() {
    io::stdout().flush().unwrap();
}

/// Print a decorative separator line.
pub fn print_separator(ch: char, width: usize) {
    let line: String = std::iter::repeat(ch).take(width).collect();
    println!("  {}", line.bright_black());
}

pub fn which_calc_decider(choice: i32, wager: f64) {
    match choice {
        1 => implied::money_prob_calc(wager),
        2 => implied::decimal_prob_calc(wager),
        3 => implied::fraction_prob_calc(wager),
        _ => println!(
            "  {} {}",
            "✗".red().bold(),
            "Please choose a valid option between 1-3.".red()
        ),
    }
}

pub fn display_main_menu() {
    let items = [
        ("1", "Moneyline", "American odds  → probability"),
        ("2", "Decimal", "Decimal odds   → probability"),
        ("3", "Fractional", "Fractional odds → probability"),
    ];

    print_separator('─', 56);
    println!(
        "  {}  {}",
        "🎰",
        "Select a Converter".bright_white().bold()
    );
    print_separator('─', 56);

    for (num, label, desc) in &items {
        println!(
            "   {} {}  {}",
            format!("[{num}]").bright_cyan().bold(),
            format!("{label:<12}").bright_white().bold(),
            desc.bright_black()
        );
    }

    print_separator('─', 56);
}

pub fn get_wager() -> f64 {
    println!();
    print!(
        "  {} {} ",
        "💰".yellow(),
        "Enter your wager (default 100):".bright_white()
    );
    flush_stdout();

    let mut wager = String::new();
    io::stdin()
        .read_line(&mut wager)
        .expect("Failed to read the number");

    if wager.trim().is_empty() {
        println!(
            "  {} {}",
            "ℹ".bright_blue().bold(),
            "Using default wager of $100.00".bright_blue()
        );
        return 100.0;
    }

    match wager.trim().parse::<f64>() {
        Ok(value) if value > 0.0 => {
            println!(
                "  {} Wager set to {}",
                "✓".green().bold(),
                format!("${value:.2}").green().bold()
            );
            value
        }
        Ok(_) => {
            println!(
                "  {} {}",
                "⚠".yellow().bold(),
                "Wager must be positive. Using default $100.00.".yellow()
            );
            100.0
        }
        Err(_) => {
            println!(
                "  {} {}",
                "✗".red().bold(),
                "Invalid input. Using default wager of $100.00.".red()
            );
            100.0
        }
    }
}

#[derive(Tabled)]
struct MetricRow {
    #[tabled(rename = "  Metric")]
    metric: String,
    #[tabled(rename = "Value  ")]
    value: String,
}

pub fn display_metrics_table(implied_prob: f64, payout: f64, return_percentage: f64) {
    let prob_color = if implied_prob >= 50.0 {
        "🟢"
    } else {
        "🔴"
    };

    let return_color = if return_percentage >= 0.0 {
        "📈"
    } else {
        "📉"
    };

    let metrics = vec![
        MetricRow {
            metric: format!("  {} Implied Probability", prob_color),
            value: format!("{implied_prob:.2}%  "),
        },
        MetricRow {
            metric: "  💵 Payout".into(),
            value: format!("${payout:.2}  "),
        },
        MetricRow {
            metric: format!("  {return_color} Return on Bet"),
            value: format!("{return_percentage:.2}%  "),
        },
    ];

    let mut style = Theme::from(Style::rounded());
    style.set_colors_top(Color::FG_CYAN);
    style.set_colors_bottom(Color::FG_CYAN);
    style.set_colors_left(Color::FG_CYAN);
    style.set_colors_right(Color::FG_CYAN);
    style.set_colors_corner_top_left(Color::FG_CYAN);
    style.set_colors_corner_top_right(Color::FG_CYAN);
    style.set_colors_corner_bottom_left(Color::FG_CYAN);
    style.set_colors_corner_bottom_right(Color::FG_CYAN);
    style.set_colors_intersection_bottom(Color::FG_CYAN);
    style.set_colors_intersection_top(Color::FG_CYAN);
    style.set_colors_intersection_right(Color::FG_CYAN);
    style.set_colors_intersection_left(Color::FG_CYAN);
    style.set_colors_intersection(Color::FG_CYAN);
    style.set_colors_horizontal(Color::FG_CYAN);
    style.set_colors_vertical(Color::FG_CYAN);

    let table = Table::new(metrics).with(style).to_string();

    // Indent the table for consistent visual alignment
    for line in table.lines() {
        println!("  {line}");
    }
}
