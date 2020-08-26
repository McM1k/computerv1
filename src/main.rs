use std::env;

/*
** of course it would be smarter to use a vector
** but the goal was to code this in the shortest amount of time possible
*/
#[derive(Clone, Copy)]
pub struct Pol {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

fn print_solution(degree: i32, pol: Pol) {
    if degree == 0 {
        if pol.c == 0.0 {
            println!("The solution is R");
        } else {
            println!("The solution is : {}", pol.c);
        }
    } else if degree == 1 {
        let x = -pol.c / pol.b;
        println!("The solution is : {}", x);
    } else {
        let d = pol.b * pol.b - 4.0 * pol.a * pol.c;
        if d < 0.0 {
            println!("Discriminant is strictly negative, there is no solution");
        } else if d == 0.0 {
            let x0 = -pol.b / (2.0 * pol.a);
            println!("Discriminant is equal to 0, the solution is : {}", x0);
        } else {
            let x1 = (-pol.b - d.sqrt()) / 2.0 * pol.a;
            let x2 = (-pol.b + d.sqrt()) / 2.0 * pol.a;
            println!(
                "Discriminant is strictly positive. x1 = {}, x2 = {}",
                x1, x2
            );
        }
    }
}

fn reduce(lhs: Pol, rhs: Pol) -> Pol {
    Pol {
        a: lhs.a - rhs.a,
        b: lhs.b - rhs.b,
        c: lhs.c - rhs.c,
    }
}

fn print_degree(pol: Pol) -> i32 {
    let mut degree = 0;
    if pol.c != 0.0 {
        degree = 0
    }
    if pol.b != 0.0 {
        degree = 1
    }
    if pol.a != 0.0 {
        degree = 2
    }
    println!("Polynomial degree : {}", degree);
    degree
}

fn print_reduced(pol: Pol) {
    let mut format = "".to_string();
    if pol.c != 0.0 {
        if pol.c < 0.0 {
            format.push_str("- ")
        }
        format.push_str(format!("{} * X^0 ", pol.c.abs()).as_str());
    }
    if pol.b != 0.0 {
        if pol.b < 0.0 {
            format.push_str("- ")
        } else if pol.c != 0.0 {
            format.push_str("+ ")
        }
        format.push_str(format!("{} * X^1 ", pol.b.abs()).as_str());
    }
    if pol.a != 0.0 {
        if pol.a < 0.0 {
            format.push_str("- ")
        } else if pol.c != 0.0 || pol.b != 0.0 {
            format.push_str("+ ")
        }
        format.push_str(format!("{} * X^2 ", pol.a.abs()).as_str());
    }
    if pol.a == 0.0 && pol.b == 0.0 && pol.c == 0.0 {
        format.push_str("0 ")
    }
    println!("Reduced form : {}= 0", format);
}

fn parse(eq: String) -> Result<Pol, String> {
    let splits: Vec<&str> = eq.split(' ').collect();
    let mut tmp = 1.0;
    let mut rhs = Pol {
        a: 0.0,
        b: 0.0,
        c: 0.0,
    };
    let mut lhs = rhs;
    for split in splits {
        match split.chars().nth(0).unwrap() {
            '-' => {
                if split == "-" {
                    tmp = -1.0;
                }
            }
            '+' => (),
            '*' => (),
            '0'..='9' => {
                tmp = match split.parse::<f64>() {
                    Ok(x) => tmp * x,
                    Err(_) => {
                        return Err(format!("Cannot parse float : \"{}\"", split));
                    }
                };
            }
            'X' => {
                match split.chars().nth(2).unwrap() {
                    '2' => {
                        rhs.a = tmp;
                    }
                    '1' => {
                        rhs.b = tmp;
                    }
                    '0' => {
                        rhs.c = tmp;
                    }
                    _ => {
                        return Err(format!("The polynomial degree is stricly greater than 2, I can't solve : \"{}\"", split));
                    }
                }
                tmp = 1.0;
            }
            '=' => {
                lhs = rhs;
                rhs.a = 0.0;
                rhs.b = 0.0;
                rhs.c = 0.0;
            }
            _ => {
                return Err(format!("wrong token : \"{}\"", split));
            }
        }
    }
    Ok(reduce(lhs, rhs))
}

fn main() {
    let eq = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("Can't get the first parameter"));
    let pol = match parse(eq) {
        Ok(x) => x,
        Err(e) => panic!(e),
    };
    print_reduced(pol);
    let degree = print_degree(pol);
    print_solution(degree, pol);
}
