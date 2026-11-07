use scorekeeper::Scoreboard;

fn main() {
    let mut board = Scoreboard::new();
    board.add_points("Ana", 30);
    board.add_points("Sam", 45);
    board.add_points("Ana", 20);

    for (place, (name, score)) in board.ranking().iter().enumerate() {
        println!("{}. {name}: {score}", place + 1);
    }
}
