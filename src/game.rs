pub struct Dice10000
{
    player: Vec<Player>
}

pub fn build_game() -> Dice10000{
    let mut p: Vec<Player> = Vec::new();
    p.push(build_player());
    p.push(build_player());

    let d = Dice10000{ player: p};

    d
}

pub struct Player {
    on_board: bool,
    score: usize,
    soft_limit: usize,
}
    
pub fn build_player( ) -> Player {
    Player {
        on_board: false,
        score: 0,
        soft_limit: 300
    }
}

