use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn load_state() -> Vec<(i32, i32)> {
    io::stdin()
        .lock()
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            let line = line.unwrap();
            line.chars()
                .enumerate()
                .filter(|entry| entry.1 == '#')
                .map(move |entry| (i as i32, entry.0 as i32))
                .collect::<Vec<(i32, i32)>>()
        })
        .collect()
}

fn boot_state<T>(
    state: &[(i32, i32)],
    deltas: &[T],
    to_point: &mut dyn FnMut(&(i32, i32)) -> T,
    add: &mut dyn FnMut(&T, &T) -> T,
) -> usize
where
    T: std::cmp::Eq + std::hash::Hash + Copy,
{
    let mut state: HashSet<T> = state.iter().map(to_point).collect();
    for _iteration in 0..6 {
        let mut neighbours_counts: HashMap<T, i32> = HashMap::new();
        // First pass: count active neighbours
        for cube in state.iter() {
            for delta in deltas {
                *neighbours_counts.entry(add(cube, delta)).or_insert(0) += 1;
            }
        }
        // Second pass: determine which entries are active for the next iteration
        state = neighbours_counts
            .into_iter()
            .filter(|(cube, count)| *count == 3 || (*count == 2 && state.contains(cube)))
            .map(|entry| entry.0)
            .collect();
    }
    state.len()
}

fn solve1(state: &[(i32, i32)]) {
    let deltas: Vec<(i32, i32, i32)> = (-1..=1)
        .flat_map(|d| (-1..=1).map(move |d0| (d0, d)))
        .flat_map(|d| (-1..=1).map(move |d0| (d0, d.0, d.1)))
        .filter(|&delta| delta != (0, 0, 0))
        .collect();

    let result = boot_state(
        state,
        &deltas,
        &mut |point| (point.0, point.1, 0),
        &mut |point, delta| (point.0 + delta.0, point.1 + delta.1, point.2 + delta.2),
    );
    println!("Result {}", result);
}

fn solve2(state: &[(i32, i32)]) {
    let deltas: Vec<[i32; 4]> = (-1..=1)
        .flat_map(|d| (-1..=1).map(move |d0| (d0, d)))
        .flat_map(|d| (-1..=1).map(move |d0| (d0, d.0, d.1)))
        .flat_map(|d| (-1..=1).map(move |d0| [d0, d.0, d.1, d.2]))
        .filter(|&delta| delta != [0, 0, 0, 0])
        .collect();

    let result = boot_state(
        state,
        &deltas,
        &mut |point| [point.0, point.1, 0, 0],
        &mut |point, delta| {
            let mut point1 = point.clone();
            for i in 0..point1.len() {
                point1[i] += delta[i];
            }
            point1
        },
    );
    println!("Result {}", result);
}

fn main() {
    let state = load_state();
    solve1(&state);
    solve2(&state);
}
