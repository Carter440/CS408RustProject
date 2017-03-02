extern crate rand;
extern crate time;

use rand::Rng;
use time::PreciseTime;

fn main() {
    use std::io;

    println!("How many test cases would you like to run?");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let input: i32 = input
        .trim()
        .parse()
        .expect("Non-integer input detected. Exiting program.");

    {
        println!("=== SORTING WITH MERGE SORT ===");

        let mut times: Vec<time::Duration> = vec![];

        for j in 1..(input + 1) {
            let start = PreciseTime::now();
            let mut rng = rand::thread_rng();
            let mut vector: Vec<i32> = vec![];
            for _ in 1..10000 * j
                {
                    vector.push(rng.gen_range(i32::min_value(), i32::max_value()));
                }

            println!("CASE {}: SORTING LIST OF {} ELEMENTS", j, 10000 * j);

            merge_sort(&mut vector);
            assert!(is_sorted(&mut vector));

            /*for k in &vector
                {
                    println!("{}", k);
                }*/

            let end = PreciseTime::now();
            println!("LIST SORTED IN {} SECONDS\n", start.to(end));
            times.push(start.to(end));
        }

        println!(" === SORTING TIMES (MERGE SORT) === ");

        for i in 1..(input + 1)
            {
                println!("CASE {}: \t {} SECONDS", i, times[(i-1) as usize]);
            }

        println!("")
    }

    {
        println!("=== SORTING WITH INSERTION SORT ===");

        let mut times: Vec<time::Duration> = vec![];

        for j in 1..(input + 1) {
            let start = PreciseTime::now();
            let mut rng = rand::thread_rng();
            let mut vector: Vec<i32> = vec![];
            for _ in 1..10000 * j
                {
                    vector.push(rng.gen_range(i32::min_value(), i32::max_value()));
                }

            println!("CASE {}: SORTING LIST OF {} ELEMENTS", j, 10000 * j);

            insertion_sort(&mut vector);
            assert!(is_sorted(&mut vector));

            /*for k in &vector
                {
                    println!("{}", k);
                }*/

            let end = PreciseTime::now();
            println!("LIST SORTED IN {} SECONDS\n", start.to(end));
            times.push(start.to(end));
        }

        println!(" === SORTING TIMES (INSERTION SORT) === ");

        for i in 1..(input + 1)
            {
                println!("CASE {}: \t {} SECONDS", i, times[(i-1) as usize]);
            }

        println!("");
    }
}

fn merge_sort(list: &mut Vec<i32>) {
    let mut result: Vec<i32> = vec![0; list.len() as usize];
    let length = list.len() as i32;
    split(list, 0, length, &mut result);
}

fn split(b: &mut Vec<i32>, begin: i32, end: i32, a: &mut Vec<i32>)
{
    if end - begin < 2 {return;}

    let middle = (begin + end)/2;

    split(b, begin, middle, a);
    split(b, middle, end, a);

    merge(b, begin, middle, end, a);
    copy_vector(b, begin as usize, end as usize, a);
}

fn merge(a: &mut Vec<i32>, begin: i32, middle: i32, end: i32, b: &mut Vec<i32>)
{
    let mut i = begin;
    let mut j = middle;

    for k in begin..end
    {
        if i < middle && (j >= end || a[i as usize] <= a[j as usize])
        {
            b[k as usize] = a[i as usize];
            i += 1;
        } else {
            b[k as usize] = a[j as usize];
            j += 1;
        }
    }
}

fn copy_vector(output: &mut Vec<i32>, begin: usize, end:usize, input: &mut Vec<i32>)
{
    for i in begin..end {output[i] = input[i];}
}

fn insertion_sort<T>(vec: &mut Vec<T>) where T: Ord + Copy {
    fn insert<U>(vec: &mut Vec<U>, pos: usize, value: U) where U: Ord + Copy {
        assert!(pos > 0);
        let mut pos: usize = pos - 1;
        loop {
            let value_at_pos = vec[pos];
            if value_at_pos <= value {
                break;
            }
            vec[pos + 1] = value_at_pos;
            if pos == 0 {
                vec[pos] = value;
                return ();
            }
            pos -= 1;
        }
        vec[pos + 1] = value;
    }
    for i in 1..vec.len() {
        let value = vec[i];
        insert(vec, i, value);
    }
}

fn is_sorted(vec: &mut Vec<i32>) -> bool
{
    for i in 1..(vec.len() - 1)
    {
        if vec[i] < vec[i - 1] {return false;}
    }
    true
}

