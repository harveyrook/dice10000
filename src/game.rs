mod player;

struct Dice10000
{
    player: Vec<player::Player>
}

fn build_game() -> Dice10000{
    let mut d = Dice10000{};
    d.player.push(player::build_player());
    d.player.push(player::build_player());

    d;
}
