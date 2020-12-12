use std::io::{self, BufRead};

#[derive(Debug)]
enum Instruction {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

fn load_instructions() -> Vec<Instruction> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (command, value) = line.split_at(1);
            let value = value.parse().unwrap();
            match command {
                "N" => Instruction::N(value),
                "S" => Instruction::S(value),
                "E" => Instruction::E(value),
                "W" => Instruction::W(value),
                "L" => Instruction::L(value),
                "R" => Instruction::R(value),
                "F" => Instruction::F(value),
                _ => panic!("unexpected command"),
            }
        })
        .collect()
}

// Normalize the direction: return value in interval [0, 360).
fn normalize_direction(direction: i32) -> i32 {
    let mut direction = direction % 360;
    if direction < 0 {
        direction += 360;
    }
    assert!(0 <= direction && direction < 360);
    direction
}

fn solve1(instructions: &[Instruction]) {
    let mut ship = (0, 0);
    let mut dir = 0; // in degrees, 0 == E, 90 == S, 180 == W, ...
    for instruction in instructions {
        match instruction {
            Instruction::N(value) => ship.0 += value,
            Instruction::S(value) => ship.0 -= value,
            Instruction::E(value) => ship.1 += value,
            Instruction::W(value) => ship.1 -= value,
            Instruction::L(value) => dir -= value,
            Instruction::R(value) => dir += value,
            Instruction::F(value) => {
                dir = normalize_direction(dir);
                assert!(dir % 90 == 0, "support only right angled moves");
                match dir / 90 {
                    0 => ship.1 += value, // move east
                    1 => ship.0 -= value, // move south
                    2 => ship.1 -= value, // move west
                    3 => ship.0 += value, // moe north
                    _ => panic!("invalid state"),
                }
            }
        }
    }
    let result = ship.0.abs() + ship.1.abs();
    println!("Result {}", result);
}

fn solve2(instructions: &[Instruction]) {
    fn turn_right(waypoint: &mut (i32, i32), degrees: i32) {
        let dir = normalize_direction(degrees);
        assert!(dir % 90 == 0, "support only right angled turns");
        match dir / 90 {
            0 => (),
            1 => {
                let tmp = waypoint.0;
                waypoint.0 = -waypoint.1;
                waypoint.1 = tmp;
            }
            2 => {
                waypoint.0 *= -1;
                waypoint.1 *= -1;
            }
            3 => {
                let tmp = waypoint.0;
                waypoint.0 = waypoint.1;
                waypoint.1 = -tmp;
            }
            _ => panic!("invalid state"),
        };
    };

    let mut ship = (0, 0);
    let mut waypoint = (1, 10);
    for instruction in instructions {
        match instruction {
            Instruction::N(value) => waypoint.0 += value,
            Instruction::S(value) => waypoint.0 -= value,
            Instruction::E(value) => waypoint.1 += value,
            Instruction::W(value) => waypoint.1 -= value,
            Instruction::L(degrees) => turn_right(&mut waypoint, -degrees),
            Instruction::R(degrees) => turn_right(&mut waypoint, *degrees),
            Instruction::F(value) => {
                ship.0 += value * waypoint.0;
                ship.1 += value * waypoint.1;
            }
        }
    }
    let result = ship.0.abs() + ship.1.abs();
    println!("Result {}", result);
}

fn main() {
    let instructions = load_instructions();
    solve1(&instructions);
    solve2(&instructions);
}
