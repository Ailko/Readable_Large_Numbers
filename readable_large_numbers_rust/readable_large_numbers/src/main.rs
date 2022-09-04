#![deny(missing_docs)]
#![deny(rustdoc::missing_doc_code_examples)]

//! This is a project that converts numbers into their text representation to allow for more easily reading big numbers aloud.

use std::io;
use num_bigint::BigUint;

const PREFIX_OVER_9: [[&str; 10]; 3] = [
    ["", "un", "duo", "tre", "quattuor", "quinqua", "se", "septe", "octo", "nove"],
    ["", "deci", "viginti", "triginta", "quadraginta", "quinquaginta", "sexaginta", "septuaginta", "octoginta", "nonaginta"],
    ["", "cent", "ducent", "trecent", "quadringent", "quingent", "sescent", "septingenti", "octingent", "nongent"],
];

const PREFIX_UNDER_10: [&str; 10] = ["", "m", "b", "tr", "quadr", "quint", "sext", "sept", "oct", "non"];

const MODIFIERS: [[[&str; 2]; 10]; 2] = [
    [
        ["", ""],
        ["", "n"],
        ["s", "m"],
        ["s", "n"],
        ["s", "n"],
        ["s", "n"],
        ["", "n"],
        ["", "n"],
        ["x", "m"],
        ["", ""],
    ],
    [
        ["", ""],
        ["x", "n"],
        ["", "n"],
        ["s", "n"],
        ["s", "n"],
        ["s", "n"],
        ["", "n"],
        ["", "n"],
        ["x", "m"],
        ["", ""],
    ],
];

fn main() {
    let input_number: BigUint = read_input();
    let mut number: BigUint = input_number.clone();
    let mut sector_count: u32 = sector_count(input_number); //Sector count is the amount of 3 number groups in the number eg. 1000000 would be 3 (1.000.000)

    while sector_count > 2 {
        if &number / BigUint::from(10_u8).pow((sector_count - 1)*3) != BigUint::from(0_u8) {
            let prefix = format!("{}", &match sector_count - 2{
                0..=9 => prefix_under_10_generator(sector_count - 2),
                10..=99 => prefix_under_100_generator(sector_count - 2),
                100..=999 => prefix_under_1000_generator(sector_count - 2),
                _ => prefix_over_999_generator(sector_count - 2),
            });

            println!("{}{}illion", hundreds(biguint_to_u32(&number / BigUint::from(10_u8).pow((sector_count - 1) * 3))), prefix);
            number -= &number / BigUint::from(10_u8).pow((sector_count - 1) * 3) * BigUint::from(10_u8).pow((sector_count - 1) * 3);
        }
        sector_count -= 1;
    }

    let mut number = biguint_to_u32(number);

    if number / 1000 > 0 {
        println!("{}", prefix_thousands(number / 1000));
        number -= number / 1000 * 1000;
    }

    if number > 0 {
        println!("{}", hundreds(number));
    }
}

fn biguint_to_u32(number: BigUint) -> u32 {
    let mut temp_vec = number.to_bytes_be();
    let mut be_arr = [0; 4];
    let mut counter = 4;
    
    while temp_vec.len() > 0 {
        counter -= 1;
        be_arr[counter] = temp_vec.pop().unwrap();
    }

    u32::from_be_bytes(be_arr)
}

fn read_input() -> BigUint {
    'read_input: loop {
        println!("Please input the number you want to read:");

        let mut input_number = String::new();

        match io::stdin()
            .read_line(&mut input_number) {
                Ok(_) => {
                    let input_number: BigUint = match input_number.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Please enter a number.");
                            continue 'read_input
                        }
                    };
                    break 'read_input input_number
                },
                Err(_) => {
                    println!("Something went wrong, please try again.");
                    continue 'read_input
                }
            }
    }
}

fn sector_count(input_number: BigUint) -> u32 {
    let len = number_length(input_number);
    (len + 2)/3
}

fn number_length(input_number: BigUint) -> u32 {
    let mut power = BigUint::from(10_u32);
    let mut count: u32 = 1;
    while input_number >= power {
        count += 1;
        power = power * 10_u8;
    }
    count
}

