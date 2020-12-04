use std::fs;

fn main() {

    let contents = fs::read_to_string("./docs/01.txt")
        .expect("Something went wrong reading the file");
    
    'outer1: for x in contents.lines() {
        for y in contents.lines() {
            let sum1 = &x.parse::<usize>().unwrap();
            let sum2 = &y.parse::<usize>().unwrap();

            if sum1 + sum2 == 2020 {
                let result = sum1 * sum2;
                println!("x = {}, y = {}, result = {}", sum1, sum2, result);
                break 'outer1;
            }
        }
    }

    'outer2: for x in contents.lines() {
        for y in contents.lines() {
            for z in contents.lines() {
                let sum1 = &x.parse::<usize>().unwrap();
                let sum2 = &y.parse::<usize>().unwrap();
                let sum3 = &z.parse::<usize>().unwrap();
    
                if sum1 + sum2 + sum3 == 2020 {
                    let result = sum1 * sum2 * sum3;
                    println!("x = {}, y = {}, result = {}", sum1, sum2, result);
                    break 'outer2;
                }
            }
        }
    }
}
