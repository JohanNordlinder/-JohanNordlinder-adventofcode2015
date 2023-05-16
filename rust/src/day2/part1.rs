#[derive(Debug)]
struct Gift {
    slack: i32,
    surface_area: i32,
}

pub fn part1(lines: &Vec<String>) {
    let mut gifts: Vec<Gift> = Vec::new();

    for line in lines {
        let parts = line
            .split("x")
            .map(|z| z
                .parse()
                .unwrap()
            ).collect::<Vec<i32>>();
        let length = parts[0];
        let width = parts[1];
        let height = parts[2];

        let mut sides: Vec<i32> = Vec::new();
        sides.push(length * width);
        sides.push(width * height);
        sides.push(height * length);

        gifts.push(Gift {
            slack: *sides.iter().min().unwrap(),
            surface_area: sides.iter().map(|x| x * 2).sum(),
        });
    }
    
    //println!("{:#?}", gifts);

    let total: i32 = gifts.iter().map(|x| x.surface_area + x.slack).sum();
    println!("Answer part 1: {}", total);
}
