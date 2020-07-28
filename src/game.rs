extern crate rand;
extern crate yaml_rust;

use yaml_rust::{YamlLoader};
use rand::distributions::{Distribution, Uniform};
use std::fs::File;
use std::io::prelude::*;
use std::io;

enum MyError {
        FileReadError
}

impl From<io::Error> for MyError {
        fn from(e: io::Error) -> MyError {
            MyError::FileReadError
        }
}

pub struct Settings{
    print_details:bool,
}

fn load_settings() -> Result<Settings,MyError>{

    println!("loading settings");
    let mut file = File::open("dice1000.yaml")?;
    println!("about to load file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("read_to_string");
    let docs = YamlLoader::load_from_str(&contents).unwrap();
    println!("Docs... {:?}", docs);

    let s = Settings{
        print_details: false,
    };

    Ok(s)
}



fn roll_dice( n:usize ) -> Vec<usize>{

    let step = Uniform::new(1, 7);
    let mut rng = rand::thread_rng();
    let choices: Vec<usize> = step.sample_iter(&mut rng).take(n).collect();

    choices
}

fn print_dice( all_dice: &Vec<usize>) {
    for dice in all_dice{
        print!("{} ", *dice);
    }

    println!("");
}


pub struct Dice10000
{
    player: Vec<Player>,

}

pub fn build_game() -> Dice10000{
    load_settings();
    let mut p: Vec<Player> = Vec::new();
    p.push(build_player(1));
    p.push(build_player(2));
    p.push(build_player(3));

    let d = Dice10000{ player: p};

    d
}

impl Dice10000 {
    pub fn play(& mut self){
        let mut keep_going = true;
        let mut remaining_times = self.player.len();

        while keep_going || remaining_times > 0 {
            for p in self.player.iter_mut() {
                p.play_hand(300, 1);
                println!("Player:{} score:{}", p.player_id, p.score);
                if p.score >= 10000{
                    keep_going = false;
                    println!("Player {} crossed 10000", p.player_id);
                }
                if !keep_going {
                    //println!("Remaining times {}", remaining_times);
                    remaining_times = remaining_times - 1;
                    if remaining_times == 0{
                        break;
                    }
                }
            }
        }
    }
}

pub struct Player {
    player_id: usize,
    on_board: bool,
    score: usize,
    soft_limit: usize,
}
    
pub fn build_player( id: usize ) -> Player {
    Player {
        player_id: id,
        on_board: false,
        score: 0,
        soft_limit: 300
    }
}

struct DiceAnalysis {
    total: usize,
    used_dice: usize,
    five_count: usize,
    two_count: usize,

}

fn evaluate_dice( all_dice: &Vec<usize>) -> DiceAnalysis {
    
    
    // The counts run to 7 (0..6) because it's very confusing to 
    // keep adjusting for the off by one errors... skips 0 and let the the 1 dice goes in slot 1. The six count in slot 6.
    let mut face_counts: [ usize; 7] = [0; 7];
    let mut type_count: [ usize; 7] = [0; 7];

    let mut da = DiceAnalysis{ total:0, used_dice:0, five_count:0, two_count:0 };

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
        da.used_dice += 6;
        da.total = 1500;
    } else if type_count[4] == 1 && type_count[2]==1 {
        // 4 of a kind and a pair is 3 pairs. 
        da.total = 1500;
        da.used_dice +=6;
    } else if type_count[2] == 3 {
        // 3 pairs
        da.total = 1500;
        da.used_dice +=6;
    } else if type_count[6] == 1 {
       da.total = 1500;
       da.used_dice +=6;
    }
    else
    {
        

        for face in 1..7{

            let mut face_count = face_counts[face];

            // print!("face_count {} face_counts {} total {} ", face_count, face_countss, total);

            match face {

                1 => {
                    da.used_dice += face_count;
                    if face_count >= 3 {
                        da.total += 1000;
                        face_count -= 3;
                    
                    }

                    if face_count > 0 {
                        da.total += face_count*100;
                    }
                
                },
                5 => {
                    da.used_dice += face_count;
                    if face_count >= 3 {
                        da.total += 500;
                        face_count -= 3;
                    
                    }

                    if face_count > 0 {
                        da.total += face_count*50;
                    }

                },

               _ => {
                    if face_count >= 3 {
                        da.total += face * 100;
                        face_count -= 3;
                        da.used_dice += 3;
                    }

                    if face_count >= 3 {
                        da.total += face * 100;
                        da.used_dice += 3;
                    }
                }
               
            }

            // print!("face_count {} face_counts {} total {} ", face_count, face_countss, total);
            // println!("");

        }
    }

    da.five_count = face_counts[5];
    da.two_count = face_counts[2];

    return da;
}

// This function keeps rolling dize until a zero is rolled
impl Player {

    fn play_hand( &mut self, soft_stop:usize, skip_fives:usize ) -> usize {
        let mut run_total = 0;
        let mut num_dice = 6;
        let mut _run_count = 0;

        loop{

            let dice = roll_dice( num_dice );
            print_dice( &dice );
            let mut da = evaluate_dice( &dice );

            // If we are rolled 3, 4 5 or 6 dice and two fives are returned then...
            // pull 1 five out and roll with it, or...
            // pull all fives out if we scored on something else
            //if( run_last_total == 100 && five_count == 2)
            //{
            //    run_last_total = 50;
            //    used_dice = 1;
            //}
            //else 
            //{
            //
            //}
            //
            //

            if skip_fives > 0 {
                // We are testing the theory that its a good thing to have more dice to role. As such
                // we should omit counting fives when we have a chance. 
                // Cases...
                // 1. Rolled two fives. In which case, only keep one five.
                // 2. Rolled something else and one or two fives. In which case keep the something
                //    else.
                // 3. Rolled 4 or 5 fives. In which case keep 3 fives.
                //
                //
                //
                //

                while da.total >= 100 && da.five_count > 0 && da.five_count < 3 {
                    da.five_count -= 1;
                    da.used_dice -= 1;
                    da.total -= 50;
                }
            }

            num_dice -= da.used_dice;
            run_total += da.total;

            _run_count += 1;

            // de we go bust?
            if da.total == 0 {
                run_total = 0;
                break;
            }

            // If we are rolled 3, 4 5 or 6 dice and two fives are returned then...
            // pull 1 five out and roll with it, or...
            // pull all fives out if we scored on something else
            //if( num_dice > 2 && five_count == 2 && used_dice == 2 )
            //{
            //    
            //}

            if self.on_board {
                if num_dice == 3 && run_total >= soft_stop{
                    break;
                }

                if num_dice < 3 {
                    break;
                }
            }else{
                if run_total >= 750 {
                    self.on_board = true;
                    break;
                }
            }

            if num_dice == 0{
                num_dice = 6;
            }

        }

        println!("run total {} ", run_total );
        self.score += run_total;
        run_total

    }
}

