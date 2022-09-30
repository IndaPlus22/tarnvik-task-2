use std::io;
use std::io::prelude::*;

fn main() {
    let input = io::stdin();

    let lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>();

    for line in lines.iter(){
        let numbers = line
        .split(" ")                                         
        .map(|component| component.parse::<i64>().unwrap()) 
        .collect::<Vec<i64>>();

        let difference = (numbers[0]-numbers[1]).abs();

        println!("{}", difference);
    } 
}