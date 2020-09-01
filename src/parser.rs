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

fn tokenize(eq: String) -> Vec<String> {
    let splits_indices: (usize, &str) = eq.match_indices(|c| {c == '+' || c == '-'}).collect();
    let mut vec = Vec::new();
    if splits_indices.len() == 0 {
        vec.push(eq);
    }
    else {
        if splits_indices[0].0 != 0 {
            vec.push(eq[0..splits_indices[0].0].to_string());
        }
        let mut i = 0;
        while i < splits_indices.len() {
            let (ind, _split) = splits_indices[i];
            let next_ind;
            if splits_indices.len() == i-1 { next_ind = eq.len(); }
            else { next_ind = splits_indices[i + 1].0; }
            vec.push(eq[ind..next_ind].to_string());
            i = i + 1;
        }
    }
    vec
}

fn transform_tokens(tokens: Vec<String>) -> Vec<f64> {
    let mut vec = Vec::new();
    for token in tokens {
        let (value, pow) = retrieve_value_pow(token);
        while vec.len() < pow + 1 {
            vec.push(0.0);
        }
        vec[pow] = vec[pow] + value;
    }
    vec
}

fn retrieve_value_pow(token: String) -> (f64, usize) {
    let split: Vec<&str> = token.split('X').collect();
    let value = split[0]
        .parse::<f64>()
        .unwrap_or_else(|_|panic!("Unable to parse value"));
    let pow;
    if split.len() > 0 {
        if split[1].len() == 0 { pow = 1; }
        else {
            pow = split[1]
                .replace("^", "")
                .parse::<usize>()
                .unwrap_or_else(|_|panic!("Unable to parse pow"));
        }
    }
    else { pow = 0; }
    (value, pow)
}

fn reduce(lhs: Vec<f64>, rhs: Vec<f64>) -> Vec<f64> {
    let mut i = 0;
    let mut pol = Vec::new();
    while lhs.len() > i || rhs.len() > i {
        if lhs.len() <= i { pol.push(- rhs[i]); }
        else if rhs.len() <= i { pol.push(lhs[i]); }
        else { pol.push(lhs[i] - rhs[i]); }
        i = i + 1;
    }
    while pol.last().is_some() && (pol[pol.len() - 1] == 0.0) { pol.pop(); }
    pol
}

pub fn parse(eq: String) -> Vec<f64> {
    let eq = cut_spaces(eq);
    let (lhs, rhs) = get_lhs_rhs(eq);
    let (lhs, rhs) = (
        transform_tokens(tokenize(lhs)),
        transform_tokens(tokenize(rhs))
    );
    reduce(lhs, rhs)
}