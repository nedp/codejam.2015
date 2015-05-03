// Development started:  2015-05-03 12:57
// Development finished: 2015-05-03 13:35
#![feature(str_words)]
use std::io;

#[allow(non_snake_case)]
fn main() {
    use std::io::BufRead;
    let stdin = io::stdin();
    let in_lines = stdin.lock().lines()
                        .map(|line| line.ok().unwrap())
                        .enumerate()
                        .skip(1); // Start with case 1.
    
    for (icase, line) in in_lines {
        let mut words = line.words();
        let Smax: usize = words.next().unwrap().parse().unwrap();
        let mut hist: Vec<u32> = Vec::with_capacity(Smax);
        
        for c in words.next().unwrap().chars() {
            hist.push(c.to_digit(10).unwrap());
        }
        
        let mut n_standing = 0;
        let mut n_friends = 0;
        for (S, &n) in hist.iter().enumerate() {
            if S > n_standing {
                n_friends += S - n_standing;
                n_standing = S;
            }
            n_standing += n as usize;
        }
        
        println!("Case #{}: {}", icase, n_friends);
    }
}
