fn round_to_two_decimal_places(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

pub fn moneyline_prob(moneyline: f64) -> f64 {
    let probability = if moneyline > 0.0 {
        (100.0 / (moneyline + 100.0)) * 100.0
    } else {
        (moneyline.abs() / (moneyline.abs() + 100.0)) * 100.0
    };

    round_to_two_decimal_places(probability)
}

fn calculate_prob(odds: f64) -> f64 {
    round_to_two_decimal_places((1.0 / odds) * 100.0)
}

pub fn decimal_prob(decimal: f64) -> f64 {
    calculate_prob(decimal)
}

pub fn fraction_prob(fractions: f64) -> f64 {
    calculate_prob(fractions)
}

pub fn calculate_payout(implied_prob: f64, wager: f64) -> f64 {
    let odds = (1.0 / implied_prob) * 100.0;
    round_to_two_decimal_places(odds * wager)
}

pub fn calculate_percentage_return(payout: f64, wager: f64) -> f64 {
    round_to_two_decimal_places(((payout - wager) / wager) * 100.0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_to_two_decimal_places() {
        assert_eq!(round_to_two_decimal_places(1.23456), 1.23);
        assert_eq!(round_to_two_decimal_places(1.235), 1.24);
        assert_eq!(round_to_two_decimal_places(100.0), 100.0);
    }

    #[test]
    fn test_moneyline_prob_positive() {
        let moneyline: f64 = 150.0;
        let expected_prob = (100.0 / (moneyline + 100.0)) * 100.0;
        assert_eq!(moneyline_prob(moneyline), round_to_two_decimal_places(expected_prob));
    }

    #[test]
    fn test_moneyline_prob_negative() {
        let moneyline: f64 = -150.0;
        let expected_prob = (moneyline.abs() / (moneyline.abs() + 100.0)) * 100.0;
        assert_eq!(moneyline_prob(moneyline), round_to_two_decimal_places(expected_prob));
    }

    #[test]
    fn test_decimal_prob() {
        let decimal: f64 = 2.5;
        let expected_prob = (1.0 / decimal) * 100.0;
        assert_eq!(decimal_prob(decimal), round_to_two_decimal_places(expected_prob));
    }

    #[test]
    fn test_fraction_prob() {
        let fractions: f64 = 5.0 / 2.0;
        let expected_prob = (1.0 / fractions) * 100.0;
        assert_eq!(fraction_prob(fractions), round_to_two_decimal_places(expected_prob));
    }

    #[test]
    fn test_calculate_payout() {
        let prob = 40.0;
        let wager = 100.0;
        let payout = calculate_payout(prob, wager);
        assert_eq!(payout, 250.0);
    }

    #[test]
    fn test_calculate_percentage_return() {
        let payout = 250.0;
        let wager = 100.0;
        assert_eq!(calculate_percentage_return(payout, wager), 150.0);
    }
}