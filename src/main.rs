use std::env;

mod game;

extern crate rand;

fn main() {

    let _args: Vec<String> = env::args().collect();
    let config_file = "./dice10000.yaml"; 
    //let mut num_dice:usize = 6;
    //let mut soft_stop:usize = 0;
    //let mut skip_fives:usize = 0;
    //if args.len() >= 2 {

    //println!("{:?}", args.len());

    //if args.len() >= 2{
    // let mut config_file = &args[1].parse::<String>().unwrap_or( "./dice10000.yaml".to_string());
    //}
    //}

    //if args.len() >= 3 {
    //    num_dice = args[2].parse::<usize>().unwrap();
    //}

    //if args.len() >= 4 {
    //    soft_stop = args[3].parse::<usize>().unwrap();
    //}

    //if args.len() >= 5 {
    //    skip_fives = args[4].parse::<usize>().unwrap();
    //}

    let mut p = game::build_game(&config_file);
    p.play_all_iterations();

    //if soft_stop > 0 { 
    //
    //    let mut total: usize = 0;
    //    for _ in 0..times{
    //        total += play_hand(soft_stop, skip_fives);
    //    }

    //    let average = total/times;
    //    println!("Average Hand: {}", average);
 
    //}else{
    //    for _ in 0..times{
    //        let dice = roll_dice( num_dice );

    //        for i in 0..num_dice{
    //            print!("{}, ", dice[i]);
    //        }

    //        // println!("");

    //        let da = evaluate_dice( &dice );

    //        println!("{}, {}", da.total, da.used_dice);
    //    }
    //}
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test( dice: &Vec<usize>, expected_total:usize){
        let (actual_total,_) = evaluate_dice( dice );
        assert_eq!(actual_total, expected_total );
    }
    #[test]
    fn internal() {
   
        run_test(&vec![6,2,2,6,5,6], 650);
        run_test(&vec![2,2,2,2,2,2], 1500);
        run_test(&vec![1,1,1], 1000);
    }

}



