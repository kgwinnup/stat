use std::collections::HashMap;
use std::io::prelude::*;
use std::path::PathBuf;

use std::str::FromStr;

pub fn median(input: &mut [f64]) -> f64 {
    if input.len() == 0 {
        return 0.0;
    }

    if input.len() == 1 {
        return input[0];
    }

    if input.len() == 2 {
        return input[0] + input[1] / 2.0;
    }

    input.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = input.len() / 2;
    input[mid]
}

pub fn mode(input: &[f64], prec: u32) -> f64 {
    let mut counts: HashMap<u32, u32> = HashMap::new();

    for val in input.iter() {
        let temp = val * (prec as f64);
        *counts.entry(temp as u32).or_insert(1) += 1;
    }

    let out = counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("cannot compute mode of zero numbers");

    out as f64 / prec as f64
}

pub fn mean(input: &[f64]) -> f64 {
    if input.len() == 0 {
        return 0.0;
    }

    if input.len() == 1 {
        return input[0];
    }

    input.iter().sum::<f64>() as f64 / input.len() as f64
}

/// (stdev, var, mean)
pub fn stdev_var_mean(input: &[f64]) -> (f64, f64, f64) {
    if input.len() == 0 {
        return (0.0, 0.0, 0.0);
    }

    if input.len() == 1 {
        return (0.0, 0.0, 0.0);
    }

    let u = mean(input);

    let mut sum = 0.0;

    for i in input.iter() {
        sum += (i - u).powf(2.0);
    }

    (
        (sum / input.len() as f64).sqrt(),
        (sum / input.len() as f64),
        u,
    )
}

pub fn confusion_matrix(tuples: &Vec<(f32, f32)>, threshold: Option<f32>) -> Vec<Vec<u32>> {
    let mut classes = HashMap::new();

    for (_, c) in tuples {
        // f32 is not hashable, convert to string
        classes.insert(format!("{}", c), 1);
    }

    let size = classes.keys().len();
    let mut matrix = vec![];

    // create the confusion matrix
    for _ in 0..size {
        let mut row = vec![];
        for _ in 0..size {
            row.push(0);
        }
        matrix.push(row);
    }

    // intended to use only with binary (0,1) ranges. Not softprob (yet).
    for (p, actual_class_col) in tuples {
        let predicted_class_row = if let Some(t) = threshold {
            (*p + (1.0 - t)) as usize
        } else if size == 2 {
            (*p + 0.5) as usize
        } else {
            *p as usize
        };

        matrix[predicted_class_row][*actual_class_col as usize] += 1;
    }

    matrix
}

pub struct CMatrixStats {
    pub label: usize,
    pub fpr: f32,
    pub tpr: f32,
    pub fnr: f32,
    pub tnr: f32,
}

pub fn confusion_matrix_stats(matrix: &Vec<Vec<u32>>) -> Vec<CMatrixStats> {
    let size = matrix.len();

    let mut counts = HashMap::new();
    for i in 0..size {
        // TP FN FP TN
        counts.insert(i, vec![0.0, 0.0, 0.0, 0.0]);
    }

    let mut total = 0.0;

    for i in 0..size {
        let mut fn_i = 0.0;

        for j in 0..size {
            // total the entire matrix
            total += matrix[i][j] as f32;

            // TP
            if i == j {
                let data = counts.get_mut(&i).unwrap();
                data[0] = matrix[i][j] as f32;
                continue;
            }

            let data = counts.get_mut(&i).unwrap();
            // FP
            data[2] += matrix[i][j] as f32;

            // FN
            fn_i += matrix[i][j] as f32;
        }

        let data = counts.get_mut(&i).unwrap();
        // FN
        data[1] = fn_i;
    }

    let mut stats = vec![];

    for k in 0..size {
        let v = counts.get_mut(&k).unwrap();
        // TN
        v[3] = total - v[0] - v[1] - v[2];

        stats.push(CMatrixStats {
            label: k,
            fpr: v[2] / (v[2] + v[3]),
            tpr: v[0] / (v[0] + v[1]),
            fnr: v[1] / (v[1] + v[0]),
            tnr: v[3] / (v[3] + v[1]),
        })
    }

    stats.sort_by(|a, b| a.label.cmp(&b.label));

    stats
}

pub fn threshold_table_stats(tuples: &Vec<(f32, f32)>) -> Vec<Vec<f32>> {
    let mut out = vec![];
    let mut t = 0.05;

    loop {
        if t > 1.0 {
            break;
        }

        let mut ttp: f32 = 0.0;
        let mut tfp: f32 = 0.0;
        let mut tfn: f32 = 0.0;
        let mut ttn: f32 = 0.0;

        for (p, a) in tuples {
            if *p >= t && *a == 1.0 {
                ttp += 1.0;
                continue;
            }

            if *p >= t && *a == 0.0 {
                tfp += 1.0;
                continue;
            }

            if *p < t && *a == 1.0 {
                tfn += 1.0;
                continue;
            }

            if *p < t && *a == 0.0 {
                ttn += 1.0;
                continue;
            }
        }

        let precision = ttp / (ttp + tfp);
        let recall = ttp / (ttp + tfn);
        let f1 = 2.0 * (recall * precision) / (recall + precision);
        let fpr = tfp / (tfp + ttn);

        out.push(vec![t, precision, recall, f1, fpr]);

        t += 0.05;
    }

    out
}

