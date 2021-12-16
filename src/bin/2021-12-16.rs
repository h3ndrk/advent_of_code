fn main() {
    let packet = parse_hexadecimal("A20D5CECBD6C061006E7801224AF251AEA06D2319904921880113A931A1402A9D83D43C9FFCC1E56FF29890E00C42984337BF22C502008C26982801009426937320124E602BC01192F4A74FD7B70692F4A74FD7B700403170400F7002DC00E7003C400B0023700082C601DF8C00D30038005AA0013F40044E7002D400D10030C008000574000AB958B4B8011074C0249769913893469A72200B42673F26A005567FCC13FE673004F003341006615421830200F4608E7142629294F92861A840118F1184C0129637C007C24B19AA2C96335400013B0C0198F716213180370AE39C7620043E0D4788B440232CB34D80260008645C86D16C401B85D0BA2D18025A00ACE7F275324137FD73428200ECDFBEFF2BDCDA70D5FE5339D95B3B6C98C1DA006772F9DC9025B057331BF7D8B65108018092599C669B4B201356763475D00480010E89709E090002130CA28C62300265C188034BA007CA58EA6FB4CDA12799FD8098021400F94A6F95E3ECC73A77359A4EFCB09CEF799A35280433D1BCCA666D5EFD6A5A389542A7DCCC010958D85EC0119EED04A73F69703669466A048C01E14FFEFD229ADD052466ED37BD8B4E1D10074B3FF8CF2BBE0094D56D7E38CADA0FA80123C8F75F9C764D29DA814E4693C4854C0118AD3C0A60144E364D944D02C99F4F82100607600AC8F6365C91EC6CBB3A072C404011CE8025221D2A0337158200C97001F6978A1CE4FFBE7C4A5050402E9ECEE709D3FE7296A894F4C6A75467EB8959F4C013815C00FACEF38A7297F42AD2600B7006A0200EC538D51500010B88919624CE694C0027B91951125AFF7B9B1682040253D006E8000844138F105C0010D84D1D2304B213007213900D95B73FE914CC9FCBFA9EEA81802FA0094A34CA3649F019800B48890C2382002E727DF7293C1B900A160008642B87312C0010F8DB08610080331720FC580");
    println!("Part One: {}", packet.sum());
    println!("Part Two: {}", packet.evaluate());
}

#[derive(Debug)]
enum Packet {
    Literal {
        version: usize,
        value: usize,
    },
    Sum {
        version: usize,
        operands: Vec<Packet>,
    },
    Product {
        version: usize,
        operands: Vec<Packet>,
    },
    Minimum {
        version: usize,
        operands: Vec<Packet>,
    },
    Maximum {
        version: usize,
        operands: Vec<Packet>,
    },
    GreaterThan {
        version: usize,
        operands: Vec<Packet>,
    },
    LessThan {
        version: usize,
        operands: Vec<Packet>,
    },
    EqualTo {
        version: usize,
        operands: Vec<Packet>,
    },
}

impl Packet {
    fn new_operator(version: usize, type_id: usize, operands: Vec<Self>) -> Self {
        match type_id {
            0 => Self::Sum { version, operands },
            1 => Self::Product { version, operands },
            2 => Self::Minimum { version, operands },
            3 => Self::Maximum { version, operands },
            5 => Self::GreaterThan { version, operands },
            6 => Self::LessThan { version, operands },
            7 => Self::EqualTo { version, operands },
            _ => unreachable!(),
        }
    }

    fn sum(&self) -> usize {
        match self {
            Packet::Literal { version, value: _ } => *version,
            Packet::Sum { version, operands }
            | Packet::Product { version, operands }
            | Packet::Minimum { version, operands }
            | Packet::Maximum { version, operands }
            | Packet::GreaterThan { version, operands }
            | Packet::LessThan { version, operands }
            | Packet::EqualTo { version, operands } => {
                version + operands.iter().fold(0, |sum, operand| sum + operand.sum())
            }
        }
    }

