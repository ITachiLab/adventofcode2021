use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::VecDeque;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = File::open("resources/day_01").expect("Cannot find input file");
    let reader = BufReader::new(file);

    let mut previous = i32::MAX;
    let mut increased = 0;

    for line in reader.lines() {
        let next = line.unwrap().parse().unwrap();

        if next > previous {
            increased += 1;
        }

        previous = next;
    }

    println!("{}", increased);
}

fn part_2() {
    let file = File::open("resources/day_01").expect("Cannot find input file");
    let reader = BufReader::new(file);

    let mut window:VecDeque<i32> = VecDeque::with_capacity(3);
    let mut read = 0;
    let mut previous = i32::MAX;
    let mut increased = 0;

    for line in reader.lines() {
        let number = line.unwrap().parse().unwrap();
        window.push_back(number);
        read += 1;

        if read >= 3 {
            let next = window_sum(&window);

            if next > previous {
                increased += 1;
            }

            previous = next;
            window.pop_front();
        }
    }

    println!("{}", increased);
}

fn window_sum(window: &VecDeque<i32>) -> i32 {
    let mut sum = 0;
    for n in window {
        sum += n;
    }

    sum
}