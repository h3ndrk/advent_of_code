use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
enum Item {
    Regular {
        start: usize,
        end: usize,
        level: usize,
        value: usize,
    },
    Pair {
        start: usize,
        end: usize,
        level: usize,
        left: Box<Item>,
        right: Box<Item>,
    },
}

impl Display for Item {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Item::Regular {
                start: _,
                end: _,
                level: _,
                value,
            } => value.fmt(formatter),
            Item::Pair {
                start: _,
                end: _,
                level: _,
                left,
                right,
            } => write!(formatter, "[{},{}]", **left, **right),
        }
    }
}

impl Item {
    fn from_string(input: &String, start: usize, end: usize, level: usize) -> Self {
        if input.chars().nth(0).unwrap() != '[' {
            let value = input.parse().unwrap();
            return Self::Regular {
                start,
                end,
                level,
                value,
            };
        }
        let mut comma_is_at = 0;
        let mut bracket_level = 0;
        for index in 0..input.chars().count() {
            bracket_level = match input.chars().nth(index).unwrap() {
                '[' => bracket_level + 1,
                ']' => bracket_level - 1,
                _ => bracket_level,
            };
            if bracket_level == 1 && input.chars().nth(index).unwrap() == ',' {
                comma_is_at = index;
                break;
            }
        }
        let first_pair_part = &input[1..comma_is_at];
        let second_pair_part = &input[(comma_is_at + 1)..(input.chars().count() - 1)];
        Self::Pair {
            start,
            end,
            level,
            left: Box::new(Self::from_string(
                &first_pair_part.to_string(),
                start + 1,
                start + comma_is_at,
                level + 1,
            )),
            right: Box::new(Self::from_string(
                &second_pair_part.to_string(),
                start + comma_is_at + 1,
                end - 1,
                level + 1,
            )),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::Regular {
                start: _,
                end: _,
                level: _,
                value,
            } => format!("{}", value),
            Self::Pair {
                start: _,
                end: _,
                level: _,
                left,
                right,
            } => format!("[{},{}]", left.to_string(), right.to_string()),
        }
    }

    fn get_start(&self) -> usize {
        match self {
            Self::Regular {
                start,
                end: _,
                level: _,
                value: _,
            } => *start,
            Self::Pair {
                start,
                end: _,
                level: _,
                left: _,
                right: _,
            } => *start,
        }
    }

    fn get_end(&self) -> usize {
        match self {
            Self::Regular {
                start: _,
                end,
                level: _,
                value: _,
            } => *end,
            Self::Pair {
                start: _,
                end,
                level: _,
                left: _,
                right: _,
            } => *end,
        }
    }

    fn is_exploding_pair(&self) -> bool {
        matches!(self, Self::Pair {
            start: _,
            end: _,
            level,
            left,
            right,
        } if *level >= 4 && matches!(**left, Self::Regular {
            start: _,
            end: _,
            level: _,
            value: _,
        }) && matches!(**right, Self::Regular {
            start: _,
            end: _,
            level: _,
            value: _,
        }))
    }

    fn flatten(&self) -> Vec<Self> {
        match self {
            Self::Regular {
                start: _,
                end: _,
                level: _,
                value: _,
            } => vec![self.clone()],
            Self::Pair {
                start: _,
                end: _,
                level: _,
                left,
                right,
            } => [self.clone()]
                .iter()
                .chain(left.flatten().iter().chain(right.flatten().iter()))
                .cloned()
                .collect(),
        }
    }

    fn replace_item(&mut self, old: &Self, new: Self) {
        if self == old {
            *self = new;
            return;
        }
        match self {
            Self::Pair {
                start: _,
                end: _,
                level: _,
                left,
                right,
            } => {
                left.replace_item(old, new.clone());
                right.replace_item(old, new);
            }
            _ => {}
        }
    }

