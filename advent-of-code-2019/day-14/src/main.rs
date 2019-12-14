use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;

fn parse(token: &str) -> (&str, i64) {
    let mut parts = token.trim().split_whitespace();
    let count = parts.next().unwrap().parse().unwrap();
    let component = parts.next().unwrap();
    (component, count)
}

fn load_input() -> Reactions {
    let mut reactions = Reactions::new();

    io::stdin().lock().lines().for_each(|line| {
        let line = line.unwrap();
        let mut parts = line.split("=>");
        let reaction = parts
            .next()
            .unwrap()
            .split(',')
            .map(|token| parse(token))
            .collect::<Vec<_>>();
        let component = parts.next().map(|token| parse(token)).unwrap();
        reactions.add_reaction(&component, &reaction);
    });

    reactions
}

struct Reactions {
    index: HashMap<String, usize>,
    reactions: Vec<(i64, HashMap<usize, i64>)>,
}

impl Reactions {
    fn new() -> Self {
        Reactions {
            index: HashMap::new(),
            reactions: Vec::new(),
        }
    }

    fn add_reaction(&mut self, component: &(&str, i64), reaction: &[(&str, i64)]) {
        let reaction: HashMap<usize, i64> = reaction
            .iter()
            .map(|(name, count)| (self.register(name), *count))
            .collect();
        let idx = self.register(&component.0);
        self.reactions[idx] = (component.1, reaction);
    }

    fn register(&mut self, name: &str) -> usize {
        let free_idx = self.index.len();
        assert_eq!(free_idx, self.reactions.len());
        let idx = *self.index.entry(name.to_string()).or_insert(free_idx);
        if idx == free_idx {
            self.reactions.push((0, HashMap::new()));
        }
        idx
    }

    fn topological_sort(&self) -> Vec<usize> {
        let mut incoming_nodes = HashMap::new();
        for (src, (_, reaction)) in self.reactions.iter().enumerate() {
            for dst in reaction.keys() {
                incoming_nodes
                    .entry(dst)
                    .or_insert_with(HashSet::new)
                    .insert(src);
            }
        }

        // Start with all nodes that don't have incoming edges.
        let mut current = (0..self.reactions.len())
            .filter(|node| !incoming_nodes.contains_key(&node))
            .collect::<Vec<_>>();
        let mut sorted = Vec::new();
        while !current.is_empty() {
            let src = current.pop().unwrap();
            sorted.push(src);

            let reaction = &self.reactions[src].1;
            for dst in reaction.keys() {
                let set = incoming_nodes.get_mut(&dst).unwrap();
                set.remove(&src);
                if set.is_empty() {
                    current.push(*dst);
                }
            }
        }
        sorted
    }

    fn lookup(&self, name: &str) -> usize {
        *self.index.get(name).unwrap()
    }

    fn substitute(&self, component: usize, resources: &mut HashMap<usize, i64>) {
        let (count, reaction) = &self.reactions[component];
        if reaction.is_empty() {
            return; // component is a basic resource
        }
        match resources.remove(&component) {
            None => {}
            Some(min_amount) => {
                let multiplier = (min_amount + count - 1) / count;
                for (resource_component, amount) in reaction {
                    *resources.entry(*resource_component).or_insert(0) += amount * multiplier;
                }
            }
        }
    }

    fn count_min_basic_resources(&self, component: usize, min_amount: i64) -> HashMap<usize, i64> {
        let (count, reaction) = &self.reactions[component];
        let multiplier = (min_amount + count - 1) / count;
        let mut resources: HashMap<usize, i64> = reaction
            .iter()
            .map(|(resource_component, amount)| (*resource_component, amount * multiplier))
            .collect();
        for component in self.topological_sort() {
            self.substitute(component, &mut resources);
        }
        resources
    }
}

fn main() {
    let reactions = load_input();
    let fuel = reactions.lookup(&"FUEL");
    let ore = reactions.lookup(&"ORE");

    {
        let result = reactions.count_min_basic_resources(fuel, 1);
        assert_eq!(1, result.len());
        println!("Result for task 1: {}", result.get(&ore).unwrap());
    }

    {
        let max_ore = 1000000000000;
        let mut count0 = 1;
        let mut count1 = max_ore;
        while count0 < count1 {
            let count = (count0 + count1) / 2;
            if count == count0 {
                break;
            }
            let result = reactions.count_min_basic_resources(fuel, count);
            assert_eq!(1, result.len());
            if *result.get(&ore).unwrap() > max_ore {
                count1 = count;
            } else {
                count0 = count;
            }
        }
        println!("Result for task 2: {}", count0);
    }
}
