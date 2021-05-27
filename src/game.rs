use rand::distributions::{Distribution, Uniform};
use std::fs::File;
use std::rc::Rc;


use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    write_details: bool,
    iterations: usize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerConfig {
    id: usize,
    skip_fives: usize,
    skip_twos: usize,
    limits: [usize; 7],
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AllSettings {
    config: Settings,
    players: Vec<PlayerConfig>,
}

static TRACE_ON: bool = false;

fn load_settings(config_file: &str) -> AllSettings {
    let file = File::open(config_file).unwrap();
    let all_settings: AllSettings = serde_yaml::from_reader(&file).unwrap();

    //TRACE_ON = all_settings.config.write_details;

    all_settings
}

fn roll_dice(n: usize) -> Vec<usize> {
    let step = Uniform::new(1, 7);
    let mut rng = rand::thread_rng();
    let choices: Vec<usize> = step.sample_iter(&mut rng).take(n).collect();

    choices
}

fn print_dice(all_dice: &Vec<usize>) {
    for dice in all_dice {
        if TRACE_ON {
            print!("{} ", *dice)
        };
    }

    if TRACE_ON {
        println!("")
    };
}

pub struct Dice10000 {
    config: Settings,
    player: Vec<Player>,
}

pub fn build_game(config_file: &str) -> Dice10000 {
    let all_settings = load_settings(config_file);
    //let mut p: Vec<Player> = Vec::new();
    //for player_config in all_settings.players {
    //    p.push(build_player(player_config));
    //}

    let p = all_settings.players.iter().map( |p| build_player(p)).collect();

    let d = Dice10000 {
        config: all_settings.config,
        player: p,
    };

    d
}

impl Dice10000 {
    pub fn play_all_iterations(&mut self) {

        for a in 0..self.config.iterations {
            self.play((a+2) % self.player.len());

            // Find the highest score
            let score_winner = self.player.iter().
                map( |p| p.score ).
                max().
                unwrap();

            self.player = self.player.iter().map( |p| update_player( p, score_winner )).collect();

        }

        for p in self.player.iter_mut() {
            println!(
                "Player {} won {} times",
                p.player_config.id, p.life_time_wins
            );
        }
    }

    pub fn play(&mut self, mut starter: usize) {
        let mut keep_going = true;
        let mut remaining_times = self.player.len();

        while keep_going || remaining_times > 0 {
            for p in self.player.iter_mut() {
                if starter > 0 {
                    starter = starter - 1;
                    continue;
                }

                p.play_hand();
                if TRACE_ON {
                    println!("Player:{} score:{}", p.player_config.id, p.score);
                }
                if p.score >= 10000 {
                    keep_going = false;
                    if TRACE_ON {
                        println!("Player {} crossed 10000", p.player_config.id);
                    }
                }
                if !keep_going {
                    //println!("Remaining times {}", remaining_times);
                    remaining_times = remaining_times - 1;
                    if remaining_times == 0 {
                        break;
                    }
                }
            }
        }
    }
}

pub struct Player {
    player_config: Rc<PlayerConfig>,
    on_board: bool,
    score: usize,
    life_time_wins: usize,
}

pub fn build_player(player_config_param: &PlayerConfig) -> Player {
    Player {
        player_config: Rc::new(player_config_param.clone()),
        on_board: false,
        score: 0,
        life_time_wins: 0,
    }
}

pub fn update_player( old_player: &Player, high_score: usize ) -> Player {

    Player {
        player_config: old_player.player_config.clone(),
        on_board: false,
        score: 0,
        life_time_wins: if old_player.score == high_score {
                            old_player.life_time_wins + 1
                        } else {
                            old_player.life_time_wins
                        }
    }        
}


struct DiceAnalysis {
    total: usize,
    used_dice: usize,
    five_count: usize,
    two_count: usize,
}

fn evaluate_dice(all_dice: &Vec<usize>) -> DiceAnalysis {
    // The counts run to 7 (0..6) because it's very confusing to
    // keep adjusting for the off by one errors... skips 0 and let the the 1 dice goes in slot 1. The six count in slot 6.

    //print_dice( all_dice );

    let mut da = DiceAnalysis {
        total: 0,
        used_dice: 0,
        five_count: 0,
        two_count: 0,
    };

    let face_counts = all_dice.iter().fold( [0; 7], | mut acc, x | { acc[*x] += 1; acc } );
    let type_count = (0..7).fold( [0; 7], | mut acc, x | { acc[face_counts[x]] += 1 ; acc } ); 

    // Evaluations...
    // One of each... 1500
    // three pairs... 1500
    // three ones... 1000
    // ones... 100 x #1's
    // five... 50 x #1's (4 5's is 550. 500 for the 3 5's and 50 for the 1 extra 5)

    if type_count[6] == 1 // six of a kind is 3 pairs
        || type_count[4] == 1 && type_count[2] == 1  // 4 of a kind and a pair is 3 pairs.
        || type_count[2] == 3 // 3 pairs
        || type_count[6] == 1 // 1 of each face
    {
        da.used_dice += 6;
        da.total = 1500;
    } else {
        for face in 1..7 {

            // print!("face_count {} face_counts {} total {} ", face_count, face_countss, total);

            let regular_calc = | fcp, b, m, dr: & mut DiceAnalysis | {
                let mut fc = fcp;

                if fc>=3 && b>0 {
                    dr.total += b;
                    fc -= 3;
                    dr.used_dice += 3;
                }

                if fc>0 && m>0{
                    dr.total += m*fc;
                    dr.used_dice += fc;
                    //fc = 0;
                }

            };

            let face_count = face_counts[face];
            match face {
                1 => {
                    regular_calc( face_count, 1000, 100, &mut da);
                }
                5 => {
                    regular_calc( face_count, 500, 50, &mut da);
                }
                _ => {
                    regular_calc( face_count, face*100, 0, &mut da); 
                }
            }

            // print!("face_count {} face_counts {} total {} ", face_count, face_countss, total);
            // println!("");
        }
    }

    da.five_count = face_counts[5];
    da.two_count = face_counts[2];

    if TRACE_ON {
        println!(
            "total {} used {}",
            da.total,
            da.used_dice );
    }

    return da;
}

// This function keeps rolling dize until a zero is rolled
impl Player {
    fn play_hand(&mut self) -> usize {
        let mut run_total = 0;
        let mut num_dice = 6;
        let mut _run_count = 0;

        loop {
            let dice = roll_dice(num_dice);
            print_dice(&dice);
            let mut da = evaluate_dice(&dice);

            if self.player_config.skip_fives > 0 {
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

                if (num_dice - da.used_dice) >= 3 {

                    if da.total >=100 && ( da.five_count == 1 || da.five_count == 4 ) {
                        da.five_count -= 1;
                        da.used_dice -= 1;
                        da.total -= 50;
                    }
                
                    if da.total >= 100 && (da.five_count == 1 || da.five_count == 4 ){
                        da.five_count -= 1;
                        da.used_dice -= 1;
                        da.total -= 50;
                    }
                }
            }


            if self.player_config.skip_twos > 0 {

                if da.total > 200 && da.two_count >= 3 {
                    da.two_count -= 3;
                    da.used_dice -= 3;
                    da.total -= 200;
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
                let soft_limit = self.player_config.limits[num_dice]; 
                if run_total >= soft_limit {
                    break;
                }
            } else {
                if run_total >= 750 && num_dice > 0 {
                    self.on_board = true;
                    break;
                }
            }

            if num_dice == 0 {
                num_dice = 6;
            }
        }

        if TRACE_ON {
            println!("run total {} ", run_total);
        }
        self.score += run_total;
        run_total
    }
}
