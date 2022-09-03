use std::io;

fn main() {
    let prefix_over_9 = (
        ("un", "duo", "tre", "quattuor", "quinqua", "se", "septe", "octo", "nove"),
        ("deci", "viginti", "triginta", "quadraginta", "quinquaginta", "sexaginta", "septuaginta", "octoginta", "nonaginta"),
        ("cent", "ducent", "trecent", "quadringent", "quingent", "sescent", "septingenti", "octingent", "nongent"),
    );
    let prefix_under_10 = ("m", "b", "tr", "quadr", "quint", "sext", "sept", "oct", "non");
    let modifiers = (
        (
            ("", "n"),
            ("s", "m"),
            ("s", "n"),
            ("s", "n"),
            ("s", "n"),
            ("", "n"),
            ("", "n"),
            ("x", "m"),
            ("", ""),
        ),
        (
            ("x", "n"),
            ("", "n"),
            ("s", "n"),
            ("s", "n"),
            ("s", "n"),
            ("", "n"),
            ("", "n"),
            ("x", "m"),
            ("", ""),
        ),
    );

    let input_number: u128 = read_input();
    let sector_count: u128 = sector_count(input_number); //Sector count is the amount of 3 number groups in the number eg. 1000000 would be 3 (1.000.000)

}

fn read_input() -> u128 {
    'read_input: loop {
        println!("Please input the number you want to read:");

        let mut input_number = String::new();

        match io::stdin()
            .read_line(&mut input_number) {
                Ok(_) => {
                    let input_number: u128 = match input_number.trim().parse() {
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

fn sector_count(input_number: u128) -> u128 {
    let len = number_length(input_number);
    (len + 2)/3
}

fn number_length(input_number: u128) -> u128 {
    let mut power = 10;
    let mut count = 1;
    while input_number >= power {
        count += 1;
        if let Some(new_power) = power.checked_mul(10) {
            power = new_power;
        } else {
            break;
        }
    }
    count
}