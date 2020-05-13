use std::fs;
use std::time::Instant;

fn main() {
    let a = vec![1,2,3,4,5,7,9,23,3654];
    assert_eq!(quadratic_search(&a, 2), Some(2));
    assert_eq!(quadratic_search(&a, 10), None);
    
    let filename = "../text.txt";
    let content = fs::read_to_string(filename).unwrap_or(String::from("Hello World"));
    let mut words: Vec<&str> = content.split_whitespace().collect();
    println!("Searching in {} words", words.len()); 
    words.sort();
    {
        let now = Instant::now();
        let result = quadratic_search(&words, "zeigt");
        let duration = now.elapsed();
        println!("Result: {:?} Took {:?}", result, duration);
    }
    {
        let now = Instant::now();
        for word in words {
            if word == "zeigt" {
                break;
            }
        }
        let duration = now.elapsed();
        println!("Linear Search Took {:?}", duration);
    }
}

/// Runs a quadratic binary search
/// 
/// # Remarks
/// The initial search index is not interpolated but choosen as the middle of the slice 
fn quadratic_search<T: PartialOrd>(vec: &[T], a: T) -> Option<T> {
    if vec.is_empty() {
        return None;
    }

    let k = vec.len() / 2;
    let step_size =  (vec.len() as f64).sqrt() as i64;

    if vec[k] == a {
        Some(a)
    } else if vec[k] > a {
        let mut upper = k as i64;
        let mut lower = k as i64 - step_size;
        while lower > 0 && vec[lower as usize] > a {
            upper -= step_size;
            lower -= step_size;
        }
        if lower > 0 {
            quadratic_search(vec.get((lower as usize)..(upper as usize)).unwrap(), a)
        } else {
            quadratic_search(vec.get(0..(upper as usize)).unwrap(), a)
        }
    } else {
        let mut upper = k as i64 + step_size;
        let mut lower = k as i64;
        while upper < vec.len() as i64 && vec[upper as usize] < a {
            upper += step_size;
            lower += step_size;
        }
        if upper < vec.len() as i64 {
            quadratic_search(vec.get((lower as usize)..(upper as usize)).unwrap(), a)
        } else {
            quadratic_search(vec.get((lower as usize)..(vec.len() - 1)).unwrap(), a)
        }
    }
}