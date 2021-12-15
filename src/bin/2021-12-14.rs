use std::collections::HashMap;

fn main() {
    // let example_rules = HashMap::from([
    //     (('C', 'H'), 'B'),
    //     (('H', 'H'), 'N'),
    //     (('C', 'B'), 'H'),
    //     (('N', 'H'), 'C'),
    //     (('H', 'B'), 'C'),
    //     (('H', 'C'), 'B'),
    //     (('H', 'N'), 'C'),
    //     (('N', 'N'), 'C'),
    //     (('B', 'H'), 'H'),
    //     (('N', 'C'), 'B'),
    //     (('N', 'B'), 'B'),
    //     (('B', 'N'), 'B'),
    //     (('B', 'B'), 'N'),
    //     (('B', 'C'), 'B'),
    //     (('C', 'C'), 'N'),
    //     (('C', 'N'), 'C'),
    // ]);
    let puzzle_rules = HashMap::from([
        (('F', 'V'), 'C'),
        (('C', 'P'), 'K'),
        (('F', 'S'), 'K'),
        (('V', 'F'), 'N'),
        (('H', 'N'), 'F'),
        (('F', 'F'), 'N'),
        (('S', 'S'), 'K'),
        (('V', 'S'), 'V'),
        (('B', 'V'), 'F'),
        (('H', 'C'), 'K'),
        (('B', 'P'), 'F'),
        (('O', 'V'), 'N'),
        (('B', 'F'), 'V'),
        (('V', 'H'), 'V'),
        (('P', 'F'), 'N'),
        (('F', 'C'), 'S'),
        (('C', 'S'), 'B'),
        (('F', 'K'), 'N'),
        (('V', 'K'), 'H'),
        (('F', 'N'), 'P'),
        (('S', 'H'), 'V'),
        (('C', 'V'), 'K'),
        (('H', 'P'), 'K'),
        (('H', 'O'), 'C'),
        (('N', 'O'), 'V'),
        (('C', 'K'), 'C'),
        (('V', 'B'), 'S'),
        (('O', 'C'), 'N'),
        (('N', 'S'), 'C'),
        (('N', 'F'), 'H'),
        (('S', 'F'), 'N'),
        (('N', 'K'), 'S'),
        (('N', 'P'), 'P'),
        (('O', 'O'), 'S'),
        (('N', 'H'), 'C'),
        (('B', 'C'), 'H'),
        (('K', 'S'), 'H'),
        (('P', 'V'), 'O'),
        (('K', 'O'), 'K'),
        (('O', 'K'), 'H'),
        (('O', 'H'), 'H'),
        (('B', 'H'), 'F'),
        (('N', 'B'), 'B'),
        (('F', 'H'), 'N'),
        (('H', 'V'), 'F'),
        (('B', 'N'), 'S'),
        (('O', 'N'), 'V'),
        (('C', 'B'), 'V'),
        (('C', 'F'), 'H'),
        (('F', 'B'), 'S'),
        (('K', 'F'), 'S'),
        (('P', 'S'), 'P'),
        (('O', 'B'), 'C'),
        (('N', 'N'), 'K'),
        (('K', 'V'), 'C'),
        (('B', 'K'), 'H'),
        (('S', 'N'), 'S'),
        (('N', 'C'), 'H'),
        (('P', 'K'), 'B'),
        (('P', 'C'), 'H'),
        (('K', 'N'), 'S'),
        (('V', 'O'), 'V'),
        (('F', 'O'), 'K'),
        (('C', 'H'), 'B'),
        (('P', 'H'), 'N'),
        (('S', 'O'), 'C'),
        (('K', 'H'), 'S'),
        (('H', 'B'), 'V'),
        (('H', 'H'), 'B'),
        (('B', 'B'), 'H'),
        (('S', 'C'), 'V'),
        (('H', 'S'), 'K'),
        (('S', 'P'), 'V'),
        (('K', 'B'), 'N'),
        (('V', 'N'), 'H'),
        (('H', 'K'), 'H'),
        (('K', 'P'), 'K'),
        (('O', 'P'), 'F'),
        (('C', 'O'), 'B'),
        (('V', 'P'), 'H'),
        (('O', 'S'), 'N'),
        (('O', 'F'), 'H'),
        (('K', 'K'), 'N'),
        (('C', 'C'), 'K'),
        (('B', 'S'), 'C'),
        (('V', 'V'), 'O'),
        (('C', 'N'), 'H'),
        (('P', 'B'), 'P'),
        (('B', 'O'), 'N'),
        (('S', 'B'), 'H'),
        (('F', 'P'), 'F'),
        (('S', 'K'), 'F'),
        (('P', 'O'), 'S'),
        (('K', 'C'), 'H'),
        (('V', 'C'), 'H'),
        (('N', 'V'), 'N'),
        (('H', 'F'), 'B'),
        (('P', 'N'), 'F'),
        (('S', 'V'), 'K'),
        (('P', 'P'), 'K'),
    ]);
    println!(
        "Part One: {}",
        calculate_counts2(&puzzle_rules, "PFVKOBSHPSPOOOCOOHBP", 10)
    );
    println!("Part Two: {}", calculate_counts2(&puzzle_rules, "PFVKOBSHPSPOOOCOOHBP", 40));
}

enum ParentIterator {
    Chars(&'static str, usize),
    PolymerIterator(PolymerIterator),
}

impl From<&'static str> for ParentIterator {
    fn from(input: &'static str) -> Self {
        Self::Chars(input, 0)
    }
}

