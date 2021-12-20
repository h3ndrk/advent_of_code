fn main() {
    // let algorithm = parse("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#");
    let algorithm = parse("#####.#.##.####.#....#.###.##.###.#..#.#...##....#.#.##..#...#..##.##.####.#....##.#.#......###..#.#.#...##.#...###...#.#...##....####....#...#.####..#.#.####....####..#..#..#....###...#..#.###.##..##..####....####.###..#.#.##.#.###.##....####.#####.##.#.#...##.######....##...#####.##..###.####.#.##..##...#.#..#.#.#.....#..#..##..##..######..#..#..##..#.##.#..###.#....#.#.#.#.#.#..#.##.#.#....###.#.#..#....#.##..##.##...#..#####.###.......###.###.#..#.##.######.#######.##..##.##..#.##......#####.#.....#..#.");
    // let image = extend(vec![
    //     parse("#..#."),
    //     parse("#...."),
    //     parse("##..#"),
    //     parse("..#.."),
    //     parse("..###"),
    // ]);
    let mut image = vec![
        parse("......................................................................................................"),
        parse("..##..##...#...#...#.##.###.##.......#....#.#..#...###.##.###..#.##.###...#..#..######...#####.#....#."),
        parse(".#.......###..##.#.#.....###.#..#..#.#..#...#..##.#.#.#.#.#.#.#.#..####.#.#.#.#....####.##.#.#.#.###.."),
        parse("....###..#..##.#.##..#.#..###...#..#.###.#.###....##...####..#.#.#.###.#.#..##..#.#...#########.##.#.."),
        parse("..#...##..#.####..#.#.#..##.#.#.##.##..##.#..#.#..##.##...#..#.#..#..#....##....###.##.###..#.#..####."),
        parse("...###..#.#.#.#.#..#...##......#..###.#.#..##....#.#####....####..##..####.#.####..####...####.#..#..."),
        parse("..##.##.#.#..#.#.##.######.##..###....#####.#...#..#...##.######..#.##.###...##.#.#.#.#.#.......#.#.#."),
        parse("........#..#..#.##.###....#.#..##..#.#..#...#..##.#..#.#.#...######.####..###...###.#....#...#..#.###."),
        parse("..####.####...........#.#....#.#.#...#.###.##.##...#.####...##..###..###..###.##...#.##.##.#.####..#.."),
        parse(".######.#....####.#.##..#.##.#.######...##.#...##.#.###..##....#.......##.#.#.#.####.#.#.....#.#...##."),
        parse(".#.##..##.....#.#.###..##..##..###....##..##.#.#..#...####.#.#.###.##.##.#.####.#.###.#..#...#.##..##."),
        parse(".##.###.###.#....###.####.##..##..#.#.#.#.#...#....###..#..#..##...#.##..##..#.#.#######...###...##.#."),
        parse(".##.##....#...##......#.#....#...##.#.#..#....#.###.#..###.#..#..###.##.....#...#.#.#.#.##...#..##...."),
        parse("..#..#..#..#####....####..#.###.#.#.##..#......####..###.##...##.#..#..........#..##.##...###.##..##.."),
        parse(".........##..#..#...####..#.##.#.#....#..#..#......#.##..#.##...##.#..........#.#.#.#.#.#......######."),
        parse("...#.##.....###...###.#.##.#######.###.###.####.##.#..##..#...#...#.#####....##...#.####..########..#."),
        parse(".#.#..####..#....#..###..#.#.##...#.###.###..#.....#....#...###.###.######.##.##.#....##.#.##.###...#."),
        parse(".#..#.....#.#.#.#.#....#...###..##..#..#.##...#.###..#......##....#.##.###.#.#.#.###.##.####.#..#..#.."),
        parse("..##....#.#.###.##.###.#.##....##.##.#..#.##...##...###....###..#..#.###.##.#..#.#.#.##.#...#.##.#.#.."),
        parse("...####.#.##.##.#.#.#.....##.#...##...##.#######.###...#..##...#.###..#..#..###..##.#.##.......#.#.##."),
        parse(".#..#####..####.#.###.##.#.#.#.......#..##.####.#...#.###.###..##.....##..#####..####....####.##.#.#.."),
        parse("..#...##.#.###.#.########......#...###...##...##.##..#.#.#...##...#.#.#..#######.###...###.#.#.#.#...."),
        parse("...#..##...#.###.#...#..#.#.##.#.#..#...###..##...##..#.##.##.##..#.#.....#.#######...#..##.###.#..##."),
        parse("..........#...##..#...##...##.#.....#.#..#..#..###......#.#..#.###.###.#...#.###.####.....#....#.#.##."),
        parse("..##.###.######..#######..#...#...#..#.#.##...##.###.#.##.##..##.###.#.###.#.####.###....#.#.#..#....."),
        parse("..#.##...####.#.##.#..##.##..##.#.##...#..##...#####...###.##.##..#..#......#####.....#.#..##..#..#..."),
        parse(".#..#####.###...#..#.....#.##.#..##...#.#...##....####.#..#..####..#..#..##...###...###.######..##...."),
        parse(".##...#...####...###.#.######....##.##..#####.#...###..#..#.#..#...###.#.#.##.###.#...##....#.##...#.."),
        parse("...#.#..##..#.#.#.#..##..#.#..#.##.#.#..#..#.#.#.####..#.........#.####..##.#.#.#.##..##.#..##..#....."),
        parse(".###.......#.##..#.#.##.#####..#..#..#####.###.#.....##.#..#..#..###..#######......####....#....#...#."),
        parse(".##........#...##..#####.###.#....#.#.#.##.#####...#.##..###...######....##.##.#...###.##.#..##..####."),
        parse("...##..##...#.#..####.#.#............#.#...#.#....##..#..########.#.##.##.##.#.#.#.###.##.#.######...."),
        parse(".#.#.##.##..#.##..##..#.##.#..#.#.#.#.#####.#.####.###....##.#..##..#...#.#...####..##.#.#..###.#..##."),
        parse("....##.##..#..#####..#..#.#.#.......#.##..###.##.##.#..###....##..#....####.##.#..#..#.#...#######.##."),
        parse("..##.#.#...####...#.#..#########.#.###..###..#...###.##.#.#.##....##..#.#####..#.##.......#.#.#..##..."),
        parse(".#...#.#...###.#.#.......##......#####..#....###..##.#.#.....#....#...#.###.#.##.####.....#..###.#.##."),
        parse(".##.#####.###.....###..#.##..####...###.#.##.######...#####...#.#..##.#.#####.#####..#..##.#..#.#..##."),
        parse(".#.##.########.#.###...#.#.#.###.###..##.##.#....##..#.#.##.#.......#.######.###..##.####.##.####.###."),
        parse(".###.##..#.#...#...###.....#..#....#.#....###....##...###.#...##......#.##..##..####.###.#.#......##.."),
        parse("..##..##..##..####.##..#.#####.####.#.#.##..#.......#...#..######.#####.#..#.#..##....#.##.#...#...#.."),
        parse("...##........#..#.#.#.#.###.###.#####...##...#.#.##.#.#...###...##..#.##.###.#....#.###..##.##.####.#."),
        parse("..##.#....#..#..##.#.####.##.####..##....##..#..#.#.#.####.#.##.##.#.#.##..#.##.##.###.##.#....#.#.#.."),
        parse("..##..###....#..#.##....#.....#.##.....#.##.#.##.###.#.#..#...#..##.##...##.#.###.###..#.###...#..##.."),
        parse(".##.##..#.#.##...#.###...#.##.#.....#..#..#.##...#.##...#####.......#.###..#.##.##.###.#.#....#.###..."),
        parse(".##.#####....#.....#.#...##.#.#..##...#.#.##...#####...........#.####.....####..#..##...####..####.#.."),
        parse(".#.##...#.##.####.#.#.#.##.#....###.##......###...#.##.#...#####.##..#.##.##...#....##.##.##.###..#.#."),
        parse(".#..#..#.###.##..##..#######..####...#####.##.#.#.#...#..###.#..#.#.#..##..##...##..####.###.##.###..."),
        parse("..###.##....##..#.#..####...#.#.#..#.#..#.#.###....##........#.####.#...#...#..####.#.....#.##....###."),
        parse(".#..#####..##..####.#####.###.#.#..#..#.#.#.##...##...####.#.#.###.....#..##.#.##.##......#...#.#.##.."),
        parse(".#####.##.###....#...#..###..#..#..#####..#..#..###.####..##....##..####.##.##.##.##.#.#..#..##..##..."),
        parse(".#.#..####..#..###..###.#....#.#..#..#..#...##...#####.....###...#.#.....#####.###....#.###...#####..."),
        parse("............####..##.#.####.####.#.##.#.#.#.##...###.#.##.##.####.....##...###....#.#..#.....##.#.#.#."),
        parse(".#......##...##.##..#.##.#.##..#..##.###....##...##....#...##.#.##.#..#....####..#.##...####.....##..."),
        parse(".########..######.#.##..##.#...#...#.##.##...#..##..##..##..#.##...#.##.#.###.###.#.#.#.#..#..#..##..."),
        parse("..#.#..#######.....##..#.##.#..###..##.#..#.#.....###.#...#..###.##..####...##..#.#....###..##.#..#.#."),
        parse("...###..#.##.#.#.....#..#..####..###..#..#...#.........#..#####.####..####...##...#.#.##.#.#.##.###.#."),
        parse("..#.....##.#..######....#.#.#.###.#.#...#.##.....###.##.#..#.##.##.#....###..###.#.#.###..####.#....#."),
        parse("..#..#.......#.#.##...........#...###.#..##...##..#.#...##.####..####......#.#####..######.#.....####."),
        parse(".#..###..##..#.##.####..########.#.##..######..####..#..##.#.#.#.#.###.##.#..##.#..###.##.....#..##..."),
        parse("....######....#.#..#######.#.##.##.#.#..###......##..#..#.####.....#..###.#.###.#...###.#.#..#..#..##."),
        parse(".###.#..#.......##.#..##..########.###..####.##..#.###..#.....#...##.....##..#.#.#.#.####...####.#..#."),
        parse(".##.##...##.#.#..##########....#..#..#.##....#..#..##.####...#......##..#.##..#..####.#.##..##.#.##.#."),
        parse("..#.####.###....#.#....####...###.###..#.#...#..#.####.....#.#.#..##.....#.##...###.####..#.##.#...#.."),
        parse(".#..##.#.#.###.......#.##.....##.###.##..#.###.##.#.########.###.#.###.#.....####..##.#.#.###.#####.#."),
        parse(".#.###.#..###...##....#.....#.####.#.#...###..##.#..##....#...###.#.#.#.#..#..##..#.####...#......##.."),
        parse("..#...#..##.###..#.#.#..##.##..###..#.####.##.#.##.###.#...###.#.###....#.###.....#.#..###.#..#####..."),
        parse(".##..##.#.#.#.#..##..####..#..#..#.##.##.#..#..###...####...#..###....#.###.#.#####.##.###.#.###...#.."),
        parse(".##.....##..#.#..####....##..######...#.......#.##..##....#.#......#..####..###...###....#######.#..#."),
        parse("...###..###..##.####.#.....#.##..#..##..###..#.....##..#.#.........#####.....####.##.#.#...#.####.###."),
        parse(".....###..##.######...#.#.#.#..##..#.##....##..#.....####.#####..#.#..#.#...##...#.##.###.#..######.#."),
        parse("......#......##..#.#...#..##.##....#####..##.#.#.......#..##.#.#..###.####.##..#.#.#.##....#####......"),
        parse(".#####.#####.#.##....###......##...##..####..##..###.##..#..##..#....#.######.#####....###...#.##..#.."),
        parse("..######..###.#.####...#.##.#.##.##..#..#..######.##.###.##..#####..#.####.####.##.###...#....#####..."),
        parse("...##.#.##.#.###.#.##...##.##..##.###..#.....#..#.###...####....#.##.#.#...#####...#.#######.....##..."),
        parse(".###...##.#...#.####.....#........##..#.#.####....#####.#.##...##..########.#...#..###.#..#.#..#.#.##."),
        parse("..#.#.##.####..#####...#..#..###..#.....####.#.#..#.#...#...#....#...####.##....##..#.###.###.#....#.."),
        parse(".#.#...#.##...##.##..#..#..#....##..#...###.#...#.#.#.###...#...#..###...#.#.#.##.#####....##....###.."),
        parse("...#####.####.##.##.##.##...#########.###.##...#.#...####....#..#.####...#...#####....#.#...#..#.#...."),
        parse("..##...#.#.#.#..#.#.#.#..##..#..###.#.....#.#.#.#...#.##.##..#.####.######.##.#.#..#.###.#......#.##.."),
        parse(".##.........###.....#.######.##.....##..####.#.##.####.#..####.##.#.#.#.######.#####....#.#######..#.."),
        parse(".###.##.####.#.....#.#..#.###..###..#..#..#.##..##..#....##...##.#..#.#.#...#..###.#.#..#....#.#.#.##."),
        parse(".#.#..###..##....##...#########..####.##.#..#..#.........##....##.#..#.......###..#....#.##.#...#.#.#."),
        parse(".#..###..####.#..###.#.###..#.......###...#####...###.#...#....##..#..##.#.#.#.#.##.##...#.##....#.#.."),
        parse(".###..###...######.......#.#....#####.....##.#..#....#.#.##..#..#..######..#..##.##.#.##.#.#.#####.##."),
        parse("...#.##.#.##.#..##...###.#####..##........#.#..#.###..##....#..####..#..###...#.#.##.##...##..##..###."),
        parse("...##.#...#...#..###.####..##..#....###.#####...##.##........###....#...####.#....#.#####.#...####.##."),
        parse(".#....#..#...##..#..######..#.##...#.#..#...##.#.#.#.#.###..##.##.####.#.##.##..#...#...###.##.#..##.."),
        parse("..#..###...###.#...##.####.#........#......#..#.#..######.###..#.##..#.#.#.###.######.##.#.#..#..#.##."),
        parse(".#...##....###.#.#....###..#..#...#.##..#.###.......##.##....#..####.##.#.##.#.####.#..#.#..##.#..##.."),
        parse(".#.#...#..#.######.#.#.#....#.###.#.#.###.#.##.##.#.#.##..#.###.###.######..#...####.#.#.##...####.#.."),
        parse(".#.....#.....#######.#.#..##.#..#.#.#......#.#.#.##...#...##########...##.###..##..#......##.#..#....."),
        parse("...##.##.#..#####..#...#.#..#...#....#...##..#####.#..#..#..##.##.#...#.#..####...#.##...#.#.##.#.##.."),
        parse(".##..######...#.##..#.#.#..#..####....#.#....##.#.#....#.###.#..##.####....#.......##..##.#....#..##.."),
        parse(".#######.#####.#..##..#.#####.##.#.#.#..#.....#....######.####.##.##....#.##.#.#.#...##.#.#.#..#...##."),
        parse("...#.##.###....#.#.#.#...#####.#..##..###....####...#..###...#.##.##.#.###.###.###.#...##.#.###.###.#."),
        parse("...##..##.#.####.####...##..##.....#..##.#.##...#.#..##.##.##.##.###..########.#####.#..###.#.##.#..#."),
        parse("......##.#.#..###.##..#..#.#..###.#.####.#.....#.....##...##.##..##.#..###.###.#.....##..#.#..#......."),
        parse(".##...###.####..##.##.#..#######..##.##......#..#.########.#####...#.......##.###.##..#..#...#.######."),
        parse("..#...##.#..###........####.#...###.#.###.#..#.###.##.#.#.#..##..#....#####......##.#.##...#.#.#.###.."),
        parse(".#.###....#.......##.#......#.##..#.....####....#...#.#.###.#####.#....##....#.#.###..#.#.##.#.##..##."),
        parse("..####..##.###.....#.####....##....##..####.#####.#.#.#..###..#.##.#.##...#.##.##.#..##..###.#..#..##."),
        parse("......................................................................................................"),
    ];
    image = extend(image);
    image = extend(image);
    print_image(&image);
    let image = apply(image, &algorithm);
    print_image(&image);
    let image = apply(image, &algorithm);
    print_image(&image);
    println!("Part One: {}", count_enabled_pixels(&image));
    let mut image = image;
    for _ in 2..50 {
        image = apply(image, &algorithm);
    }
    print_image(&image);
    println!("Part Two: {}", count_enabled_pixels(&image));
    // let mut output_image = input_image.clone();
    // for y in 0..(input_image.len() - 2) {
    //     for x in 0..(input_image[y].len() - 2) {
    //         let mut index = 0;
    //         for sub_y in 0..3 {
    //             for sub_x in 0..3 {
    //                 let bit_index = 8 - (sub_y * 3 + sub_x);
    //                 index |= match input_image[y + sub_y][x + sub_x] {
    //                     true => (1 << bit_index),
    //                     _ => 0,
    //                 };
    //             }
    //         }
    //         println!("bits at {} {}: {:09b} = {}", x, y, index, index);
    //         output_image[y + 1][x + 1] = algorithm[index];
    //     }
    // }
    // print_image(&output_image);
}

