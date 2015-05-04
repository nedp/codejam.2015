// Development started:  2015-05-05 01:40
// Development paused:   2015-05-05 02:51
#![feature(rustc_private)]

#[macro_use] extern crate log;

use std::io;

fn main() {
    use std::io::BufRead;
    let stdin = io::stdin();
    let mut in_lines = stdin.lock().lines().map(|l| l.ok().unwrap());
    let t = in_lines.next().unwrap()
                    .parse::<u8>().unwrap();
    for icase in 1..t+1 {
        let line = in_lines.next().unwrap();
        let (x, c, r) = {
            let mut params = line.split_whitespace()
                                 .map(|p| p.parse::<u8>().unwrap());
            let x = params.next().unwrap() as u8;
            let c = params.next().unwrap() as u8;
            let r = params.next().unwrap() as u8;
            (x, c, r)
        };
        
        if richard_wins(x, c, r) {
            println!("Case #{}: RICHARD", icase);
        } else {
            println!("Case #{}: GABRIEL", icase);
        }
    }
}

fn richard_wins(x: u8, c: u8, r: u8) -> bool {
    let x = x as u16;
    let r = r as u16;
    let c = c as u16;
    // Richard can win by default.
    let n_squares = c * r;
    if n_squares % x != 0 {
        debug!("n_squares isn't divisible by x");
        return true;
    }
    
    // Richard can win by picking a shape which
    // can't be placed, or which blocks off
    // a corner containing a number of squares
    // indivisibe by x.
    for i in 1..1+x/2 {
        let rest = x - i + 1;
        if (i > c || rest > r) && (i > r || rest > c) {
            debug!("({}, {}) L shape can't fit", i, rest);
            return true;
        }
        if i == c || i == r {
            for j in 0..1+i/2 {
                let area = (rest-1) * (j+1);
                if area % x != 0 {
                    debug!("({} ({}) {}) T shape blocks indivisble corner region"
                           , j, rest - 1, i - j);
                    return true;
                }
            }
        }
    } 
    
    // Richard can win by bisecting the board such
    // that the difference in half sizes is 1, or 2.
    // The difference must not be divisible by x.
    // Richard can't do this if x has the same
    // evenness to either c or r.
    if x <= c || x <= r {
        debug!("x too small to bisect");
        return false;
    }
    let x_even = x % 2 == 0;
    let r_even = r % 2 == 0;
    let c_even = c % 2 == 0;
    if r_even != c_even {
        debug!("odd (r - c)");
        return false;
    }
    if x_even == r_even {
        debug!("even (x - r)");
        return false;
    }
    debug!("richard can bisect unevenly");
    true
}
