struct Player {
    on_board: bool,
    score: bool,
    soft_limit: usize,
}
    
fn build_player( ) -> Player {
    Player {
        on_board: false,
        score: 0,
        soft_limit: 300
    }
}
