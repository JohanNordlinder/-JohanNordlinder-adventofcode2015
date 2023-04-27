use std::fs::File;
use std::io::{BufReader, Lines};

#[derive(Debug)]
struct Gift {
    ribbon: i32,
    bow: i32,
}

pub fn part2(lines: Lines<BufReader<File>>) {
    let mut gifts: Vec<Gift> = Vec::new();

    for line in lines {
        let raw_gift = line.unwrap();
        let mut sides = raw_gift
            .split("x")
            .map(|z| z
                .parse()
                .unwrap()
            ).collect::<Vec<i32>>();
        sides.sort();
        gifts.push(Gift {
            ribbon: sides.iter().take(2).map(|x| x * 2).sum(),
            bow: sides[0] * sides[1] * sides[2],
        });
    }
    
    //println!("{:#?}", gifts);

    let total: i32 = gifts.iter().map(|x| x.ribbon + x.bow).sum();
    println!("Answer: {}", total);
}