impl From<PolymerIterator> for ParentIterator {
    fn from(iterator: PolymerIterator) -> Self {
        Self::PolymerIterator(iterator)
    }
}

impl Iterator for ParentIterator {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ParentIterator::Chars(string, index) => {
                println!("ParentIterator::Chars::next -> {}", index);
                let character = string.chars().nth(*index);
                *index += 1;
                character
            }
            ParentIterator::PolymerIterator(iterator) => iterator.next(),
        }
    }
}

struct PolymerIterator {
    rules: HashMap<(char, char), char>,
    parent: Box<ParentIterator>,
    current: char,
    next: Option<char>,
    reached_end: bool,
}

impl PolymerIterator {
    fn from_str(input: &'static str, rules: &HashMap<(char, char), char>) -> Self {
        let mut parent: ParentIterator = input.into();
        let current = parent.next();
        let reached_end = current.is_none();
        Self {
            rules: rules.clone(),
            parent: Box::new(parent),
            current: if reached_end { '?' } else { current.unwrap() },
            next: None,
            reached_end,
        }
    }

    fn from_iterator(iterator: PolymerIterator, rules: &HashMap<(char, char), char>) -> Self {
        let mut parent: ParentIterator = iterator.into();
        let current = parent.next();
        let reached_end = current.is_none();
        Self {
            rules: rules.clone(),
            parent: Box::new(parent),
            current: if reached_end { '?' } else { current.unwrap() },
            next: None,
            reached_end,
        }
    }
}

impl Iterator for PolymerIterator {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reached_end {
            return None;
        }
        match self.next {
            Some(next) => {
                let generated = self.rules[&(self.current, next)];
                self.current = next;
                self.next = None;
                Some(generated)
            }
            None => {
                let next = self.parent.next();
                match next {
                    Some(next) => {
                        self.next = Some(next);
                    }
                    None => {
                        self.reached_end = true;
                    }
                }
                Some(self.current)
            }
        }
    }
}

fn calculate_counts(
    rules: &HashMap<(char, char), char>,
    polymer_template: &'static str,
    steps: usize,
) -> usize {
    let mut iterator = PolymerIterator::from_str(polymer_template, rules);
    for _ in 1..steps {
        iterator = PolymerIterator::from_iterator(iterator, rules);
    }
    let counts = iterator.fold(HashMap::new(), |counts: HashMap<char, usize>, character| {
        let mut counts: HashMap<char, usize> = counts
            .iter()
            .map(|(counted_character, amount)| {
                if *counted_character == character {
                    (*counted_character, *amount + 1)
                } else {
                    (*counted_character, *amount)
                }
            })
            .collect();
        if !counts.contains_key(&character) {
            counts.insert(character, 1);
        }
        counts
    });
    println!("Counts: {:?}", counts);
    let minimum_amount = counts
        .iter()
        .min_by_key(|(_character, count)| *count)
        .unwrap()
        .1;
    let maximum_amount = counts
        .iter()
        .max_by_key(|(_character, count)| *count)
        .unwrap()
        .1;
    println!("Minimum Amount: {}", minimum_amount);
    println!("Maximum Amount: {}", maximum_amount);
    maximum_amount - minimum_amount
}

fn calculate_counts2(
    rules: &HashMap<(char, char), char>,
    polymer_template: &'static str,
    steps: usize,
) -> usize {
    let mut pairs: HashMap<(char, char), usize> = HashMap::new();
    for index in 1..polymer_template.chars().count() {
        let entry = pairs
            .entry((
                polymer_template.chars().nth(index - 1).unwrap(),
                polymer_template.chars().nth(index).unwrap(),
            ))
            .or_insert(0);
        *entry += 1;
    }
    // println!("pairs: {:?}", pairs);
    for _ in 0..steps {
        // println!("step");
        for (characters, count) in pairs.clone().iter() {
            let (left_character, right_character) = characters;
            let middle_character = rules[characters];
            // println!(
            //     "{}{} -> {}{}, {}{}",
            //     left_character,
            //     right_character,
            //     left_character,
            //     middle_character,
            //     middle_character,
            //     right_character
            // );
            let previous_entry = pairs.entry(*characters).or_insert(0);
            // println!("{}{} -= {}", left_character, right_character, count);
            *previous_entry -= count;
            let next_left_entry = pairs
                .entry((*left_character, middle_character))
                .or_insert(0);
            // println!("{}{} += {}", left_character, middle_character, count);
            *next_left_entry += count;
            let next_right_entry = pairs
                .entry((middle_character, *right_character))
                .or_insert(0);
            // println!("{}{} += {}", middle_character, right_character, count);
            *next_right_entry += count;
        }
        // println!("pairs: {:?}", pairs);
    }
    let mut counts: HashMap<char, usize> = HashMap::new();
    let entry = counts
        .entry(polymer_template.chars().last().unwrap())
        .or_insert(0);
    *entry += 1;
    for (characters, count) in pairs.iter() {
        let entry = counts.entry(characters.0).or_insert(0);
        *entry += count;
    }
    // println!("{:?}", counts);
    let minimum_amount = counts
        .iter()
        .min_by_key(|(_character, count)| *count)
        .unwrap()
        .1;
    let maximum_amount = counts
        .iter()
        .max_by_key(|(_character, count)| *count)
        .unwrap()
        .1;
    println!("Minimum Amount: {}", minimum_amount);
    println!("Maximum Amount: {}", maximum_amount);
    maximum_amount - minimum_amount
}
