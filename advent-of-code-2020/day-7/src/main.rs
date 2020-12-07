use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

struct Graph {
    index: HashMap<String, usize>,
    edges: Vec<Vec<(usize, u32)>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            index: HashMap::new(),
            edges: Vec::new(),
        }
    }

    fn register(&mut self, name: &str) -> usize {
        let free_idx = self.index.len();
        assert_eq!(free_idx, self.edges.len());
        let idx = *self.index.entry(name.to_string()).or_insert(free_idx);
        if idx == free_idx {
            self.edges.push(Vec::new());
        }
        idx
    }

    fn lookup(&self, name: &str) -> usize {
        *self.index.get(name).unwrap()
    }

    fn reverse(&self) -> Self {
        let mut edges = vec![Vec::new(); self.edges.len()];
        for (idx0, nodes) in self.edges.iter().enumerate() {
            for (idx1, value) in nodes.iter() {
                edges[*idx1].push((idx0, *value));
            }
        }

        Graph {
            index: self.index.clone(),
            edges,
        }
    }
}

fn load_graph() -> Graph {
    let mut graph = Graph::new();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        // A line is in format: "$color0 bags contain $number1 $color1 bags, ...."
        // or "$color0 bags contain no other bags."
        let parts: Vec<&str> = line
            .trim_end_matches('.')
            // TODO: use split_once()
            .splitn(2, " bags contain ")
            .collect();

        let idx0 = graph.register(parts[0]); // $color0
        match parts[1] {
            "no other bags" => (),
            list => {
                for entry in list.split(", ") {
                    let num_entry: Vec<&str> = entry
                        // remove " bag[s]" part
                        .trim_end_matches('s')
                        .trim_end_matches(" bag")
                        // Split into "$number $color". TODO: use split_once()
                        .splitn(2, ' ')
                        .collect();
                    let idx1 = graph.register(num_entry[1]);
                    //graph.edges[idx0].push((idx, 42)); // reverse graph!
                    graph.edges[idx0].push((idx1, num_entry[0].parse().unwrap()));
                }
            }
        }
    }
    graph
}

fn solve1(graph: &Graph) {
    let graph = graph.reverse();
    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    queue.push(graph.lookup("shiny gold"));
    while !queue.is_empty() {
        let next = queue.pop().unwrap();
        if visited.insert(next) {
            for (node, _) in graph.edges[next].iter() {
                queue.push(*node);
            }
        }
    }
    println!("Result: {}", visited.len() - 1);
}

fn count(graph: &Graph, cache: &mut HashMap<usize, u32>, node: usize) -> u32 {
    match cache.get(&node) {
        Some(total) => *total,
        None => {
            let mut total: u32 = 0;
            for (node1, value) in graph.edges[node].iter() {
                total += value;
                total += value * count(graph, cache, *node1);
            }
            cache.insert(node, total);
            total
        }
    }
}

fn solve2(graph: &Graph) {
    let result = count(graph, &mut HashMap::new(), graph.lookup("shiny gold"));
    println!("Result: {}", result);
}

fn main() {
    let graph = load_graph();
    solve1(&graph);
    solve2(&graph);
}
