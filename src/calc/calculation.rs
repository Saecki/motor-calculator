use std::f64::consts::PI;

use crate::calc::equation::Equation;
use crate::calc::number::Num;
use crate::calc::operation::Op;

/// A struct that holds the data necessary for calculations regarding an electrical motor and it's
/// transmission.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Calculation {
    // Motor
    /// Voltage: U [V]
    pub u: Num,
    /// Current: I [A]
    pub i: Num,
    /// Armature resistance: R<sub>A</sub> [Ω]
    pub r_a: Num,
    /// Input power: P<sub>in</sub> [W]
    pub p_in: Num,
    /// Motor power: P<sub>M</sub> [W]
    pub p_m: Num,
    /// Motor power loss: P<sub>ML</sub> [W]
    pub p_m_l: Num,
    /// Electrical motor power loss: P<sub>ML_el</sub> [W]
    pub p_m_l_el: Num,
    /// Mechanical motor power loss: P<sub>ML_mech</sub> [W]
    pub p_m_l_mech: Num,
    /// Motor Efficiency: η<sub>M</sub> [%]
    pub eta_m: Num,
    /// Motor torque: M<sub>M</sub> [Nm]
    pub m_m: Num,
    /// Motor speed: n<sub>M</sub> [rpm]
    pub n_m: Num,

    // Transmission
    /// Transmission power: P<sub>T</sub> [W]
    pub p_t: Num,
    /// Transmission power loss: P<sub>TL</sub> [W]
    pub p_t_l: Num,
    /// Transmission Efficiency: η<sub>M</sub> [%]
    pub eta_t: Num,
    /// Transmission torque: M<sub>T</sub> [Nm]
    pub m_t: Num,
    /// Transmission speed: n<sub>T</sub> [rpm]
    pub n_t: Num,
}

impl Calculation {
    pub fn new() -> Calculation {
        Calculation {
            u: Num::None,
            i: Num::None,
            r_a: Num::None,
            p_in: Num::None,
            p_m: Num::None,
            p_m_l: Num::None,
            p_m_l_el: Num::None,
            p_m_l_mech: Num::None,
            eta_m: Num::None,
            m_m: Num::None,
            n_m: Num::None,
            p_t: Num::None,
            p_t_l: Num::None,
            eta_t: Num::None,
            m_t: Num::None,
            n_t: Num::None,
        }
    }

    pub fn try_fill_missing(&self) -> crate::error::Result<Calculation> {
        let mut calc = self.clear_output();

        for _ in 0..6 {
            if let Ok(c) = calc.calculate() {
                calc = c;
            }
        }

        Ok(calc)
    }

    pub fn calculate(&self) -> crate::error::Result<Calculation> {
        let mut calc = *self;

        if let Ok(eq) = Equation::new(calc.u, Op::Mul, calc.i, calc.p_in).solve() {
            calc.u = eq.a;
            calc.i = eq.b;
            calc.p_in = eq.c;
        }

        if let Ok(eq) = Equation::new(calc.i * calc.i, Op::Mul, calc.r_a, calc.p_m_l_el).solve() {
            calc.i = eq.a / calc.i;
            calc.r_a = eq.b;
            calc.p_m_l_el = eq.c;
        }

        if let Ok(eq) = Equation::new(calc.p_in, Op::Mul, calc.eta_m / 100.0, calc.p_m).solve() {
            calc.p_in = eq.a;
            calc.eta_m = eq.b * 100.0;
            calc.p_m = eq.c;
        }

        if let Ok(eq) = Equation::new(calc.p_m_l, Op::Add, calc.p_m, calc.p_in).solve() {
            calc.p_m_l = eq.a;
            calc.p_m = eq.b;
            calc.p_in = eq.c;
        }

        if let Ok(eq) = Equation::new(calc.p_m_l_el, Op::Add, calc.p_m_l_mech, calc.p_m_l).solve() {
            calc.p_m_l_el = eq.a;
            calc.p_m_l_mech = eq.b;
            calc.p_m_l = eq.c;
        }

        if let Ok(eq) = Equation::new(calc.n_m * (2.0 * PI / 60.0), Op::Mul, calc.m_m, calc.p_m).solve() {
            calc.n_m = eq.a / (2.0 * PI / 60.0);
            calc.m_m = eq.b;
            calc.p_m = eq.c;
        }

        if let Ok(eq) = Equation::new(calc.p_m, Op::Mul, calc.eta_t / 100.0, calc.p_t).solve() {
            calc.p_m = eq.a;
            calc.eta_t = eq.b * 100.0;
            calc.p_t = eq.c;
        }

        if let Ok(eq) = Equation::new(calc.p_t, Op::Add, calc.p_t_l, calc.p_m).solve() {
            calc.p_t = eq.a;
            calc.p_t_l = eq.b;
            calc.p_m = eq.c;
        }

        if let Ok(eq) = Equation::new(calc.n_t * (2.0 * PI / 60.0), Op::Mul, calc.m_t, calc.p_t).solve() {
            calc.n_t = eq.a / (2.0 * PI / 60.0);
            calc.m_t = eq.b;
            calc.p_t = eq.c;
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
        if calc.p_m_l.is_output() { calc.p_m_l = Num::None; }
        if calc.p_m_l_el.is_output() { calc.p_m_l_el = Num::None; }
        if calc.p_m_l_mech.is_output() { calc.p_m_l_mech = Num::None; }
        if calc.eta_m.is_output() { calc.eta_m = Num::None; }
        if calc.m_m.is_output() { calc.m_m = Num::None; }
        if calc.n_m.is_output() { calc.n_m = Num::None; }
        if calc.p_t.is_output() { calc.p_t = Num::None; }
        if calc.p_t_l.is_output() { calc.p_t_l = Num::None; }
        if calc.eta_t.is_output() { calc.eta_t = Num::None; }
        if calc.m_t.is_output() { calc.m_t = Num::None; }
        if calc.n_t.is_output() { calc.n_t = Num::None; }

        calc
    }
}

#[cfg(test)]
mod test {
    use rand::Rng;

    use crate::calc::calculation::Calculation;
    use crate::calc::number::Num;
    use crate::error::ErrorKind::Overconstrained;

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
