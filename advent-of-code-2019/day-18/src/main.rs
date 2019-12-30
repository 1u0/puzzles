use std::collections::{HashMap, HashSet, VecDeque};
use std::io;
use std::io::BufRead;

#[derive(Hash, PartialEq, Eq, Debug, PartialOrd, Ord, Clone)]
enum Node {
    Bot(usize),
    Key(u32),
    Gate(u32),
}

impl Node {
    fn from_char(ch: char) -> Option<Self> {
        match ch {
            '@' => Some(Node::Bot(0)),
            '$' => Some(Node::Bot(1)),
            '%' => Some(Node::Bot(2)),
            '^' => Some(Node::Bot(3)),
            'a'..='z' => Some(Node::Key(1 << (ch as u32 - 'a' as u32))),
            'A'..='Z' => Some(Node::Gate(1 << (ch as u32 - 'A' as u32))),
            _ => None,
        }
    }
}

type Maze = Vec<Vec<char>>;

fn read_maze() -> Maze {
    io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().trim().chars().collect())
        .collect()
}

// Returns only directly reachable (adjacent) nodes from given point and corresponding distance to them.
fn traverse(maze: &Maze, coord: (usize, usize)) -> Vec<(Node, usize)> {
    let n = maze.len();
    let m = maze[0].len();
    let mut visited = HashSet::new();
    let mut front = HashSet::new();
    visited.insert(coord);
    front.insert(coord);

    let mut it: usize = 0;
    let mut result = Vec::new();
    while !front.is_empty() {
        it += 1;
        let old_front = front;
        front = HashSet::new();
        for (i, j) in old_front.iter() {
            let i = *i;
            let j = *j;
            // The maze has a fence near the border and (i, j) is an internal point.
            assert!(0 < i && i < n - 1);
            assert!(0 < j && j < m - 1);
            for (i, j) in &[(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                let ch = maze[*i][*j];
                if ch == '#' {
                    continue;
                }
                let pos = (*i, *j);
                if !visited.insert(pos) {
                    continue;
                }
                if let Some(node) = Node::from_char(ch) {
                    result.push((node, it));
                } else {
                    front.insert(pos);
                }
            }
        }
    }
    result
}

type Graph = HashMap<Node, Vec<(Node, usize)>>;

// Simplify the maze to the graph of adjacent nodes
fn get_graph(maze: &Maze) -> Graph {
    let mut graph = HashMap::new();
    for (i, row) in maze.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if let Some(node) = Node::from_char(*ch) {
                graph.insert(node, traverse(maze, (i, j)));
            }
        }
    }
    graph
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct PathData {
    len: usize,
    keys: u32,  // used as bitset
    gates: u32, // used as bitset
}
impl PathData {
    fn new(start_node: &Node) -> Self {
        let len = 0;
        let mut keys = 0;
        let mut gates = 0;
        match start_node {
            Node::Key(ind) => {
                keys |= ind;
            }
            Node::Gate(ind) => {
                gates |= ind;
            }
            _ => {}
        }
        PathData { len, keys, gates }
    }

    fn with_node(&self, next_node: &Node, len: usize) -> Self {
        let len = self.len + len;
        let mut keys = self.keys;
        let mut gates = self.gates;
        match next_node {
            Node::Key(ind) => {
                keys |= ind;
            }
            Node::Gate(ind) => {
                gates |= ind;
            }
            _ => {}
        }
        PathData { len, keys, gates }
    }
}
fn traverse_all_paths(node: &Node, graph: &Graph) -> HashMap<Node, PathData> {
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    visited.insert(node);
    let state = (node, PathData::new(node));
    queue.push_back(state);
    while !queue.is_empty() {
        let (node, path_data) = queue.pop_front().unwrap();
        for (next_node, len) in graph.get(node).unwrap().iter() {
            if visited.insert(next_node) {
                let next_path = path_data.with_node(next_node, *len);
                queue.push_back((next_node, next_path.clone()));
                distances.insert((*next_node).clone(), next_path);
            }
        }
    }
    distances
}

fn get_full_graph(graph: &Graph) -> HashMap<Node, HashMap<Node, PathData>> {
    let mut full_graph = HashMap::new();
    for node in graph.keys().cloned() {
        match node {
            Node::Bot(_) | Node::Key(_) => {
                let distances = traverse_all_paths(&node, graph);
                full_graph.insert(node, distances);
            }
            _ => {}
        }
    }
    full_graph
}

fn get_keys(nodes: &HashMap<Node, PathData>) -> u32 {
    nodes
        .keys()
        .map(|node| match node {
            Node::Key(ind) => *ind,
            _ => 0,
        })
        .sum()
}

fn find_shortest_distance(
    distances: &HashMap<Node, HashMap<Node, PathData>>,
    cache: &mut HashMap<(Node, u32), usize>,
    node: &Node,
    keys: u32,
    all_keys: u32,
) -> usize {
    if keys == all_keys {
        return 0;
    }
    match cache.get(&(node.clone(), keys)) {
        Some(distance) => *distance,
        None => {
            let distance = distances
                .get(node)
                .unwrap()
                .iter()
                .filter(|(next_node, path)| match next_node {
                    Node::Key(idx) => (keys & idx == 0) && (path.gates & keys == path.gates),
                    _ => false,
                })
                .map(|(next_node, path)| {
                    path.len
                        + find_shortest_distance(
                            distances,
                            cache,
                            next_node,
                            path.keys | keys,
                            all_keys,
                        )
                })
                .min()
                .unwrap();
            cache.insert((node.clone(), keys), distance);
            distance
        }
    }
}

fn solve1(maze: &Maze) {
    let all_distances = get_full_graph(&get_graph(maze));
    let all_keys: u32 = get_keys(&all_distances[&Node::Bot(0)]);
    let res = find_shortest_distance(
        &all_distances,
        &mut HashMap::new(),
        &Node::Bot(0),
        0,
        all_keys,
    );
    println!("Result for task 1: {}", res);
}

fn find_bot(maze: &Maze) -> (usize, usize) {
    for (i, row) in maze.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if let Some(Node::Bot(_)) = Node::from_char(*ch) {
                return (i, j);
            }
        }
    }
    unreachable!();
}

fn change_maze(maze: &mut Maze) {
    let (i, j) = find_bot(&maze);
    maze[i - 1][j - 1] = '@';
    maze[i - 1][j] = '#';
    maze[i - 1][j + 1] = '$';
    maze[i][j - 1] = '#';
    maze[i][j] = '#';
    maze[i][j + 1] = '#';
    maze[i + 1][j - 1] = '%';
    maze[i + 1][j] = '#';
    maze[i + 1][j + 1] = '^';
}

fn solve2(maze: &Maze) {
    let all_distances = get_full_graph(&get_graph(maze));
    let mut keys_per_bot = vec![0; 4];
    for i in 0..4 {
        keys_per_bot[i] = get_keys(&all_distances[&Node::Bot(i)]);
        for j in 0..i {
            assert!(keys_per_bot[i] & keys_per_bot[j] == 0);
        }
    }
    let all_keys = keys_per_bot.iter().sum();
    let result: usize = (0..4)
        .map(|i| {
            find_shortest_distance(
                &all_distances,
                &mut HashMap::new(),
                &Node::Bot(i),
                all_keys ^ keys_per_bot[i],
                all_keys,
            )
        })
        .sum();
    println!("Result for task 2: {}", result);
}

fn main() {
    let mut maze = read_maze();
    solve1(&maze);

    change_maze(&mut maze);
    solve2(&maze);
}
