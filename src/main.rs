mod parser;

use std::env;

fn ft_abs(x: f64) -> f64 {
    if 0.0 > x { return -x; }
    x
}

fn ft_sqrt(x: f64) -> f64 {
    let mut res = x;
    for _i in 0..500 {
        res = (res + (x / res)) / 2.0;
    }
    res
}

fn ft_disc(a: f64, b: f64, c: f64) -> f64 {
    let d = b * b - 4.0 * a * c;
    println!("delta = {} ^ 2 - 4 * {} * {} = {}",b ,a ,c, d);
    d
}

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
            let d = ft_disc(pol[2], pol[1], pol[0]);
            if d < 0.0 {
                let r = ft_sqrt(ft_abs(d));
                let y = - pol[1] / (2.0 * pol[2]);
                let x1 = - r / (2.0 * pol[2]);
                let x2 = r / (2.0 * pol[2]);
                let sign;
                if y < 0.0 {
                    sign = "-";
                } else {
                    sign = "+";
                }
                let z1 = format!("{}i {} {}", x1, sign, ft_abs(y));
                let z2 = format!("{}i {} {}", x2, sign, ft_abs(y));
                println!("Discriminant is strictly negative, z1 = {}, z2 = {}", z1, z2);
            } else if d == 0.0 {
                let x0 = -pol[1] / (2.0 * pol[2]);
                println!("Discriminant is equal to 0, the solution is : {}", x0);
            } else {
                let x1 = (-pol[1] - ft_sqrt(d)) / (2.0 * pol[2]);
                let x2 = (-pol[1] + ft_sqrt(d)) / (2.0 * pol[2]);
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

#[cfg(test)]
mod main_tests {
    mod ft_abs {
        use super::super::ft_abs;

        #[test]
        fn x_pos() {
            let x = 1.0;
            assert_eq!(ft_abs(x), 1.0);
        }

        #[test]
        fn x_neg() {
            let x = -1.0;
            assert_eq!(ft_abs(x), 1.0);
        }
    }

    mod ft_sqrt {
        use super::super::ft_sqrt;

        #[test]
        fn sqrt_one() {
            let x = 1.0;
            assert_eq!(ft_sqrt(x), 1.0);
        }

        #[test]
        fn sqrt_four() {
            let x = 4.0;
            let res = ft_sqrt(x);
            println!("{}", res);
            assert!(res > 1.99 && res < 2.01 && res == 2.0);
        }
    }
}