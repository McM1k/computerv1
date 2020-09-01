mod parser;

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


fn main() {
    let eq = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("Can't get the first parameter"));
    let pol = match parser::parse(eq) {
        Ok(x) => x,
        Err(e) => panic!(e),
    };
    print_reduced(pol.copy());
    let degree = print_degree(pol.copy());
    print_solution(degree, pol);
}