    fn explode(&mut self) -> bool {
        let items = self.flatten();
        let exploding_pair = items.iter().find(|item| item.is_exploding_pair());
        if let Some(exploding_pair) = exploding_pair {
            let item_before = items
                .iter()
                .filter(|item| match item {
                    Self::Regular {
                        start: _,
                        end,
                        level: _,
                        value: _,
                    } => *end < exploding_pair.get_start(),
                    Self::Pair {
                        start: _,
                        end: _,
                        level: _,
                        left: _,
                        right: _,
                    } => false,
                })
                .max_by_key(|item| item.get_end());
            let item_after = items
                .iter()
                .filter(|item| match item {
                    Self::Regular {
                        start,
                        end: _,
                        level: _,
                        value: _,
                    } => *start > exploding_pair.get_end(),
                    Self::Pair {
                        start: _,
                        end: _,
                        level: _,
                        left: _,
                        right: _,
                    } => false,
                })
                .min_by_key(|item| item.get_start());
            if let Some(item_before) = item_before {
                let left_part = match exploding_pair {
                    Self::Pair {
                        start: _,
                        end: _,
                        level: _,
                        left,
                        right: _,
                    } => (**left).clone(),
                    _ => unreachable!(),
                };
                let left_part_value = match left_part {
                    Self::Regular {
                        start: _,
                        end: _,
                        level: _,
                        value,
                    } => value,
                    _ => unreachable!(),
                };
                let (start, end, level, value) = match item_before {
                    Self::Regular {
                        start,
                        end,
                        level,
                        value,
                    } => (*start, *end, *level, *value),
                    _ => unreachable!(),
                };
                self.replace_item(
                    item_before,
                    Self::Regular {
                        start,
                        end,
                        level,
                        value: value + left_part_value,
                    },
                );
            }

            if let Some(item_after) = item_after {
                let right_part = match exploding_pair {
                    Self::Pair {
                        start: _,
                        end: _,
                        level: _,
                        left: _,
                        right,
                    } => (**right).clone(),
                    _ => unreachable!(),
                };
                let left_part_value = match right_part {
                    Self::Regular {
                        start: _,
                        end: _,
                        level: _,
                        value,
                    } => value,
                    _ => unreachable!(),
                };
                let (start, end, level, value) = match item_after {
                    Self::Regular {
                        start,
                        end,
                        level,
                        value,
                    } => (*start, *end, *level, *value),
                    _ => unreachable!(),
                };
                self.replace_item(
                    item_after,
                    Self::Regular {
                        start,
                        end,
                        level,
                        value: value + left_part_value,
                    },
                );
            }

            let (start, end, level) = match exploding_pair {
                Self::Pair {
                    start,
                    end,
                    level,
                    left: _,
                    right: _,
                } => (*start, *end, *level),
                _ => unreachable!(),
            };
            self.replace_item(
                exploding_pair,
                Self::Regular {
                    start,
                    end,
                    level,
                    value: 0,
                },
            );
            return true;
        }
        false
    }

    fn split(&mut self) -> bool {
        let items = self.flatten();
        let item_to_be_splitted = items.iter().find(|item| match item {
            Self::Regular {
                start: _,
                end: _,
                level: _,
                value,
            } => *value >= 10,
            _ => false,
        });
        if let Some(item_to_be_splitted) = item_to_be_splitted {
            let (start, end, level, value) = match item_to_be_splitted {
                Self::Regular {
                    start,
                    end,
                    level,
                    value,
                } => (*start, *end, *level, *value),
                _ => unreachable!(),
            };
            let left_value = value / 2;
            let mut right_value = value / 2;
            if left_value + right_value < value {
                right_value += 1;
            }
            self.replace_item(
                item_to_be_splitted,
                Self::Pair {
                    start,
                    end,
                    level,
                    left: Box::new(Self::Regular {
                        start,
                        end,
                        level,
                        value: left_value,
                    }),
                    right: Box::new(Self::Regular {
                        start,
                        end,
                        level,
                        value: right_value,
                    }),
                },
            );
            return true;
        }
        false
    }

