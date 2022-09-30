use std::io;
use std::io::prelude::*;

fn main() {
    let input = io::stdin();

    let mut lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap());
    
    let numbers = lines
    .next().unwrap()
    .split(" ")                                         
    .map(|component| component.parse::<i64>().unwrap()) 
    .collect::<Vec<i64>>();   

    let rows = numbers[0];
    let colums = numbers[1];

    for r in 1..=rows {
        for c in 1..= colums{
            let row_value = (r-rows).abs();
            let colum_value = (c-colums).abs();
            print!("{}", smallest_value(r, c, colum_value, row_value));                    
        }
        println!();
    }
}

fn smallest_value (r: i64, c: i64, colum_value: i64, row_value: i64) -> String{
    if row_value >= 9 && colum_value >= 9 && c > 9 && r > 9 {
        return ".".to_string();
    } else if r <= c && r <= colum_value && r <= row_value {
        return r.to_string();
    } else if c < r && c <= colum_value && c <= row_value {
        return c.to_string();
    } else if colum_value < r && colum_value < c && colum_value < row_value {
        return (colum_value+1).to_string();
    } else {
        return (row_value+1).to_string();
    }
}