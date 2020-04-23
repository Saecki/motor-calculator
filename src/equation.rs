use crate::error::ErrorKind::{Overconstrained, Underconstrained};
use crate::error::Error;

#[derive(Copy, Clone, Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn is_add(&self) -> bool {
        if let Op::Add = self { true } else { false }
    }

    pub fn is_sub(&self) -> bool {
        if let Op::Sub = self { true } else { false }
    }

    pub fn is_mul(&self) -> bool {
        if let Op::Mul = self { true } else { false }
    }

    pub fn is_div(&self) -> bool {
        if let Op::Div = self { true } else { false }
    }

    pub fn opposite(&self) -> Op {
        match self {
            Op::Add => Op::Sub,
            Op::Sub => Op::Add,
            Op::Mul => Op::Div,
            Op::Div => Op::Mul,
        }
    }

    pub fn calculated(&self, a: f64, b: f64) -> f64 {
        match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Equation {
    operation: Op,
    a: Option<f64>,
    b: Option<f64>,
    c: Option<f64>,
}

impl Equation {
    pub fn new(a: Option<f64>, operation: Op, b: Option<f64>, c: Option<f64>) -> Equation {
        Equation { operation, a, b, c }
    }

    pub fn from(a: f64, operation: Op, b: f64) -> Equation {
        Equation { operation, a: Some(a), b: Some(b), c: None }
    }

    pub fn solved(&self) -> crate::error::Result<Equation> {
        let mut calc = *self;
        let mut commutative = true;
        let mut overconstrained = false;

        //normalizing
        if calc.operation.is_sub() || calc.operation.is_div() {
            let temp_c = calc.c;
            calc.c = calc.a;
            calc.a = temp_c;
            calc.operation = calc.operation.opposite();
            commutative = false
        }

        //rearrange
        if let (Some(a), Some(b)) = (calc.a, calc.b) {
            if calc.c.is_none() {
                calc.c = Some(calc.operation.calculated(a, b));
            } else {
                overconstrained = true;
            };
        } else if let (Some(a), Some(c)) = (calc.a, calc.c) {
            if calc.b.is_none() {
                calc.b = Some(calc.operation.opposite().calculated(c, a));
            } else {
                overconstrained = true;
            };
        } else if let (Some(b), Some(c)) = (calc.b, calc.c) {
            if calc.a.is_none() {
                calc.a = Some(calc.operation.opposite().calculated(c, b));
            } else {
                overconstrained = true;
            };
        } else {
            return Err(Error::new(
                Underconstrained,
                "At least one move value is needed",
            ));
        }

        //reverting normalization
        if !commutative {
            let temp_c = calc.c;
            calc.c = calc.a;
            calc.a = temp_c;
            calc.operation = calc.operation.opposite();
        }

        if overconstrained {
            return Err(Error::new(
                Overconstrained,
                "One value to many was provided",
            ));
        }

        Ok(calc)
    }
}

mod test {
    use crate::equation::{Equation, Op};
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();
        let a = rng.gen();
        let b = rng.gen();

        let equation1 = Equation::from(a, Op::Add, b);
        assert_eq!(equation1.solved().unwrap().c.unwrap(), a + b);

        let equation2 = Equation::from(a, Op::Sub, b);
        assert_eq!(equation2.solved().unwrap().c.unwrap(), a - b);

        let equation3 = Equation::from(a, Op::Mul, b);
        assert_eq!(equation3.solved().unwrap().c.unwrap(), a * b);

        let equation4 = Equation::from(a, Op::Div, b);
        assert_eq!(equation4.solved().unwrap().c.unwrap(), a / b);
    }
}
