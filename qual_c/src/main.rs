// Development started:  2015-05-04 ~14:30
// Development started:  2015-05-05 01:00
// with long breaks

use std::io;
use std::cmp;
use std::ops::Mul;

fn main() {
    use std::io::BufRead;
    let stdin = std::io::stdin();
    let mut in_lines = stdin.lock().lines().map(|l| l.ok().unwrap());
    let t = in_lines.next().unwrap()
                    .parse::<usize>().unwrap();
    
    for icase in 1..t+1 {
        // Handle case
        let line = in_lines.next().unwrap();
        let mut params = line.split_whitespace()
                             .map(|p| p.parse::<usize>().unwrap());
        let l = params.next().unwrap();
        let x = params.next().unwrap();
        let s = in_lines.next().unwrap();
        
        if icase != 6 {
            continue;
        }
        if is_valid(&s[0..l], x) {
            println!("Case #{}: YES", icase);
        } else {
            println!("Case #{}: NO", icase);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sign {
    P,
    N,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Q {
    I,
    J,
    K,
    One,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct QV {
    q: Q,
    s: Sign,
}

fn is_valid(s: &str, x: usize) -> bool {
    const NEG_ONE: QV = QV{q: Q::One, s: Sign::N};
    product(s, x) == NEG_ONE && has_ij(s, x)
}

fn product(s: &str, x: usize) -> QV {
    // Should test for x = 1..4
    const ONE: QV = QV{q: Q::One, s: Sign::P};
    if x % 4 == 0 {
        return ONE;
    }
    // O(L)
    let mut v = ONE;
    for c in s.chars() {
        v = v * QV::from_char(c).unwrap();
    }
    let v0 = v;
    for i in 2..x%4 + 1 {
        v = v * v0;
    }
    return v
}

fn has_ij(s: &str, x: usize) -> bool {
    // O(L)
    const ONE: QV = QV{q: Q::One, s: Sign::P};
    const I: QV = QV{q: Q::I, s: Sign::P};
    const J: QV = QV{q: Q::J, s: Sign::P};
    // Value after `imax` chars is always 1,
    // because z^4 = 1 for all z = product(valid_s).
    let nr = cmp::min(x, 8);
    let mut i = ONE;
    let mut j = ONE;
    for r in 0..nr {
        for c in s.chars() {
            if i != I {
                i = i * QV::from_char(c).unwrap();
            } else if j != J {
                j = j * QV::from_char(c).unwrap();
            } else {
                break;
            }
        }
    }
    i == I && j == J
}

impl QV {
    fn from_char(c: char) -> Option<QV> {
        use Q::{I, J, K, One};
        use Sign::P;
        match c {
            'i' => Some(QV{q: I, s: P}),
            'j' => Some(QV{q: J, s: P}),
            'k' => Some(QV{q: K, s: P}),
            _ => None,
        }
    }
}

impl Mul<Sign> for Sign {
    type Output = Sign;
    
    fn mul(self, other: Sign) -> Sign {
        use Sign::{P, N};
        match self {
            P => other,
            N => match other {
                P => N,
                N => P,
            }
        }
    }
}

impl Q {
    fn sign_mul(self, other: Q) -> Sign {
        use Q::{I, J, K, One};
        use Sign::{P, N};
        
        match self {
            One => P,
            I => match other {
                I | K => N,
                _ => P,
            },
            J => match other {
                I | J => N,
                _ => P,
            },
            K => match other {
                J | K => N,
                _ => P,
            },
        }
    }
}

impl Mul<Q> for Q {
    type Output = Q;
    
    fn mul(self, other: Q) -> Q {
        use Q::{I, J, K, One};
        match self {
            One => other,
            I => match other {
                One => I,
                I   => One,
                J   => K,
                K   => J,
            },
            J => match other {
                One => J,
                I   => K,
                J   => One,
                K   => I,
            },
            K => match other {
                One => K,
                I   => J,
                J   => I,
                K   => One,
            },
        }
    }
}

impl Mul<QV> for QV {
    type Output = QV;
    
    fn mul(self, other: QV) -> QV {
        QV{q: self.q * other.q, s: (self.s * other.s) * self.q.sign_mul(other.q)}
    }
}
