use crate::equation::Equation;
use crate::error::Error;
use crate::error::ErrorKind::{Overconstrained, Underconstrained};
use crate::number::Num;
use crate::operation::Op;

#[derive(Copy, Clone)]
pub struct Calculation {
    //motor
    u: Num,
    i: Num,
    r_a: Num,
    p_in: Num,
    p_m: Num,
    p_m_v_el: Num,
    p_m_v_mech: Num,
    //gearing
    p_g: Num,
    m_g: Num,
    n_g: Num,
    p_g_v: Num,
}

impl Calculation {
    pub fn new() -> Calculation {
        Calculation {
            u: Num::None,
            i: Num::None,
            r_a: Num::None,
            p_in: Num::None,
            p_m: Num::None,
            p_m_v_el: Num::None,
            p_m_v_mech: Num::None,
            p_g: Num::None,
            m_g: Num::None,
            n_g: Num::None,
            p_g_v: Num::None,
        }
    }

    pub fn try_fill_missing(&self) -> crate::error::Result<Calculation> {
        let calc = self.clear_output();
        calc.try_fill_missing_motor()
    }

    pub fn try_fill_missing_motor(&self) -> crate::error::Result<Calculation> {
        let mut calc = *self;
        let mut overconstrained = false;

        match Equation::new(calc.u, Op::Mul, calc.i, calc.p_in).solve() {
            Err(e) => if let Overconstrained = e.kind {
                return Err(e);
            }
            Ok(eq) => {
                calc.u = eq.a;
                calc.i = eq.b;
                calc.p_in = eq.c;
            }
        }

        Ok(calc)
    }

    pub fn clear_output(&self) -> Self {
        let mut calc = *self;

        if calc.u.is_output() { calc.u = Num::None; }
        if calc.i.is_output() { calc.i = Num::None; }
        if calc.r_a.is_output() { calc.r_a = Num::None; }
        if calc.p_in.is_output() { calc.p_in = Num::None; }
        if calc.p_m.is_output() { calc.p_m = Num::None; }
        if calc.p_m_v_el.is_output() { calc.p_m_v_el = Num::None; }
        if calc.p_m_v_mech.is_output() { calc.p_m_v_mech = Num::None; }
        if calc.p_g.is_output() { calc.p_g = Num::None; }
        if calc.m_g.is_output() { calc.m_g = Num::None; }
        if calc.n_g.is_output() { calc.n_g = Num::None; }
        if calc.p_g_v.is_output() { calc.p_g_v = Num::None; }

        calc
    }
}

mod test {
    use rand::Rng;

    use crate::calculation::Calculation;
    use crate::error::ErrorKind::Overconstrained;
    use crate::number::Num;

    #[test]
    fn test_calculation1() {
        let mut rng = rand::thread_rng();
        let first = rng.gen();
        let second = rng.gen();

        let mut calc1 = Calculation::new();
        calc1.u = Num::In(first);
        calc1.i = Num::In(second);
        assert_eq!(calc1.try_fill_missing().unwrap().p_in.num(), second * first);

        let mut calc2 = Calculation::new();
        calc2.u = Num::In(first);
        calc2.p_in = Num::In(second);
        assert_eq!(calc2.try_fill_missing().unwrap().i.num(), second / first);

        let mut calc3 = Calculation::new();
        calc3.i = Num::In(first);
        calc3.p_in = Num::In(second);
        assert_eq!(calc3.try_fill_missing().unwrap().u.num(), second / first);

        let mut calc4 = Calculation::new();
        calc4.u = Num::In(first);
        calc4.i = Num::In(first);
        calc4.p_in = Num::In(second);
        match calc4.try_fill_missing().err().unwrap().kind {
            Overconstrained => (),
            _ => panic!("Expected Error with ErrorKind Overconstrained")
        }
    }
}
