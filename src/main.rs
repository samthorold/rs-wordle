use std::io;

#[derive(Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Clone, Copy, Debug)]
struct Move {
    row: usize,
    col: usize,
}

struct Node {
    player: Player,
    state: Vec<Vec<Option<Player>>>,
    moves: Vec<Move>,
    is_max: bool,
}

impl Clone for Node {
    fn clone(&self) -> Node {
        Node {
            moves: self.moves.clone(),
            state: self.state.clone(),
            ..*self
        }
    }
}

impl Node {
    pub fn from_string(string: String, player: Player) -> Node {
        let mut state = Vec::new();
        for r in 0..3 {
            let mut this_row = Vec::new();
            for c in 0..3 {
                let ch = string.chars().nth(r * 3 + c).unwrap();
                match String::from(ch).as_str() {
                    "." => this_row.push(Option::None),
                    "x" => this_row.push(Some(Player::X)),
                    "o" => this_row.push(Some(Player::O)),
                    _ => panic!("Only '.xo' accepted."),
                }
            }
            state.push(this_row);
        }
        Node {
            player,
            state,
            moves: Vec::new(),
            is_max: match player {
                Player::X => true,
                Player::O => false,
            },
        }
    }

    pub fn string(&self) -> String {
        let mut s = String::from("");
        for row in self.state.clone() {
            for val in row.clone() {
                match val {
                    Some(Player::X) => s.push_str("x"),
                    Some(Player::O) => s.push_str("o"),
                    _ => s.push_str("."),
                }
            }
            s.push_str("\n");
        }
        s.push_str("\n");
        s
    }
    pub fn is_terminal(&self) -> bool {
        if self.score() != 0 {
            return true;
        }

        if self
            .state
            .iter()
            .all(|row| row.iter().all(|val| val.is_some()))
        {
            return true;
        }

        false
    }

    pub fn is_maximising(&self) -> bool {
        return self.is_max;
    }

    pub fn children(&self) -> Vec<Node> {
        let mut chdn = Vec::new();
        if self.is_terminal() {
            return chdn;
        }
        for (rix, row) in self.state.iter().enumerate() {
            for (cix, val) in row.iter().enumerate() {
                if !val.is_some() {
                    chdn.push(self.make_move(Move { row: rix, col: cix }));
                }
            }
        }
        chdn
    }

    pub fn score(&self) -> i32 {
        for p in [Player::X, Player::O] {
            let sign = match p {
                Player::X => 1,
                Player::O => -1,
            };
            if self
                .state
                .iter()
                .any(|row| row.iter().all(|val| val.is_some() && val.unwrap() == p))
            {
                return 10 * sign;
            }

            for c in 0..3 {
                if (self.state[0][c].is_some() && self.state[0][c].unwrap() == p)
                    && (self.state[1][c].is_some() && self.state[1][c].unwrap() == p)
                    && (self.state[2][c].is_some() && self.state[2][c].unwrap() == p)
                {
                    return 10 * sign;
                }
            }

            if (self.state[0][0].is_some() && self.state[0][0].unwrap() == p)
                && (self.state[1][1].is_some() && self.state[1][1].unwrap() == p)
                && (self.state[2][2].is_some() && self.state[2][2].unwrap() == p)
            {
                return 10 * sign;
            }

            if (self.state[0][2].is_some() && self.state[0][2].unwrap() == p)
                && (self.state[1][1].is_some() && self.state[1][1].unwrap() == p)
                && (self.state[2][0].is_some() && self.state[2][0].unwrap() == p)
            {
                return 10 * sign;
            }
        }
        0
    }

    pub fn make_move(&self, m: Move) -> Node {
        let rix = m.row;
        let cix = m.col;
        let maybe_cell = self.state[rix][cix];
        match maybe_cell {
            None => {
                let mut next_state = self.state.clone();
                next_state[rix][cix] = Some(self.player);
                let mut next_moves = self.moves.clone();
                next_moves.push(m);
                return Node {
                    state: next_state,
                    player: match self.player {
                        Player::X => Player::O,
                        Player::O => Player::X,
                    },
                    moves: next_moves,
                    is_max: !self.is_max,
                };
            }
            Some(_) => panic!("cell occupied {}{}", rix, cix),
        }
    }
}

fn minimax(node: Node) -> Node {
    if node.is_terminal() {
        return node;
    }
    let mut best_score = match node.is_maximising() {
        true => -100,
        false => 100,
    };

    if node.children().len() == 0 {
        panic!("Not terminal but no children.")
    }

    let mut best_node = node.clone();

    for child in node.children() {
        let variation = minimax(child.clone());
        if node.is_maximising() {
            if variation.score() > best_score {
                best_node = variation;
                best_score = best_node.score();
            }
        } else {
            if variation.score() < best_score {
                best_node = variation;
                best_score = best_node.score();
            }
        }
    }
    best_node
}

fn get_move() -> Move {
    let mut mv = String::new();
    io::stdin().read_line(&mut mv).expect("Failed to read line");
    let cmps = mv.trim().split_at(1);

    Move {
        row: cmps.0.parse().expect("Row not a number."),
        col: cmps.1.parse().expect("Col not a number."),
    }
}

fn main() {
    println!("Rust Wordle implementation.");
    let mut node = Node::from_string(String::from("........."), Player::O);
    loop {
        let mv = get_move();
        node = node.make_move(mv);
        print!("{}", node.string());
        if node.is_terminal() {
            break;
        }
        let variation = minimax(node.clone());
        node = node.make_move(variation.moves[node.moves.len()]);
        print!("{}", node.string());
        if node.is_terminal() {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_string() {
        let _ = Node::from_string(String::from("ox.o....."), Player::X);
    }

    #[test]
    fn to_string() {
        let node = Node::from_string(String::from("ox.o....."), Player::X);
        let _ = node.string();
    }

    #[test]
    fn to_score_0() {
        let node = Node::from_string(String::from("ox.o....."), Player::X);
        let score = node.score();
        assert_eq!(0, score);
    }

    #[test]
    fn to_score_10() {
        let node = Node::from_string(String::from("xxxoo...."), Player::X);
        let score = node.score();
        assert_eq!(10, score);
    }

    #[test]
    fn to_score_minus_10() {
        let node = Node::from_string(String::from("ox.ox.o.."), Player::X);
        let score = node.score();
        assert_eq!(-10, score);
        let node = Node::from_string(String::from(".xo.xo..o"), Player::X);
        let score = node.score();
        assert_eq!(-10, score);
    }

    #[test]
    fn children() {
        let node = Node::from_string(String::from("ox.o....."), Player::X);
        let children = node.children();
        assert_eq!(6, children.len());
    }

    #[test]
    fn no_children() {
        let node = Node::from_string(String::from("ox.ox.o.."), Player::X);
        let children = node.children();
        assert_eq!(0, children.len());
    }

    #[test]
    fn make_move() {
        let node = Node::from_string(String::from("ox.o....."), Player::X);
        let child = node.make_move(Move { row: 0, col: 2 });
        assert_eq!("oxx\no..\n...\n\n", child.string());
    }
}
