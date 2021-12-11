const WIDTH: usize = 10;
const HEIGHT: usize = 10;

fn main() {
    let mut octopusses = [
        // [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
        // [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
        // [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
        // [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
        // [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
        // [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
        // [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
        // [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
        // [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
        // [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        // [1, 1, 1, 1, 1],
        // [1, 9, 9, 9, 1],
        // [1, 9, 1, 9, 1],
        // [1, 9, 9, 9, 1],
        // [1, 1, 1, 1, 1],
        [5, 2, 1, 2, 1, 6, 6, 7, 1, 6],
        [1, 5, 6, 7, 3, 2, 2, 5, 8, 1],
        [2, 2, 6, 8, 4, 6, 1, 5, 4, 8],
        [3, 4, 8, 1, 5, 6, 1, 7, 4, 4],
        [6, 2, 4, 8, 3, 4, 2, 2, 4, 8],
        [6, 5, 2, 6, 6, 6, 7, 3, 6, 8],
        [5, 6, 2, 7, 3, 3, 5, 7, 7, 5],
        [8, 1, 2, 4, 5, 1, 1, 7, 5, 4],
        [4, 6, 1, 4, 1, 3, 7, 6, 8, 3],
        [4, 7, 2, 4, 5, 6, 1, 1, 5, 6],
    ];

    print_octopusses(&octopusses);
    let mut number_of_flashes = 0;
    for _ in 0..100 {
        number_of_flashes += calculate_step(&mut octopusses);
    }
    print_octopusses(&octopusses);
    println!("Part One: {}", number_of_flashes);

    let mut octopusses = [
        [5, 2, 1, 2, 1, 6, 6, 7, 1, 6],
        [1, 5, 6, 7, 3, 2, 2, 5, 8, 1],
        [2, 2, 6, 8, 4, 6, 1, 5, 4, 8],
        [3, 4, 8, 1, 5, 6, 1, 7, 4, 4],
        [6, 2, 4, 8, 3, 4, 2, 2, 4, 8],
        [6, 5, 2, 6, 6, 6, 7, 3, 6, 8],
        [5, 6, 2, 7, 3, 3, 5, 7, 7, 5],
        [8, 1, 2, 4, 5, 1, 1, 7, 5, 4],
        [4, 6, 1, 4, 1, 3, 7, 6, 8, 3],
        [4, 7, 2, 4, 5, 6, 1, 1, 5, 6],
    ];

    print_octopusses(&octopusses);
    let mut step_index = 0;
    loop {
        step_index += 1;
        let number_of_flashes_in_this_step = calculate_step(&mut octopusses);
        if number_of_flashes_in_this_step == WIDTH * HEIGHT {
            println!("Part Two: {}", step_index);
            break;
        }
    }
    print_octopusses(&octopusses);
}

fn calculate_step(octopusses: &mut [[usize; WIDTH]; HEIGHT]) -> usize {
    println!("Calculating step...");
    let mut flashed_in_this_step = [[false; WIDTH]; HEIGHT];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            octopusses[y][x] += 1;
        }
    }
    calculate_flashes(octopusses, &mut flashed_in_this_step);
    let mut number_of_flashes = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if flashed_in_this_step[y][x] {
                octopusses[y][x] = 0;
                number_of_flashes += 1;
            }
        }
    }
    number_of_flashes
}

fn calculate_flashes(
    octopusses: &mut [[usize; WIDTH]; HEIGHT],
    flashed_in_this_step: &mut [[bool; WIDTH]; HEIGHT],
) {
    let mut continue_to_flash = true;
    while continue_to_flash {
        continue_to_flash = false;
        'outer: for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if octopusses[y][x] > 9 && !flashed_in_this_step[y][x] {
                    flash_at(octopusses, y, x);
                    flashed_in_this_step[y][x] = true;
                    continue_to_flash = true;
                    break 'outer;
                }
            }
        }
    }
}

fn flash_at(octopusses: &mut [[usize; WIDTH]; HEIGHT], center_y: usize, center_x: usize) {
    let start_y = if center_y > 0 { center_y - 1 } else { 0 };
    let end_y = if center_y < HEIGHT - 1 {
        center_y + 1
    } else {
        center_y
    };
    let start_x = if center_x > 0 { center_x - 1 } else { 0 };
    let end_x = if center_x < WIDTH - 1 {
        center_x + 1
    } else {
        center_x
    };
    // println!(
    //     "flash: center_y={}, center_x={}, start_y={}, end_y={}, start_x={}, end_x={}",
    //     center_y, center_x, start_y, end_y, start_x, end_x
    // );
    for y in start_y..=end_y {
        for x in start_x..=end_x {
            if y != center_y || x != center_x {
                octopusses[y][x] += 1;
            }
        }
    }
}

fn print_octopusses(octopusses: &[[usize; WIDTH]; HEIGHT]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", octopusses[y][x]);
        }
        println!();
    }
}
