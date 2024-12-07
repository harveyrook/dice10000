use std::env;

mod game;

extern crate rand;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_file = "./dice10000.yaml";

    let mut p = game::build_game(&config_file);

    if args[1] == *"stress" {
        p.play_millions();
    }else{
        p.play_all_iterations();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(dice: &Vec<usize>, expected_total: usize) {
        let (actual_total, _) = evaluate_dice(dice);
        assert_eq!(actual_total, expected_total);
    }
    #[test]
    fn internal() {
        run_test(&vec![6, 2, 2, 6, 5, 6], 650);
        run_test(&vec![2, 2, 2, 2, 2, 2], 1500);
        run_test(&vec![1, 1, 1], 1000);
    }
}
