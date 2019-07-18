pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string()
    }
    let names = vec![
        "",
        "thousand",
        "million",
        "billion",
        "trillion",
        "quadrillion",
        "quintillion",
        "sextillion",
    ];
    
    let mut string = String::new();
    let mut n = n;
    
    for name in names {
        if n == 0 {
            break
        }
        let chunk = n % 1000;
        if chunk > 0 {
            string = format!("{} {} {}", hundreds(chunk), name, string);
        }
        n = n / 1000
    }
    string.trim_end().to_string()
}

fn single(n: u64) -> &'static str {
    match n {
        0 => "",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        n => panic!("Single not 0-9, was {}", n),
    }
}

fn double(n: u64) -> String {
    if n < 10 {
        single(n).to_string()
    } else if n < 20 {
        match n {
            10 => "ten",
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            19 => "nineteen",
            _ => panic!(""),
        }
        .to_string()
    } else {
        let tens = match n / 10 {
            2 => "twenty",
            3 => "thirty",
            4 => "forty",
            5 => "fifty",
            6 => "sixty",
            7 => "seventy",
            8 => "eighty",
            9 => "ninety",
            _ => panic!(),
        };
        if (n % 10) == 0 {
            tens.to_string()
        } else {
            format!("{}-{}", tens, single(n % 10))
        }
    }
}

fn hundreds(n: u64) -> String {
    println!("{}", n);
    if n < 100 {
        double(n)
    } else {
        format!("{} hundred {}", single(n / 100), double(n % 100)).trim_end().to_string()
    }
}
