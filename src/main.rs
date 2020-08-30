use std::env;

fn print_solution(degree: i32, pol: Vec<f64>) {
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

fn print_degree(pol: Vec<f64>) -> i32 {
    println!("Polynomial degree : {}", pol.len());
    degree
}

fn print_reduced(pol: Vec<f64>) {
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

fn parse(eq: String) -> Result<Vec<f64>, String> {
    let splits: Vec<&str> = eq.split(' ').collect();
    let mut rhs = false;
    let mut tmp = 1.0;
    let mut pol = vec![0.0];
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
                    Err(_) => { return Err(format!("Cannot parse float : \"{}\"", split)); }
                };
            }
            'X' => {
                let pow = match split[2..].parse::<usize>() {
                    Ok(x) => x,
                    Err(_) => { return Err(format!("Cannot parse power : \"{}\"", split)); }
                };
                while pow < pol.len() {

                }
                pol[pow] = tmp;
                tmp = 1.0;
            }
            '=' => { rhs = true; }
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
