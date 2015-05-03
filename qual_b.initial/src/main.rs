// Development started:  2015-05-03 13:53
// Development finished: --
#![feature(str_words)]
use std::io;

#[allow(non_snake_case)]
fn main() {
    use std::io::BufRead;
    let stdin = io::stdin();
    let mut in_lines = stdin.lock().lines()
                            .map(|line| line.ok().unwrap());
    
    let T: u8 = in_lines.next().unwrap().parse().unwrap();
   
    // Optimal Strategy:
    // If a split is optimal, that split must be a 50% split.
    for icase in 1..T+1 {
        use std::collections::{VecMap};
        // Skip the number of plates. Assume valid.
        let _ = in_lines.next().unwrap();
        let line = in_lines.next().unwrap();
        let iter = line.words()
                       .map(|s| s.parse::<usize>().unwrap());
         
        let mut Ps = VecMap::with_capacity(1000);
        
        // Create sorted histogram.
        for P in iter {
            if Ps.contains_key(&P) {
                let n: &mut usize = Ps.get_mut(&P).unwrap();
                *n += 1;
            } else {
                Ps.insert(P, 1);
            }
        }
       
        let mut nsplits = 0;
        let mut neats = 0;
        
        // Each step
        'sim: loop {
            let Pg;
            let Pnext;
            {
                let mut iter = Ps.keys().rev();
                Pg = iter.next().unwrap() - neats;
                let next_opt = iter.next();
                Pnext = match next_opt {
                    None => 0,
                    Some(P) => {
                        if P < neats {
                            0
                        } else {
                            P - neats
                        }
                    }
                }
            }
            if Pg == 0 {
                break 'sim;
            }
            
            let &nPg = Ps.get(&(Pg + neats)).unwrap();
           
            let Phalf = (Pg+1) / 2;
            let Psplit = if Phalf > Pnext { Phalf } else { Pnext };
            // If we don't need fewer minutes by splitting,
            if nPg + Psplit >= Pg {
                // Eat once
                neats += 1;
            } else {
                // Otherwise split nPg times
                nsplits += nPg;
                Ps.remove(&Pg);
                let Pa_in = Pg/2 + neats;
                if Ps.contains_key(&Pa_in) {
                    *Ps.get_mut(&Pa_in).unwrap() += nPg;
                } else {
                    Ps.insert(Pa_in, nPg);
                }
                
                let Pb_in = (Pg+1)/2 + neats;
                if Ps.contains_key(&Pb_in) {
                    *Ps.get_mut(&Pb_in).unwrap() += nPg;
                } else {
                    Ps.insert(Pb_in, nPg);
                }
            }
        } // 'sim
        println!("Case #{}: {}", icase, nsplits + neats);
    }
}
