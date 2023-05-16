#[derive(Debug)]
struct Gift {
    ribbon: i32,
    bow: i32,
}

pub fn part2(lines: &Vec<String>) {
    let mut gifts: Vec<Gift> = Vec::new();

    for line in lines.iter() {
        let mut sides = line
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
    println!("Answer part 2: {}", total);
}
