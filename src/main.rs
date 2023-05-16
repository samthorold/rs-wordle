use rs_wordle;

fn main() {
    println!("Rust Wordle implementation.");
    let mut node = rs_wordle::Node::from_string(String::from("........."), rs_wordle::Player::O);
    loop {
        let mv = rs_wordle::get_move();
        node = node.make_move(mv);
        print!("{}", node.string());
        if node.is_terminal() {
            break;
        }
        node = node.auto_move();
        print!("{}", node.string());
        if node.is_terminal() {
            break;
        }
    }
}
