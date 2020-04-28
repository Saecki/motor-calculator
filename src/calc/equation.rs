use crate::calc::number::Num;
use crate::calc::operation::Op;
use crate::error::Error;
use crate::error::ErrorKind::{Overconstrained, Underconstrained};

/// A stucture representing a simple equation consisting of three variables and an operation. The
/// variables are dependent in the pattern a operation b = c.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Equation {
    pub op: Op,
    pub a: Num,
    pub b: Num,
    pub c: Num,
}

impl Equation {
    /// Creates a new equation containing the variables and the operation.
    pub fn new(a: Num, op: Op, b: Num, c: Num) -> Equation {
        Equation { op, a, b, c }
    }

    /// Attempts to solve the equation by calculating the value of the remaining variable. Returns
    /// a Error of kind Overconstrained if all 3 numbers a defined or Underconstrained if only one
    /// was defined. Otherwise the solved equation is returned.
    pub fn solve(&self) -> crate::error::Result<Equation> {
        let mut equation = *self;
        let mut commutative = true;
        let mut overconstrained = false;

        //normalizing
        if !equation.op.is_commutative() {
            let temp_c = equation.c;
            equation.c = equation.a;
            equation.a = temp_c;
            equation.op = equation.op.inv();
            commutative = false
        }

        //rearrange and calculate
        if equation.a.is_num() && equation.b.is_num() {
            if equation.c.is_none() {
                equation.c = Num::Out(equation.op.calc(equation.a.num(), equation.b.num()));
            } else {
                overconstrained = true;
            };
        } else if equation.a.is_num() && equation.c.is_num() {
            if equation.b.is_none() {
                equation.b = Num::Out(equation.op.inv().calc(equation.c.num(), equation.a.num()));
            } else {
                overconstrained = true;
            };
        } else if equation.b.is_num() && equation.c.is_num() {
            if equation.a.is_none() {
                equation.a = Num::Out(equation.op.inv().calc(equation.c.num(), equation.b.num()));
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
            let temp_c = equation.c;
            equation.c = equation.a;
            equation.a = temp_c;
            equation.op = equation.op.inv();
        }

        if overconstrained {
            return Err(Error::new(
                Overconstrained,
                "One value to many was provided",
            ));
        }

        Ok(equation)
    }
}

#[cfg(test)]
mod test {
    use rand::Rng;

    use crate::calc::equation::Equation;
    use crate::calc::number::Num;
    use crate::calc::operation::Op;
    use crate::error::ErrorKind::Overconstrained;

    #[test]
    fn test_add_equations() {
        let mut rng = rand::thread_rng();
        let first = rng.gen();
        let second = rng.gen();

        let equation1 = Equation::new(Num::In(first), Op::Add, Num::In(second), Num::None);
        assert_eq!(equation1.solve().unwrap().c.num(), first + second);

        let equation2 = Equation::new(Num::In(first), Op::Add, Num::None, Num::In(second));
        assert_eq!(equation2.solve().unwrap().b.num(), second - first);

        let equation3 = Equation::new(Num::None, Op::Add, Num::In(first), Num::In(second));
        assert_eq!(equation3.solve().unwrap().a.num(), second - first);

        let equation4 = Equation::new(Num::In(first), Op::Add, Num::In(first), Num::In(second));
        match equation4.solve().err().unwrap().kind {
            Overconstrained => (),
            _ => panic!("Expected Error with ErrorKind Overconstrained")
        }
    }

    #[test]
    fn test_sub_equations() {
        let mut rng = rand::thread_rng();
        let first = rng.gen();
        let second = rng.gen();

        let equation1 = Equation::new(Num::In(first), Op::Sub, Num::In(second), Num::None);
        assert_eq!(equation1.solve().unwrap().c.num(), first - second);

        let equation2 = Equation::new(Num::In(first), Op::Sub, Num::None, Num::In(second));
        assert_eq!(equation2.solve().unwrap().b.num(), first - second);

        let equation3 = Equation::new(Num::None, Op::Sub, Num::In(first), Num::In(second));
        assert_eq!(equation3.solve().unwrap().a.num(), first + second);

        let equation4 = Equation::new(Num::In(first), Op::Sub, Num::In(first), Num::In(second));
        match equation4.solve().err().unwrap().kind {
            Overconstrained => (),
            _ => panic!("Expected Error with ErrorKind Overconstrained")
        }
    }

    #[test]
    fn test_mul_equations() {
        let mut rng = rand::thread_rng();
        let first = rng.gen();
        let second = rng.gen();

        let equation1 = Equation::new(Num::In(first), Op::Mul, Num::In(second), Num::None);
        assert_eq!(equation1.solve().unwrap().c.num(), first * second);

        let equation2 = Equation::new(Num::In(first), Op::Mul, Num::None, Num::In(second));
        assert_eq!(equation2.solve().unwrap().b.num(), second / first);

        let equation3 = Equation::new(Num::None, Op::Mul, Num::In(first), Num::In(second));
        assert_eq!(equation3.solve().unwrap().a.num(), second / first);

        let equation4 = Equation::new(Num::In(first), Op::Mul, Num::In(first), Num::In(second));
        match equation4.solve().err().unwrap().kind {
            Overconstrained => (),
            _ => panic!("Expected Error with ErrorKind Overconstrained")
        }
    }

    #[test]
    fn test_div_equations() {
        let mut rng = rand::thread_rng();
        let first = rng.gen();
        let second = rng.gen();

        let equation1 = Equation::new(Num::In(first), Op::Div, Num::In(second), Num::None);
        assert_eq!(equation1.solve().unwrap().c.num(), first / second);

        let equation2 = Equation::new(Num::In(first), Op::Div, Num::None, Num::In(second));
        assert_eq!(equation2.solve().unwrap().b.num(), first / second);

        let equation3 = Equation::new(Num::None, Op::Div, Num::In(first), Num::In(second));
        assert_eq!(equation3.solve().unwrap().a.num(), first * second);

        let equation4 = Equation::new(Num::In(first), Op::Div, Num::In(first), Num::In(second));
        match equation4.solve().err().unwrap().kind {
            Overconstrained => (),
            _ => panic!("Expected Error with ErrorKind Overconstrained")
        }
    }
}
