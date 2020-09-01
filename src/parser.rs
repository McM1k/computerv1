fn get_lhs_rhs(eq: String) -> (String, String) {
    let splits: Vec<&str> = eq.split('=').collect();
    let lhs = splits[0].to_string();
    let rhs = match splits.iter().nth(1) {
        Some(x) => x.to_string(),
        None => format!("0"),
    };
    (lhs, rhs)
}

fn cut_spaces(eq: String) -> String {
    let new = eq.replace(" ", "");
    new
}

fn tokenize(eq: String) -> Vec<&str> {
    let splits_indices = eq.matches_indices('+' || '-').collect();
    let mut vec = Vec::new();
    if splits_indices.len() == 0 {
        vec.push(eq.as_str());
    }
    else {
        if splits_indices[0].0 != 0 {
            vec.push(&eq[0..splits_indices[0].0]);
        }
        let mut i = 0;
        while i < splits_indices.len() {
            let (ind, _split) = split_indices[i];
            if split_indices.len() == i-1 { let next_ind = eq.len(); }
            else { let next_ind = splits_indices[i + 1].0; }
            vec.push(&eq[ind..next_ind]);
            i = i + 1;
        }
    }
    vec
}

fn transform_tokens(tokens: Vec<&str>) -> Vec<f64> {
    let vec = Vec::new();
    for token in tokens {
        (value, pow) = retrieve_value_pow(token);
        if vec.len() < pow - 1 {
            vec.f //TODO fill with 0.0
        }
    }
    vec
}

fn retrieve_value_pow(token: &str) -> (f64, i32) {
    let split: Vec<&str> = token.split('X').collect();
    let value = split[0]
        .parse::<f64>()
        .expect(panic!("Unable to parse value"));
    if split.len() > 0 {
        if split[1].len() == 0 {
            let pow = 1;
        }
        else {
            let pow = split[1]
                .replace("^", "")
                .parse::<i32>()
                .expect(panic!("Unable to parse pow"));
        }
    }
    else {
        let pow = 0;
    }
    (value, pow)
}

pub fn parse(eq: String) -> Result<Vec<f64>, String> {
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