fn count_enabled_pixels(image: &Vec<Vec<bool>>) -> usize {
    let mut enabled_pixels = 0;
    for row in image.iter() {
        for pixel in row.iter() {
            if *pixel {
                enabled_pixels += 1;
            }
        }
    }
    enabled_pixels
}

fn apply(input: Vec<Vec<bool>>, algorithm: &Vec<bool>) -> Vec<Vec<bool>> {
    let input = extend(input);
    let mut output = input.clone();
    for y in 0..(input.len() - 2) {
        for x in 0..(input[y].len() - 2) {
            let mut index = 0;
            for sub_y in 0..3 {
                for sub_x in 0..3 {
                    let bit_index = 8 - (sub_y * 3 + sub_x);
                    index |= match input[y + sub_y][x + sub_x] {
                        true => (1 << bit_index),
                        _ => 0,
                    };
                }
            }
            output[y + 1][x + 1] = algorithm[index];
        }
    }
    let outer = output[1][1];
    for y in 0..output.len() {
        output[y][0] = outer;
    }
    let end = output[0].len() -1;
    for y in 0..output[end].len() {
        output[y][end] = outer;
    }
    for x in 0..output[0].len() {
        output[0][x] = outer;
    }
    let end = output.len() -1;
    for x in 0..output[end].len() {
        output[end][x] = outer;
    }
    output
}

fn extend(mut input: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let padding = input[0][0];
    let width = input[0].len() + 2;
    let mut output = vec![vec![padding; width]];
    for row in input.iter_mut() {
        let mut output_row = vec![padding];
        output_row.append(row);
        output_row.push(padding);
        output.push(output_row);
    }
    output.push(vec![padding; width]);
    output
}

fn parse(input: &str) -> Vec<bool> {
    input.chars().map(|character| character == '#').collect()
}

fn print_image(image: &Vec<Vec<bool>>) {
    for row in image.iter() {
        for pixel in row.iter() {
            if *pixel {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}