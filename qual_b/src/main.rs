// Development started:  2015-05-03 ~7:00
// Development finished: 2015-05-03 ~10:00
use std::io;
use std::cmp;

fn main() {
    use std::io::BufRead;
    
    let stdin = io::stdin();
    let mut in_lines = stdin.lock().lines();
    let t = in_lines.next().unwrap()
                    .ok().unwrap()
                    .parse::<usize>().unwrap(); 
    
    for icase in 1..t+1 {
        // Load input
        let _ = in_lines.next().unwrap()
                        .ok().unwrap()
                        .parse::<usize>().unwrap();
       
        let line = in_lines.next().unwrap().ok().unwrap();
        let mut ps = line.split_whitespace()
                         .map(|s| s.parse::<usize>().unwrap())
                         .collect::<Vec<_>>();
        ps.sort();
        let ps = ps;
        
        let &p_max = ps.iter().max().unwrap();
        
        // Try all thresholds up to max_p.
        let mut best = p_max;
        for threshold in 2..p_max {
            let mut total_moves = 0;
            for &p in ps.iter() {
                total_moves += (p-1)/threshold; // Round up, subtract 1
            }
            best = cmp::min(best, threshold + total_moves);
        }
        println!("Case #{}: {}", icase, best);
    }
}
