pub fn encode(n: u64) -> String {
    let mut output = String::new();
    if n == 0 {
        return String::from("zero");
    }
    if n < 1000 && n > 0 {
        return convert_1_to_999(n);
    }

    let a = (n / 1_000_000_000_000_000_000) % 1000;
    let b = (n / 1_000_000_000_000_000) % 1000;
    let c = (n / 1_000_000_000_000) % 1000;
    let d = (n / 1_000_000_000) % 1000;
    let e = (n / 1_000_000) % 1000;
    let f = (n / 1_000) % 1000;
    let g = n % 1000;

    for (i, ele) in [a, b, c, d, e, f, g].iter().enumerate() {
        if *ele != 0 {
            let the_string = format!(" {} {}", &convert_1_to_999(*ele), UNITS[i].1);
            output.push_str(&the_string);
        }
    }
    output.trim().to_string()
}

const ZERO_TO_NINETEEN: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const UNITS: [(u64, &str); 7] = [
    (1_000_000_000_000_000_000, "quintillion"),
    (1_000_000_000_000_000, "quadrillion"),
    (1_000_000_000_000, "trillion"),
    (1_000_000_000, "billion"),
    (1_000_000, "million"),
    (1000, "thousand"),
    (1, ""),
];

fn convert_tens(n: u64) -> String {
    match n / 10 {
        2 => return String::from("twenty"),
        3 => return String::from("thirty"),
        4 => return String::from("forty"),
        5 => return String::from("fifty"),
        6 => return String::from("sixty"),
        7 => return String::from("seventy"),
        8 => return String::from("eighty"),
        9 => return String::from("ninety"),
        _ => unreachable!(),
    }
}

fn convert_20_to_99(n: u64) -> String {
    if n % 10 == 0 {
        return convert_tens(n);
    } else {
        let tens = convert_tens(n);
        let ones = ZERO_TO_NINETEEN[(n % 10) as usize];
        return format!("{}-{}", tens, ones);
    }
}

fn convert_1_to_999(n: u64) -> String {
    let tens;
    if (0..20).contains(&(n % 100)) {
        tens = ZERO_TO_NINETEEN[(n % 100) as usize].to_string()
    } else {
        tens = convert_20_to_99(n % 100)
    }
    if n < 100 {
        return format!("{}", tens);
    } else if n % 100 == 0 {
        return format!("{} hundred", ZERO_TO_NINETEEN[(n / 100) as usize]);
    } else {
        return format!("{} hundred {}", ZERO_TO_NINETEEN[(n / 100) as usize], tens);
    }
}
