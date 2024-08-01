use std::env;

fn calculate_pin_from_name(pin_name: &str) -> u32 {
    let bank = pin_name[4..5].parse::<u32>().unwrap();
    let group = (pin_name.chars().nth(6).unwrap() as u8 - 'A' as u8) as u32;
    let x = pin_name[7..8].parse::<u32>().unwrap();

    let number = group * 8 + x;
    println!("GPIO_NUM: {}", number);
    let pin = bank * 32 + number;
    pin
}

fn calculate_bank_group_x(pin: u32) -> (u32, u32, u32) {
    let bank = pin / 32;
    let number = pin % 32;
    let group = number / 8;
    let x = number % 8;
    (bank, group, x)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} -p <pin_name> | -n <pin_number>", args[0]);
        return;
    }

    match args[1].as_str() {
        "-p" | "--pin" => {
            let pin_name = &args[2];
            let pin = calculate_pin_from_name(pin_name);
            println!("Pin name {} corresponds to pin number: {}", pin_name, pin);
        }
        "-n" | "--number" => {
            let pin_number = args[2].parse::<u32>().unwrap();
            let (bank, group, x) = calculate_bank_group_x(pin_number);
            println!(
                "Pin number {} corresponds to: [GPIO{}_{}{}]",
                pin_number,
                bank,
                (group as u8 + 'A' as u8) as char,
                x
            );
        }
        _ => {
            println!("Invalid option. Use -p <pin_name> or -n <pin_number>.");
        }
    }
}