fn irregularity_generator(units: u32, modifier_type: i8) -> String {
    format!("{}{}",
        PREFIX_OVER_9[0][units as usize],
        match units {
            3|6 => &MODIFIERS[modifier_type as usize][units as usize][0],
            7|9 => &MODIFIERS[modifier_type as usize][units as usize][1],
            _ => "",
        }
    )
}

fn units(units: u32) -> String {
    String::from(match units {
        1 => "one ",
        2 => "two ",
        3 => "three ",
        4 => "four ",
        5 => "five ",
        6 => "six ",
        7 => "seven ",
        8 => "eight ",
        9 => "nine ",
        _ => "",
    })
}

fn tens(tens: u32) -> String {
    match tens {
        0..=9 => units(tens),
        10..=19 => {
            String::from(match tens {
                10 => "ten ",
                11 => "eleven ",
                12 => "twelve ",
                13 => "thirteen ",
                14 => "fourteen ",
                15 => "fifteen ",
                16 => "sixteen ",
                17 => "seventeen ",
                18 => "eighteen ",
                19 => "nineteen ",
                _ => "",
            })
        },
        _ => {
            let mut result = String::new();
            result.push_str(&match tens {
                20..=29 => "twenty ",
                30..=39 => "thirty ",
                40..=49 => "fourty ",
                50..=59 => "fifty ",
                60..=69 => "sixty ",
                70..=79 => "seventy ",
                80..=89 => "eighty ",
                90..=99 => "ninety ",
                _ => "",
            });
            result.push_str(&units(tens - (tens/10)*10));
            result
        }
    }
}

fn hundreds(hundreds: u32) -> String {
    let mut result = String::new();
    if hundreds / 100 > 0 {
        result.push_str(&units(hundreds / 100));
        result.push_str("hundred ");
    }
    let tens = tens(hundreds - hundreds / 100 * 100);
    if tens.len() > 0 {
        result.push_str("and ");
        result.push_str(&tens);
    }
    result
}

fn prefix_thousands(thousands: u32) -> String {
    if thousands > 0 {
        format!("{}thousand ",
            hundreds(thousands)
        )
    } else {
        String::from("")
    }
}

fn prefix_under_10_generator(sector_count: u32) -> String {
    String::from(PREFIX_UNDER_10[(sector_count) as usize])
}

fn prefix_under_100_generator(sector_count: u32) -> String {
    let sector_tens = sector_count / 10;
    let sector_units = sector_count - sector_tens * 10;

    format!("{}{}",
        irregularity_generator(sector_units, 1),
        &PREFIX_OVER_9
            [1]
            [sector_tens as usize]
            [..(match PREFIX_OVER_9
                [1]
                [sector_tens as usize]
                .len()
                .checked_sub(1) {
                    None => 0,
                    Some(num) => num,
                }
            )]
    )
}

fn prefix_under_1000_generator(sector_count: u32) -> String {
    let sector_hundreds = sector_count / 100;
    let sector_tens = sector_count / 10 - sector_hundreds * 10;
    let sector_units = sector_count - sector_tens * 10 - sector_hundreds * 100;

    format!("{}{}{}",
        prefix_under_100_generator(sector_tens*10 + sector_units),
        &PREFIX_OVER_9
            [1]
            [sector_tens as usize]
            [(match PREFIX_OVER_9
                [1]
                [sector_tens as usize]
                .len()
                .checked_sub(1) {
                    None => 0,
                    Some(num) => num,
                }
            )..],
        &PREFIX_OVER_9[2][sector_hundreds as usize]
    )
}

fn prefix_over_999_generator(sector_count: u32) -> String {
    let mut result = String::new();
    let sector_thousands = sector_count/1000;
    let rest = sector_count - sector_thousands*1000;

    result.push_str(&prefix_under_1000_generator(rest));
    if rest != 0 {
        result.push_str("illion");
    }
    if sector_thousands != 0 {
        result.push_str(&prefix_over_999_generator(sector_thousands));
    }
    result.push_str("illin");

    result
}