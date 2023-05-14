use std::collections::HashMap;

struct TreeEntry {
    children: Vec<String>,
    max: i32,
    min: i32,
}

struct Tree {
    contents: HashMap<String, TreeEntry>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            contents: HashMap::new(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Clone)]
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
    pub fn from_string(string: String) -> Node {
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
            player: Player::O,
            state,
            moves: Vec::new(),
            is_max: true,
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
        false
    }

    pub fn is_maximising(&self) -> bool {
        return self.is_max;
    }

    pub fn children(&self) -> Vec<Node> {
        vec![Node {
            player: match self.player {
                Player::X => Player::O,
                Player::O => Player::X,
            },
            state: self.state.clone(),
            is_max: !self.is_max,
            moves: self.moves.to_vec(),
        }]
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
            if self
                .state
                .iter()
                .any(|row| row.iter().all(|val| val.is_some() && val.unwrap() == p))
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

fn minimax(node: &Node) -> &Node {
    if node.is_terminal() {
        return node;
    }
    let mut best_score = match node.is_maximising() {
        true => -100,
        false => 100,
    };

    let mut best_node = node;

    for child in node.children() {
        let variation = minimax(&child);
        if node.is_maximising() {
            if variation.score() > best_score {
                best_node = node;
                best_score = best_node.score();
            }
        } else {
            if variation.score() < best_score {
                best_node = node;
                best_score = best_node.score();
            }
        }
    }
    best_node
}

fn main() {
    println!("Rust Wordle implementation.");
    let node = Node::from_string(String::from("ox.o....."));
    print!("{}", node.string());
}