    fn evaluate(&self) -> usize {
        match self {
            Packet::Literal { version: _, value } => *value,
            Packet::Sum {
                version: _,
                operands,
            } => operands
                .iter()
                .fold(0, |sum, operand| sum + operand.evaluate()),
            Packet::Product {
                version: _,
                operands,
            } => operands
                .iter()
                .fold(1, |product, operand| product * operand.evaluate()),
            Packet::Minimum {
                version: _,
                operands,
            } => operands
                .iter()
                .map(|operand| operand.evaluate())
                .min()
                .unwrap(),
            Packet::Maximum {
                version: _,
                operands,
            } => operands
                .iter()
                .map(|operand| operand.evaluate())
                .max()
                .unwrap(),
            Packet::GreaterThan {
                version: _,
                operands,
            } => {
                if operands[0].evaluate() > operands[1].evaluate() {
                    1
                } else {
                    0
                }
            }
            Packet::LessThan {
                version: _,
                operands,
            } => {
                if operands[0].evaluate() < operands[1].evaluate() {
                    1
                } else {
                    0
                }
            }
            Packet::EqualTo {
                version: _,
                operands,
            } => {
                if operands[0].evaluate() == operands[1].evaluate() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn parse_hexadecimal(input: &'static str) -> Packet {
    // println!("parse_hexadecimal({})", input);
    let binary: String = input
        .chars()
        .fold(String::new(), |packet, hexadecimal| {
            packet
                + match hexadecimal {
                    '0' => "0000",
                    '1' => "0001",
                    '2' => "0010",
                    '3' => "0011",
                    '4' => "0100",
                    '5' => "0101",
                    '6' => "0110",
                    '7' => "0111",
                    '8' => "1000",
                    '9' => "1001",
                    'A' => "1010",
                    'B' => "1011",
                    'C' => "1100",
                    'D' => "1101",
                    'E' => "1110",
                    'F' => "1111",
                    _ => unreachable!(),
                }
        })
        .into();
    parse_binary(&binary[..]).0
}

fn parse_binary(input: &str) -> (Packet, usize) {
    // println!("parse_binary({})", input);
    let version = parse_binary_as_decimal(&input[0..3]);
    let type_id = parse_binary_as_decimal(&input[3..6]);
    match type_id {
        4 => parse_literal_value_groups(version, &input[6..]),
        _ => parse_operator(version, type_id, &input[6..]),
    }
}

fn parse_literal_value_groups(version: usize, input: &str) -> (Packet, usize) {
    // println!("parse_binary({}, {}, {})", version, type_id, input);
    let mut start = 0;
    let mut parsed_groups = vec![];
    loop {
        let (parsed_group, more_to_read) = parse_literal_value_group(&input[start..(start + 5)]);
        parsed_groups.push(parsed_group);
        start += 5;
        if !more_to_read {
            break;
        }
    }
    let value = parsed_groups
        .iter()
        .rev()
        .enumerate()
        .fold(0, |value, (index, group)| value + (group << (index * 4)));
    // println!("Used Bits: {}, Value: {}", 6 + start, value);
    (Packet::Literal { version, value }, 6 + start)
}

fn parse_operator(version: usize, type_id: usize, input: &str) -> (Packet, usize) {
    // println!("parse_operator({}, {}, {})", version, type_id, input);
    match input.chars().nth(0).unwrap() {
        '0' => parse_operator_with_total_length_in_bits(version, type_id, &input[1..]),
        '1' => parse_operator_with_number_of_contained_packets(version, type_id, &input[1..]),
        _ => unreachable!(),
    }
}

fn parse_operator_with_total_length_in_bits(
    version: usize,
    type_id: usize,
    input: &str,
) -> (Packet, usize) {
    // println!(
    //     "parse_operator_with_total_length_in_bits({}, {}, {})",
    //     version, type_id, input
    // );
    let total_length_in_bits = parse_binary_as_decimal(&input[0..15]) + 15;
    // println!(
    //     "input: {} ({}), total_length_in_bits: {}",
    //     input, &input[0..15], total_length_in_bits
    // );
    let mut start = 15;
    let mut packets = vec![];
    while total_length_in_bits - start > 0 {
        // println!("Parsing at {}", start);
        let (packet, used_bits) = parse_binary(&input[start..]);
        packets.push(packet);
        start += used_bits;
        // println!(
        //     "total_length_in_bits ({}) - start ({}) = {}",
        //     total_length_in_bits,
        //     start,
        //     total_length_in_bits - start
        // );
    }
    (
        Packet::new_operator(version, type_id, packets),
        6 + 1 + total_length_in_bits,
    )
}

fn parse_operator_with_number_of_contained_packets(
    version: usize,
    type_id: usize,
    input: &str,
) -> (Packet, usize) {
    // println!(
    //     "parse_operator_with_number_of_contained_packets({}, {}, {})",
    //     version, type_id, input
    // );
    let number_of_contained_packets = parse_binary_as_decimal(&input[0..11]);
    // println!(
    //     "input: {} ({}), number_of_contained_packets: {}",
    //     input, &input[0..11], number_of_contained_packets
    // );
    let mut start = 11;
    let mut packets = vec![];
    for _ in 0..number_of_contained_packets {
        // println!("Parsing at {}", start);
        let (packet, used_bits) = parse_binary(&input[start..]);
        // println!("Pushing packet {:?} with used bits {}", packet, used_bits);
        packets.push(packet);
        start += used_bits;
    }
    (
        Packet::new_operator(version, type_id, packets),
        6 + 1 + start,
    )
}

fn parse_binary_as_decimal(input: &str) -> usize {
    usize::from_str_radix(input, 2).unwrap()
}

fn parse_literal_value_group(input: &str) -> (usize, bool) {
    (
        usize::from_str_radix(&input[1..5], 2).unwrap(),
        input.chars().nth(0).unwrap() == '1',
    )
}
