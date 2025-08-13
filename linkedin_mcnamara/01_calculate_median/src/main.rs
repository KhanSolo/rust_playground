// calculate median for a vec of f32
use std::cmp::Ordering;

fn main() {
    let mut vec:Vec<f32> = vec![];
    let option = median(&mut vec);
    let msg = match option {
        Some(m) => format!("Median: {m}"),
        None => "Median doesn't exist".into(),
    };
    println!("{msg}");
}

fn median(vec: &mut Vec<f32>) -> Option<f32> {
    match vec.len() {
        0 => None,
        n if n%2==0 => {
            let m = get_m(vec);
            Some(vec[m-1] + (vec[m]-vec[m-1])/2.0)
        },
        _ => {
            let m = get_m(vec);
            Some(vec[m])
        }
    }
}

fn get_m(vec: &mut Vec<f32>) -> usize {
    vec.sort_by(|x,y| x.partial_cmp(y).unwrap_or(Ordering::Equal));
    vec.len() / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_vec() {
        let mut vec:Vec<f32> = vec![];
        let result = median(&mut vec);
        assert_eq!(result, None);
    }

    #[test]
    fn odd_vec_len() {
        let mut vec = vec![55.0, 0.0, 5.0 ];
        let result = median(&mut vec);
        assert_eq!(result, Some(5.0));      
    }

    #[test]
    fn even_vec_len() {
        let mut vec = vec![55.0, 0.0, 6.0, 8.0 ];
        let result = median(&mut vec);
        assert_eq!(result, Some(7.0));
    }
}