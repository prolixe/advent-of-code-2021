use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum CommandType {
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
struct Command {
    command_type: CommandType,
    unit: i32,
}

struct Position {
    x: i32,
    y: i32,
}

struct NewPosition {
    depth: i32,
    hori_pos: i32,
    aim: i32,
}

pub fn day_02() -> std::io::Result<()> {
    //let mut file = File::open("./resources/day02_small.txt")?;
    let mut file = File::open("./resources/day02.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    //println!("contents:\n{}", contents);

    let commands: Vec<&str> = contents.trim().split('\n').collect();

    let commands = commands
        .iter()
        .map(|&command| parse_command(command).unwrap())
        .collect::<Vec<Command>>();

    //println!("commands:\n{:?}", commands);

    let mut pos = Position { x: 0, y: 0 };
    let mut new_pos = NewPosition {
        depth: 0,
        hori_pos: 0,
        aim: 0,
    };

    for command in commands.iter() {
        match command.command_type {
            CommandType::Forward => pos.x += command.unit,
            CommandType::Up => pos.y -= command.unit,
            CommandType::Down => pos.y += command.unit,
        }
        match command.command_type {
            CommandType::Down => new_pos.aim += command.unit,
            CommandType::Up => new_pos.aim -= command.unit,
            CommandType::Forward => {
                new_pos.hori_pos += command.unit;
                new_pos.depth += new_pos.aim * command.unit;
            }
        }
    }

    println!(
        "Final depth depth calculated with simple method {}",
        pos.x * pos.y
    );
    println!(
        "Final depth depth calculated with complex method {}",
        new_pos.depth * new_pos.hori_pos
    );
    return Ok(());
}

fn parse_command(com_str: &str) -> Result<Command, &'static str> {
    let (command, unit) = com_str.split_once(' ').unwrap();

    let command = match command {
        "forward" => CommandType::Forward,
        "up" => CommandType::Up,
        "down" => CommandType::Down,
        _ => return Err("Wrong command"),
    };

    Ok(Command {
        command_type: command,
        unit: unit.parse::<i32>().unwrap(),
    })
}
