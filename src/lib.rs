struct Rational{
    a: i64,
    b: i64,
}

impl Rational {
    pub fn new(a: i64, b: i64) -> Rational{
        Rational{
            a: a,
            b: b,
        }
    }
    
    pub fn add(a: Rational, b: Rational) -> Rational{
        let new_a = lcm(a.a, b.a);
        Rational{
            a: new_a,
            b: new_a / a.a * b.b + new_a / b.a * a.b,
        }
    }
    
    pub fn mult(a: Rational, b: Rational) -> Rational{
        let aa_bb: lcm(a.a, b.b);
        let ab_ba: lcm(a.b, b.a);
        Rational{
            a: a.a * b.a,
            b: a.b * b.b,
        }
    }
    
    pub fn inverse(r: Rational) -> Rational {
        Rational {
            a: r.b,
            b: r.a,
        }
    }
    
    pub fn minus(r: Rational) -> Rational {
        Rational {
            a: r.a * -1,
            b: r.b,
        }
    }
}

fn gcd(a: i64, b: i64) -> i64{
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::Rational;
    #[test]
    fn new_test() {
        let r :Rational = Rational::new(1, 2);
        assert_eq!(r.a, 1);
        assert_eq!(r.b, 2);
    }
    
    #[test]
    fn lcm_test(){
        use super::lcm;
        assert_eq!(lcm(3, 2), 6);
        assert_eq!(lcm(4, 8), 8);
        assert_eq!(lcm(12, 8), 24);
    }
    
    #[test]
    fn gcd_test(){
        use super::gcd;
        assert_eq!(gcd(3, 2), 1);
        assert_eq!(gcd(4, 8), 4);
        assert_eq!(gcd(12, 8), 4);
    }
    
    #[test]
    fn add_test(){
        let r1: Rational = Rational::new(2, 1);
        let r2: Rational = Rational::new(3, 1);
        let ans = Rational::add(r1, r2);
        assert_eq!(ans.a, 6);
        assert_eq!(ans.b, 5);
    }
    
    #[test]
    fn mult_test(){
        let r1: Rational = Rational::new(2, 1);
        let r2: Rational = Rational::new(3, 1);
        let ans = Rational::mult(r1, r2);
        assert_eq!(ans.a, 6);
        assert_eq!(ans.b, 1);
    }
    
    #[test]
    fn inverse_test(){
        let r: Rational = Rational::new(2, 3);
        let ans: Rational = Rational::inverse(r);
        assert_eq!(ans.a, 3);
        assert_eq!(ans.b, 2);
    }
    
    #[test]
    fn minus_test(){
        let r: Rational = Rational::new(2, 3);
        let ans: Rational = Rational::minus(r);
        assert_eq!(ans.a, -2);
        assert_eq!(ans.b, 3);
    }
}