    fn magnitude(&self) -> usize {
        match self {
            Self::Regular {
                start: _,
                end: _,
                level: _,
                value,
            } => *value,
            Self::Pair {
                start: _,
                end: _,
                level: _,
                left,
                right,
            } => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

fn try_explode(number: &mut String) -> bool {
    let mut item = Item::from_string(number, 0, number.chars().count(), 0);
    let exploded = item.explode();
    *number = item.to_string();
    exploded
}

fn try_split(number: &mut String) -> bool {
    let mut item = Item::from_string(number, 0, number.chars().count(), 0);
    let splitted = item.split();
    *number = item.to_string();
    splitted
}

fn process(number: &mut String) {
    let mut dirty = true;
    while dirty {
        dirty = false;
        let exploded = try_explode(number);
        // println!("exploded: {} -> {}", exploded, number);
        if exploded {
            dirty = true;
        } else {
            let splitted = try_split(number);
            // println!("splitted: {} -> {}", splitted, number);
            if splitted {
                dirty = true;
            }
        }
    }
}

fn magnitude(number: &String) -> usize {
    let item = Item::from_string(number, 0, number.chars().count(), 0);
    item.magnitude()
}

fn calculate_sum(numbers: &[&str]) -> String {
    let mut number = numbers.first().unwrap().to_string();
    process(&mut number);
    for next_number in numbers.iter().skip(1) {
        println!("Number: {}", next_number);
        number = format!("[{},{}]", number, next_number);
        process(&mut number);
    }
    number
}

fn calculate_largest_magnitude(numbers: &[&str]) -> usize {
    let mut largest_magnitude = 0;
    for left_number in numbers.iter() {
        for right_number in numbers.iter() {
            let mut number = format!("[{},{}]", left_number, right_number);
            process(&mut number);
            let magnitude = magnitude(&number);
            if magnitude > largest_magnitude {
                largest_magnitude = magnitude;
            }
        }
    }
    largest_magnitude
}

fn main() {
    let mut number = "[[[[[9,8],1],2],3],4]".to_string();
    let exploded = try_explode(&mut number);
    println!("{} {}", exploded, number);
    let mut number = "[7,[6,[5,[4,[3,2]]]]]".to_string();
    let exploded = try_explode(&mut number);
    println!("{} {}", exploded, number);
    let mut number = "[[6,[5,[4,[3,2]]]],1]".to_string();
    let exploded = try_explode(&mut number);
    println!("{} {}", exploded, number);
    let mut number = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".to_string();
    let exploded = try_explode(&mut number);
    println!("{} {}", exploded, number);
    let mut number = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".to_string();
    let exploded = try_explode(&mut number);
    println!("{} {}", exploded, number);
    let mut number = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".to_string();
    let exploded = try_explode(&mut number);
    println!("{} {}", exploded, number);
    let mut number = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".to_string();
    let mut dirty = true;
    while dirty {
        dirty = false;
        let exploded = try_explode(&mut number);
        println!("exploded: {} -> {}", exploded, number);
        if exploded {
            dirty = true;
        } else {
            let splitted = try_split(&mut number);
            println!("splitted: {} -> {}", splitted, number);
            if splitted {
                dirty = true;
            }
        }
    }
    println!(
        "Result: {}",
        calculate_sum(&["[1,1]", "[2,2]", "[3,3]", "[4,4]",])
    );
    println!(
        "Result: {}",
        calculate_sum(&["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"])
    );
    println!(
        "Result: {}",
        calculate_sum(&["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"])
    );
    println!(
        "Result: {}",
        calculate_sum(&[
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ])
    );

    println!("Magnitude: {}", magnitude(&"[9,1]".to_string()));
    println!("Magnitude: {}", magnitude(&"[[9,1],[1,9]]".to_string()));
    println!(
        "Magnitude: {}",
        magnitude(&"[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]".to_string())
    );

    let sum = calculate_sum(&[
        "[[3,[[6,3],[9,6]]],[6,[[0,9],[9,7]]]]",
        "[[[3,9],[[0,8],[7,6]]],[[[7,9],1],[1,3]]]",
        "[8,[[[9,6],[8,4]],4]]",
        "[5,[[1,2],[3,7]]]",
        "[[[[7,7],5],[[3,5],8]],4]",
        "[[[5,[0,7]],3],[[5,[5,3]],[1,[9,4]]]]",
        "[[[[3,5],[7,1]],6],[[[3,6],[5,6]],[[3,2],5]]]",
        "[[[[2,0],[3,0]],[5,7]],[[4,4],[[9,9],[9,3]]]]",
        "[[[[8,0],7],[[7,1],9]],[[3,[8,6]],8]]",
        "[[6,[7,5]],[[6,8],9]]",
        "[[[9,[1,8]],2],[[[4,0],[9,3]],1]]",
        "[[7,[1,[3,8]]],[[4,7],[8,1]]]",
        "[[[5,5],[[4,5],[2,9]]],[[[7,7],0],8]]",
        "[[[[4,7],3],5],[[[4,3],[3,8]],[[6,5],5]]]",
        "[[[[3,8],2],[1,7]],[[[3,1],4],9]]",
        "[[[[2,1],4],[[9,5],[1,4]]],[[3,5],[[9,1],9]]]",
        "[[[6,[1,8]],[0,0]],[9,[0,3]]]",
        "[[[[2,2],[3,3]],[[4,8],4]],[[[6,8],4],5]]",
        "[4,[[[7,8],[3,4]],[[3,2],9]]]",
        "[[[9,0],3],[[[7,1],4],7]]",
        "[[[1,4],8],[[7,5],[[8,0],[0,7]]]]",
        "[9,[[4,6],[[2,9],1]]]",
        "[[[[1,8],8],6],[[[2,0],6],[0,5]]]",
        "[[[5,5],[6,4]],[[3,8],[9,[7,6]]]]",
        "[[0,[8,[1,4]]],2]",
        "[[[[9,5],0],5],[9,[7,5]]]",
        "[[9,[4,8]],[[8,1],[[8,6],[7,1]]]]",
        "[4,[[[9,6],5],9]]",
        "[[[[3,7],6],0],[[7,7],[[2,7],[9,3]]]]",
        "[[[6,[3,7]],[[8,3],2]],[8,[6,[8,5]]]]",
        "[[[5,[2,7]],[[6,7],3]],[5,[[4,4],1]]]",
        "[[1,0],[[2,8],[[0,4],9]]]",
        "[[[1,4],6],[[[9,8],[1,0]],1]]",
        "[[3,4],[[1,[8,4]],8]]",
        "[[[[9,4],[0,7]],[[5,4],[8,2]]],2]",
        "[5,[[[8,7],[3,4]],[2,4]]]",
        "[[[[1,3],[8,6]],[[3,4],6]],[[8,5],[[9,3],[5,7]]]]",
        "[[0,[[0,9],[7,8]]],[3,9]]",
        "[0,[[8,[2,3]],[[3,5],[4,9]]]]",
        "[[[4,3],[[1,9],[1,5]]],[4,[[9,1],1]]]",
        "[[[[3,6],[2,5]],3],[[8,[8,0]],[[6,9],[5,8]]]]",
        "[7,[[3,[3,6]],[[6,9],[2,7]]]]",
        "[[[[8,3],[6,5]],[[3,9],2]],[6,1]]",
        "[[[2,0],[2,3]],8]",
        "[[1,[[8,7],2]],[[[9,4],8],[4,[9,0]]]]",
        "[[[6,7],[[5,2],3]],[[0,5],[[9,4],[2,6]]]]",
        "[[[9,[5,8]],[[9,3],[6,9]]],5]",
        "[[[5,[4,6]],[5,[3,2]]],[2,[9,[5,4]]]]",
        "[8,6]",
        "[[[4,8],[3,1]],[1,[[7,8],[7,5]]]]",
        "[[4,[[8,8],4]],[5,[8,[3,9]]]]",
        "[[[4,[9,0]],[[0,3],5]],[[5,[3,0]],[6,[2,3]]]]",
        "[[[4,0],8],[[[4,0],7],[[9,6],3]]]",
        "[[8,[[7,8],5]],[[[6,2],8],[1,[0,4]]]]",
        "[[1,[[3,4],[0,8]]],[[6,5],3]]",
        "[[5,2],[[8,6],[1,[9,7]]]]",
        "[5,[6,[[1,3],[1,0]]]]",
        "[[0,[[1,9],[5,6]]],[[[6,2],[5,1]],[[1,2],[1,0]]]]",
        "[[[7,1],4],[[[0,3],3],[[4,8],1]]]",
        "[[3,[9,[3,4]]],[1,[[0,0],[1,4]]]]",
        "[1,[7,[1,[3,7]]]]",
        "[[[0,[5,6]],[[7,4],[5,7]]],[[[6,8],[4,6]],9]]",
        "[[[9,8],[7,[1,3]]],3]",
        "[[[4,[0,3]],[[3,0],6]],[[2,[9,2]],1]]",
        "[[[[1,9],[3,3]],[8,1]],5]",
        "[[7,[5,2]],[[4,[0,1]],[3,3]]]",
        "[[[6,6],[0,6]],[[3,[5,9]],[[4,2],[4,3]]]]",
        "[[[7,[5,4]],[7,1]],9]",
        "[[6,[5,2]],[[7,[0,5]],4]]",
        "[[[8,1],[[7,6],[4,1]]],2]",
        "[[[[4,3],[1,4]],[9,6]],[3,[[2,5],3]]]",
        "[[[[9,3],[5,0]],1],[1,[[9,7],9]]]",
        "[[[8,5],[5,9]],[2,[4,[0,0]]]]",
        "[[[[7,9],2],[[8,8],[6,3]]],[7,[0,9]]]",
        "[[[[6,6],[0,2]],[2,[9,0]]],[[0,9],[9,9]]]",
        "[[[9,[1,3]],[6,5]],[[[1,1],8],[9,[7,2]]]]",
        "[[8,[[8,4],6]],[[4,[5,9]],0]]",
        "[[8,[5,[6,7]]],[[[1,9],9],[0,[0,9]]]]",
        "[[9,[9,[7,3]]],[4,[4,7]]]",
        "[[[[9,3],7],5],[[5,[8,5]],[0,[8,0]]]]",
        "[[[5,[9,0]],[[7,4],[5,3]]],[3,[[1,1],[1,8]]]]",
        "[[1,[[1,4],[5,9]]],[[[9,1],[6,5]],[9,[0,7]]]]",
        "[[[[9,4],9],[5,3]],[[[4,2],[2,2]],[[1,0],0]]]",
        "[[[6,[8,6]],9],[8,[[0,1],[9,7]]]]",
        "[[2,0],[5,[[8,3],4]]]",
        "[[[[0,2],0],8],[8,[[2,5],[8,2]]]]",
        "[[[[7,4],8],[9,[7,5]]],[8,[7,[5,3]]]]",
        "[[2,4],[3,[3,8]]]",
        "[[5,4],[[0,[5,8]],[4,3]]]",
        "[6,[[5,[4,7]],9]]",
        "[[[2,[6,8]],[5,5]],[[[3,0],4],[[6,6],[0,1]]]]",
        "[[[1,[4,2]],[[8,0],8]],[8,[[6,1],[0,0]]]]",
        "[[9,[2,[3,3]]],[[2,6],[[5,2],[5,8]]]]",
        "[[9,[4,4]],[[[8,6],1],2]]",
        "[2,[[[0,7],7],[[7,8],5]]]",
        "[[[4,0],[[1,1],[7,6]]],[[6,7],[[7,2],1]]]",
        "[[[[2,5],0],[[9,5],9]],[6,[7,[6,1]]]]",
        "[[[7,8],1],[[[6,2],0],[[9,7],[3,5]]]]",
        "[[[9,1],0],[3,[[6,1],[6,9]]]]",
        "[[[[9,0],0],[4,[7,0]]],[[6,[4,0]],[8,[4,2]]]]",
    ]);

    println!("Sum: {}", sum);
    println!("Magnitude: {}", magnitude(&sum));

    println!(
        "Largest pair-wise magnitude: {}",
        calculate_largest_magnitude(&[
            "[[3,[[6,3],[9,6]]],[6,[[0,9],[9,7]]]]",
            "[[[3,9],[[0,8],[7,6]]],[[[7,9],1],[1,3]]]",
            "[8,[[[9,6],[8,4]],4]]",
            "[5,[[1,2],[3,7]]]",
            "[[[[7,7],5],[[3,5],8]],4]",
            "[[[5,[0,7]],3],[[5,[5,3]],[1,[9,4]]]]",
            "[[[[3,5],[7,1]],6],[[[3,6],[5,6]],[[3,2],5]]]",
            "[[[[2,0],[3,0]],[5,7]],[[4,4],[[9,9],[9,3]]]]",
            "[[[[8,0],7],[[7,1],9]],[[3,[8,6]],8]]",
            "[[6,[7,5]],[[6,8],9]]",
            "[[[9,[1,8]],2],[[[4,0],[9,3]],1]]",
            "[[7,[1,[3,8]]],[[4,7],[8,1]]]",
            "[[[5,5],[[4,5],[2,9]]],[[[7,7],0],8]]",
            "[[[[4,7],3],5],[[[4,3],[3,8]],[[6,5],5]]]",
            "[[[[3,8],2],[1,7]],[[[3,1],4],9]]",
            "[[[[2,1],4],[[9,5],[1,4]]],[[3,5],[[9,1],9]]]",
            "[[[6,[1,8]],[0,0]],[9,[0,3]]]",
            "[[[[2,2],[3,3]],[[4,8],4]],[[[6,8],4],5]]",
            "[4,[[[7,8],[3,4]],[[3,2],9]]]",
            "[[[9,0],3],[[[7,1],4],7]]",
            "[[[1,4],8],[[7,5],[[8,0],[0,7]]]]",
            "[9,[[4,6],[[2,9],1]]]",
            "[[[[1,8],8],6],[[[2,0],6],[0,5]]]",
            "[[[5,5],[6,4]],[[3,8],[9,[7,6]]]]",
            "[[0,[8,[1,4]]],2]",
            "[[[[9,5],0],5],[9,[7,5]]]",
            "[[9,[4,8]],[[8,1],[[8,6],[7,1]]]]",
            "[4,[[[9,6],5],9]]",
            "[[[[3,7],6],0],[[7,7],[[2,7],[9,3]]]]",
            "[[[6,[3,7]],[[8,3],2]],[8,[6,[8,5]]]]",
            "[[[5,[2,7]],[[6,7],3]],[5,[[4,4],1]]]",
            "[[1,0],[[2,8],[[0,4],9]]]",
            "[[[1,4],6],[[[9,8],[1,0]],1]]",
            "[[3,4],[[1,[8,4]],8]]",
            "[[[[9,4],[0,7]],[[5,4],[8,2]]],2]",
            "[5,[[[8,7],[3,4]],[2,4]]]",
            "[[[[1,3],[8,6]],[[3,4],6]],[[8,5],[[9,3],[5,7]]]]",
            "[[0,[[0,9],[7,8]]],[3,9]]",
            "[0,[[8,[2,3]],[[3,5],[4,9]]]]",
            "[[[4,3],[[1,9],[1,5]]],[4,[[9,1],1]]]",
            "[[[[3,6],[2,5]],3],[[8,[8,0]],[[6,9],[5,8]]]]",
            "[7,[[3,[3,6]],[[6,9],[2,7]]]]",
            "[[[[8,3],[6,5]],[[3,9],2]],[6,1]]",
            "[[[2,0],[2,3]],8]",
            "[[1,[[8,7],2]],[[[9,4],8],[4,[9,0]]]]",
            "[[[6,7],[[5,2],3]],[[0,5],[[9,4],[2,6]]]]",
            "[[[9,[5,8]],[[9,3],[6,9]]],5]",
            "[[[5,[4,6]],[5,[3,2]]],[2,[9,[5,4]]]]",
            "[8,6]",
            "[[[4,8],[3,1]],[1,[[7,8],[7,5]]]]",
            "[[4,[[8,8],4]],[5,[8,[3,9]]]]",
            "[[[4,[9,0]],[[0,3],5]],[[5,[3,0]],[6,[2,3]]]]",
            "[[[4,0],8],[[[4,0],7],[[9,6],3]]]",
            "[[8,[[7,8],5]],[[[6,2],8],[1,[0,4]]]]",
            "[[1,[[3,4],[0,8]]],[[6,5],3]]",
            "[[5,2],[[8,6],[1,[9,7]]]]",
            "[5,[6,[[1,3],[1,0]]]]",
            "[[0,[[1,9],[5,6]]],[[[6,2],[5,1]],[[1,2],[1,0]]]]",
            "[[[7,1],4],[[[0,3],3],[[4,8],1]]]",
            "[[3,[9,[3,4]]],[1,[[0,0],[1,4]]]]",
            "[1,[7,[1,[3,7]]]]",
            "[[[0,[5,6]],[[7,4],[5,7]]],[[[6,8],[4,6]],9]]",
            "[[[9,8],[7,[1,3]]],3]",
            "[[[4,[0,3]],[[3,0],6]],[[2,[9,2]],1]]",
            "[[[[1,9],[3,3]],[8,1]],5]",
            "[[7,[5,2]],[[4,[0,1]],[3,3]]]",
            "[[[6,6],[0,6]],[[3,[5,9]],[[4,2],[4,3]]]]",
            "[[[7,[5,4]],[7,1]],9]",
            "[[6,[5,2]],[[7,[0,5]],4]]",
            "[[[8,1],[[7,6],[4,1]]],2]",
            "[[[[4,3],[1,4]],[9,6]],[3,[[2,5],3]]]",
            "[[[[9,3],[5,0]],1],[1,[[9,7],9]]]",
            "[[[8,5],[5,9]],[2,[4,[0,0]]]]",
            "[[[[7,9],2],[[8,8],[6,3]]],[7,[0,9]]]",
            "[[[[6,6],[0,2]],[2,[9,0]]],[[0,9],[9,9]]]",
            "[[[9,[1,3]],[6,5]],[[[1,1],8],[9,[7,2]]]]",
            "[[8,[[8,4],6]],[[4,[5,9]],0]]",
            "[[8,[5,[6,7]]],[[[1,9],9],[0,[0,9]]]]",
            "[[9,[9,[7,3]]],[4,[4,7]]]",
            "[[[[9,3],7],5],[[5,[8,5]],[0,[8,0]]]]",
            "[[[5,[9,0]],[[7,4],[5,3]]],[3,[[1,1],[1,8]]]]",
            "[[1,[[1,4],[5,9]]],[[[9,1],[6,5]],[9,[0,7]]]]",
            "[[[[9,4],9],[5,3]],[[[4,2],[2,2]],[[1,0],0]]]",
            "[[[6,[8,6]],9],[8,[[0,1],[9,7]]]]",
            "[[2,0],[5,[[8,3],4]]]",
            "[[[[0,2],0],8],[8,[[2,5],[8,2]]]]",
            "[[[[7,4],8],[9,[7,5]]],[8,[7,[5,3]]]]",
            "[[2,4],[3,[3,8]]]",
            "[[5,4],[[0,[5,8]],[4,3]]]",
            "[6,[[5,[4,7]],9]]",
            "[[[2,[6,8]],[5,5]],[[[3,0],4],[[6,6],[0,1]]]]",
            "[[[1,[4,2]],[[8,0],8]],[8,[[6,1],[0,0]]]]",
            "[[9,[2,[3,3]]],[[2,6],[[5,2],[5,8]]]]",
            "[[9,[4,4]],[[[8,6],1],2]]",
            "[2,[[[0,7],7],[[7,8],5]]]",
            "[[[4,0],[[1,1],[7,6]]],[[6,7],[[7,2],1]]]",
            "[[[[2,5],0],[[9,5],9]],[6,[7,[6,1]]]]",
            "[[[7,8],1],[[[6,2],0],[[9,7],[3,5]]]]",
            "[[[9,1],0],[3,[[6,1],[6,9]]]]",
            "[[[[9,0],0],[4,[7,0]]],[[6,[4,0]],[8,[4,2]]]]",
        ])
    );
}