/// get_input will check if the input parameter is_some, and if so read input from a file, else,
/// read input from stdin. A new String is returned with the contents.
pub fn get_input(input: Option<PathBuf>) -> String {
    if let Some(path) = input {
        match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => {
                eprintln!("failed to read input file");
                std::process::exit(1);
            }
        }
    } else {
        let mut input = String::new();
        let stdin = std::io::stdin();

        for line in stdin.lock().lines() {
            let buf = line.expect("failed to read line");
            input.push_str(&buf);
            input.push('\n');
        }

        input
    }
}

pub fn get_input_bytes(input: Option<PathBuf>) -> Vec<u8> {
    if let Some(path) = input {
        match std::fs::read(path) {
            Ok(bs) => bs,
            Err(_) => {
                eprintln!("failed to read input file");
                std::process::exit(1);
            }
        }
    } else {
        let mut stdin = std::io::stdin();
        let mut buf = vec![];
        let _ = stdin.read_to_end(&mut buf);
        buf
    }
}

pub fn entropy(bytes: &[u8]) -> f64 {
    let mut histo: Vec<f64> = Vec::with_capacity(256);

    for _ in 0..256 {
        histo.push(0.0)
    }

    bytes.iter().for_each(|b| {
        histo[*b as usize] += 1.0;
    });

    let mut out = 0.0;
    let len = bytes.len() as f64;

    for n in histo {
        if n == 0.0 {
            continue;
        }

        let freq = (n as f64) / len;
        out -= freq * freq.log(2.0);
    }

    out
}

/// calculates and normalizes the byte histogram
pub fn to_byte_histogram(bytes: &[u8]) -> Vec<f64> {
    let mut histo: Vec<f64> = Vec::with_capacity(256);

    for _ in 0..256 {
        histo.push(0.0)
    }

    bytes.iter().for_each(|b| {
        histo[*b as usize] += 1.0;
    });

    let sum = bytes.len() as f64;

    histo.iter().map(|x| x / sum).collect()
}

pub fn str_to_vector<F: FromStr>(s: &str, sep: &str) -> Result<Vec<F>, <F as FromStr>::Err> {
    let mut out = vec![];
    for i in s.split(sep).into_iter() {
        match i.trim().parse() {
            Ok(f) => {
                out.push(f);
            }

            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(out)
}

pub fn to_tuple(input: &str) -> Vec<(f32, f32)> {
    let mut data = vec![];

    for line in input.split("\n") {
        if line == "\n" || line == "" {
            continue;
        }

        let cols = line.split(",").collect::<Vec<&str>>();
        if cols.len() != 2 {
            eprintln!("warning: invalid column count for parse_tuple: {}", line);
            continue;
        }

        let temp: Result<f32, _> = cols[0].trim().parse();
        let v1 = match temp {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };

        let temp: Result<f32, _> = cols[1].trim().parse();
        let v2 = match temp {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };

        data.push((v1, v2));
    }

    data
}

pub fn to_vector(raw_inputs: &str, with_header: bool) -> Vec<f64> {
    let mut data = vec![];

    for (index, line) in raw_inputs.split("\n").enumerate() {
        if index == 0 && with_header {
            continue;
        }

        if line == "\n" || line == "" {
            continue;
        }

        let temp: Result<f64, _> = line.trim().parse();
        match temp {
            Ok(f) => data.push(f),
            Err(_) => {
                eprintln!("error converting to float: {} at line {}", line, index);
                std::process::exit(1);
            }
        }
    }

    data
}

/// to_matrix parses a input and builds a DMatrix for use with XGBoost. If 'ycol' is a valid column
/// index, that column will be held out and used as the labels for the DMatrix.
/// One odd piece of this function is that it returns a tuple instead of just the DMatrix. This
/// second value in the tuple is a vector of lines from the raw input. This is used as the output
/// of the prediction subcommand. I couldn't think of a better way a the time to do this
/// efficiently.
pub fn to_matrix(raw_inputs: &str, ycol: usize, with_header: bool) -> (Vec<Vec<f64>>, Vec<f32>) {
    let mut xdata = Vec::new();
    let mut ydata = Vec::new();

    for (index, line) in raw_inputs.split("\n").enumerate() {
        if index == 0 && with_header {
            continue;
        }

        if line == "\n" || line == "" {
            continue;
        }

        let split = line.split(",");

        let mut row = vec![];

        for (index, val) in split.enumerate() {
            let temp: Result<f64, _> = val.trim().parse();
            match temp {
                Ok(f) => {
                    if index == ycol {
                        ydata.push(f as f32)
                    } else {
                        row.push(f)
                    }
                }
                Err(_) => {
                    eprintln!("error converting to float: {} at line {}", line, index);
                    std::process::exit(1);
                }
            }
        }

        xdata.push(row);
    }

    (xdata, ydata)
}
