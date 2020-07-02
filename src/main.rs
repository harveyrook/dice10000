use std::env;
extern crate rand;
use rand::Rng;

fn roll_dice( n:usize ) -> Vec<usize>{

    let mut vec = Vec::with_capacity(n);

    for _ in 0..n {
        let num = rand::thread_rng().gen_range(1, 7);    
        vec.push(num);
    }

    vec
}

fn evaluate_dice( all_dice: &Vec<usize>) -> (usize, usize) {
    
    
    // The counts run to 7 (0..6) because it's very confusing to 
    // keep adjusting for the off by one errors... skips 0 and let the the 1 dice goes in slot 1. The six count in slot 6.
    let mut face_counts: [ usize; 7] = [0; 7];
    let mut type_count: [ usize; 7] = [0; 7];
    let mut total:usize = 0;

    let mut used_dice:usize = 0;

    for dice in all_dice{
        face_counts[*dice] = face_counts[*dice]+1;
    }

    for d in 0..7{
        let pos:usize = face_counts[d];
        type_count[pos] += 1;
    }

    // Evaluations...
    // One of each... 1500
    // three pairs... 1500
    // three ones... 1000
    // ones... 100 x #1's
    // five... 50 x #1's (4 5's is 550. 500 for the 3 5's and 50 for the 1 extra 5)

    // for d in 0..6{
    //     print!("{} ", type_count[d]);
    // }
    // println!("");

    if type_count[6] == 1 {
        // six of a kind is 3 pairs
        used_dice += 6;
        total = 1500;
    } else if type_count[4] == 1 && type_count[2]==1 {
        // 4 of a kind and a pair is 3 pairs. 
        total = 1500;
        used_dice +=6;
    } else if type_count[2] == 3 {
        // 3 pairs
        total = 1500;
        used_dice +=6;
    } else if type_count[6] == 1 {
        
    }
    else
    {
        

        for face in 1..7{

            let mut face_count = face_counts[face];

            // print!("face_count {} face_counts {} total {} ", face_count, face_countss, total);

            match face {

                1 => {
                    used_dice += face_count;
                    if face_count >= 3 {
                        total += 1000;
                        face_count -= 3;
                    
                    }

                    if face_count > 0 {
                        total += face_count*100;
                    }

                },
                5 => {
                    used_dice += face_count;
                    if face_count >= 3 {
                        total += 500;
                        face_count -= 3;
                    
                    }

                    if face_count > 0 {
                        total += face_count*50;
                    }

                },

                _ => {
                    if face_count >= 3 {
                        total += face * 100;
                        face_count -= 3;
                        used_dice += 3;
                    }

                    if face_count >= 3 {
                        total += face * 100;
                        used_dice += 3;
                    }
                }
               
            }

            // print!("face_count {} face_counts {} total {} ", face_count, face_countss, total);
            // println!("");

        }
    }

    (total, used_dice)
}

// This function keeps rolling dize until a zero is rolled

fn play_hand( soft_stop:usize) -> usize {
    let mut run_total = 0;
    let mut num_dice = 6;
    let mut _run_count = 0;

    loop{

        let dice = roll_dice( num_dice );
        let (run_last_total, used_dice ) = evaluate_dice( &dice );

        run_total += run_last_total;

        _run_count += 1;

        // de we go bust?
        if run_last_total == 0 {
            run_total = 0;
            break;
        }

        num_dice -= used_dice;

        if num_dice == 3 && run_total >= soft_stop{
            break;
        }

        if num_dice < 3 {
            break;
        }

        if num_dice == 0{
            num_dice = 6;
        }

    }

    //println!("{} {}", run_total, run_count );

    run_total

}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut times = 1;
    let mut num_dice:usize = 6;
    let mut soft_stop:usize = 0;
    if args.len() >= 2 {

        times = args[1].parse::<usize>().unwrap();
    }

    if args.len() >= 3 {
        num_dice = args[2].parse::<usize>().unwrap();
    }

    if args.len() >= 4 {
        soft_stop = args[3].parse::<usize>().unwrap();
    }

    // println!("{:?}", args);

    if soft_stop > 0 {
        let mut total: usize = 0;
        for _ in 0..times{
            total += play_hand(soft_stop);
        }

        let average = total/times;
        println!("Average Hand: {}", average);
 
    }else{
        for _ in 0..times{
            let dice = roll_dice( num_dice );

            for i in 0..num_dice{
                print!("{}, ", dice[i]);
            }

            // println!("");

            let (total, all) = evaluate_dice( &dice );

            println!("{}, {}", total, all);
        }
    }
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



