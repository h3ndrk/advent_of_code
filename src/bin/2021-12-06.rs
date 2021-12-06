fn main() {
    let laternfish = [
        1, 1, 1, 2, 1, 5, 1, 1, 2, 1, 4, 1, 4, 1, 1, 1, 1, 1, 1, 4, 1, 1, 1, 1, 4, 1, 1, 5, 1, 3,
        1, 2, 1, 1, 1, 2, 1, 1, 1, 4, 1, 1, 3, 1, 5, 1, 1, 1, 1, 3, 5, 5, 2, 1, 1, 1, 2, 1, 1, 1,
        1, 1, 1, 1, 1, 5, 4, 1, 1, 1, 1, 1, 3, 1, 1, 2, 4, 4, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 5,
        1, 3, 1, 5, 1, 2, 1, 1, 5, 1, 1, 1, 5, 3, 3, 1, 4, 1, 3, 1, 3, 1, 1, 1, 1, 3, 1, 4, 1, 1,
        1, 1, 1, 2, 1, 1, 1, 4, 2, 1, 1, 5, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1,
        5, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 3, 4, 1, 2, 1, 3, 2, 1, 1, 2, 1, 1, 1, 1, 4, 1, 1, 1, 1,
        4, 1, 1, 1, 1, 1, 2, 1, 1, 4, 1, 1, 1, 5, 3, 2, 2, 1, 1, 3, 1, 5, 1, 5, 1, 1, 1, 1, 1, 5,
        1, 4, 1, 2, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 3, 1, 4, 3, 1, 4, 1, 3, 2,
        1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 5, 1, 1, 1, 1, 2, 1, 1, 1, 3, 5, 1,
        1, 1, 1, 5, 1, 1, 2, 1, 2, 4, 2, 2, 1, 1, 1, 5, 2, 1, 1, 5, 1, 1, 1, 1, 5, 1, 1, 1, 2, 1,
    ];

    println!("Part One: {}", number_of_lanternfish(&laternfish, 80));
    println!("Part Two: {}", number_of_lanternfish(&laternfish, 256));
}

fn number_of_lanternfish(existing: &[usize], days: usize) -> usize {
    let mut amount_of_timer_values = [0; 9];
    for timer_value in existing {
        amount_of_timer_values[*timer_value] += 1;
    }

    for _ in 0..days {
        amount_of_timer_values = [
            amount_of_timer_values[1],
            amount_of_timer_values[2],
            amount_of_timer_values[3],
            amount_of_timer_values[4],
            amount_of_timer_values[5],
            amount_of_timer_values[6],
            amount_of_timer_values[7] + amount_of_timer_values[0],
            amount_of_timer_values[8],
            amount_of_timer_values[0],
        ];
    }

    amount_of_timer_values.iter().sum()
}
