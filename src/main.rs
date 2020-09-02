mod parser;

use std::env;

fn print_solution(pol: Vec<f64>) {
    let degree = pol.len();
    match degree {
        0 => { println!("The solution is R"); },
        1 => { println!("The solution is not really {}, but you gave me an equation that is ultimately wrong, so...", pol[0]); },
        2 => {
            let x = -pol[0] / pol[1];
            println!("The solution is : {}", x);
        },
        3 => {
            let d = pol[1] * pol[1] - 4.0 * pol[2] * pol[0];
            if d < 0.0 {
                let r = d.abs().sqrt();
                let y = - pol[1] / 2.0 * pol[2];
                let x1 = - r / 2.0 * pol[2];
                let x2 = r / 2.0 * pol[2];
                let sign;
                if y < 0.0 {
                    sign = "-";
                } else {
                    sign = "+";
                }
                let z1 = format!("{}i {} {}", x1, sign, y.abs());
                let z2 = format!("{}i {} {}", x2, sign, y.abs());
                println!("Discriminant is strictly negative, z1 = {}, z2 = {}", z1, z2);
            } else if d == 0.0 {
                let x0 = -pol[1] / (2.0 * pol[2]);
                println!("Discriminant is equal to 0, the solution is : {}", x0);
            } else {
                let x1 = (-pol[1] - d.sqrt()) / 2.0 * pol[2];
                let x2 = (-pol[1] + d.sqrt()) / 2.0 * pol[2];
                println!(
                    "Discriminant is strictly positive. x1 = {}, x2 = {}",
                    x1, x2
                );
            }
        },
        _ => {
            println!("The degree is strictly greater than 2, can't do that");
        },
    }
}

fn print_degree(pol: Vec<f64>) {
    let degree = match pol.len() {
        0 => 0,
        _ => pol.len() - 1,
    };
    println!("Polynomial degree : {}", degree);
}

fn print_reduced(pol: Vec<f64>) {
    let mut str = String::new();
    if pol.is_empty() { str.push_str("0 "); }
    else {
        for (pow, &value) in pol.iter().enumerate() {
            if value != 0.0 {
                if !(str.is_empty() && value > 0.0) {
                    if value < 0.0 { str.push_str("- "); } else { str.push_str("+ "); }
                }
                if pow == 0 { str.push_str(format!("{} ", value.abs()).as_str()); } else {
                    if value.abs() != 1.0 { str.push_str(format!("{}", value.abs()).as_str()); }
                    str.push_str("X");
                    if pow > 1 { str.push_str(format!("^{}", pow).as_str()); }
                    str.push_str(" ");
                }
            }
        }
    }
    println!("Reduced form : {}= 0", str);
}


fn main() {
    let eq = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("Can't get the first parameter"));
    let pol = parser::parse(eq);
    print_reduced(pol.clone());
    print_degree(pol.clone());
    print_solution(pol);
}
