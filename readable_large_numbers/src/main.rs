use std::io;
use num_bigint::BigUint;

const prefix_over_9_units: [&str; 9] = ["un", "duo", "tre", "quattuor", "quinqua", "se", "septe", "octo", "nove"];
const prefix_over_9_tens: [&str; 9] = ["deci", "viginti", "triginta", "quadraginta", "quinquaginta", "sexaginta", "septuaginta", "octoginta", "nonaginta"];
const prefix_over_9_hundreths: [&str; 9] = ["cent", "ducent", "trecent", "quadringent", "quingent", "sescent", "septingenti", "octingent", "nongent"];

const prefix_under_10: [&str; 9] = ["m", "b", "tr", "quadr", "quint", "sext", "sept", "oct", "non"];

const modifiers: [[[&str; 2]; 9]; 2] = [
    [
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
            let mut prefix = format!("{}", match sector_count - 2{
                0..=9 => prefix_under_10_generator(sector_count - 2),
                10..=99 => Over100(sectorcount - 2);
            });
        }
    }
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

fn prefix_under_10_generator(sector_count: u32) -> &'static str{
    prefix_under_10[(sector_count - 1) as usize]
}