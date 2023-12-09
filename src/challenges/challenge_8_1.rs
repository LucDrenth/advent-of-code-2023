use std::collections::HashMap;

use crate::utils::file_utils;

const DIRECTION_LEFT: usize = 0;
const DIRECTION_RIGHT: usize = 1;
const DIRECTION_UNDEFINED: usize = 2;

pub fn execute() {
    let input = file_utils::get_all_lines("assets/input_8.txt");
    let directions = get_directions(&input[0]);
    let graph = create_graph(&input[2..].to_vec());
    let steps = graph.count_steps("AAA", "ZZZ", &directions);

    println!("{steps}");
}

fn get_directions(line: &String) -> Vec<usize> {
    let mut directions: Vec<usize> = vec![];

    for direction in line.chars() {
        if direction == 'L' {
            directions.push(DIRECTION_LEFT);
        } else if direction == 'R' {
            directions.push(DIRECTION_RIGHT);
        } else {
            panic!("unhandled direction");
        }
    }

    directions
}

struct Graph {
    lookup: HashMap<String, usize>, // Node ID -> node index
    nodes: Vec<Node>,
}

struct Node {
    directions: [usize; 2],
    id: String,
}

fn create_graph(lines: &Vec<String>) -> Graph {
    let mut graph = Graph{
        lookup: HashMap::default(),
        nodes: vec![],
    };

    for line in lines {
        let node_id = line[0..=2].to_string();
        let node_left_id = line[7..=9].to_string();
        let node_right_id = line[12..=14].to_string();

        // Ensure the left/right nodes are in the graph.
        let node_left_index = match graph.lookup.get(&node_left_id) {
            Some(&i) => i,
            None => {
                graph.nodes.push(Node { directions: [DIRECTION_UNDEFINED; 2], id: node_left_id.clone() });
                let node_index = graph.nodes.len() - 1;
                graph.lookup.insert(node_left_id, node_index);

                node_index
            },
        };
        let node_right_index = match graph.lookup.get(&node_right_id) {
            Some(&i) => i,
            None => {
                graph.nodes.push(Node { directions: [DIRECTION_UNDEFINED; 2], id: node_right_id.clone() });
                let node_index = graph.nodes.len() - 1;
                graph.lookup.insert(node_right_id, node_index);

                node_index
            },
        };

        match graph.lookup.get(&node_id) {
            Some(i) => {
                graph.nodes[*i].directions[DIRECTION_LEFT] = node_left_index;
                graph.nodes[*i].directions[DIRECTION_RIGHT ] = node_right_index;
            },
            None => {
                graph.nodes.push(Node { directions: [node_left_index, node_right_index], id: node_id.clone() });
                let node_index = graph.nodes.len() - 1;
                graph.lookup.insert(node_id, node_index);
            },
        }
    }

    graph
}

impl Graph {
    fn count_steps(&self, from:  impl Into<String>, to: impl Into<String>, directions: &Vec<usize>) -> usize {
        let mut current_index = *(self.lookup.get(&from.into()).unwrap());
        let mut steps_taken = 0;

        let destination = to.into();

        loop {
            for direction in directions {
                if self.nodes[current_index].id == destination {
                    return steps_taken;
                }

                current_index = self.nodes[current_index].directions[*direction];
                steps_taken += 1;
            }
        }
    }
}
