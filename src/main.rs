fn get_arg_number(nth: usize) -> i16 {
    let slot = if std::env::args().len() > nth {
        std::env::args().nth(nth).expect("0")
    } else {
        String::from("0")
    };

    match slot.parse::<i16>()
    {
        Ok(x) => x,
        _ => {
            println!("Argument {} is not a number.", nth);
            0
        }
    }
}

fn get_arg(nth: usize) -> String {
    if std::env::args().len() > nth {
        std::env::args().nth(nth).expect("0")
    } else {
        String::from("+")
    }
}

fn main() {
    let slot_a = get_arg_number(1);
    let expr = get_arg(2);
    let slot_b = get_arg_number(3);

    let result = match expr.as_ref() {
        "+" => (slot_a + slot_b).to_string(),
        "-" => (slot_a - slot_b).to_string(),
        "*" => (slot_a * slot_b).to_string(),
        "/" => (slot_a / slot_b).to_string(),
        "%" => (slot_a % slot_b).to_string(),
        _ => String::from("undefined")
    };

    println!("{} {} {} = {}", slot_a, expr, slot_b, result);
}
