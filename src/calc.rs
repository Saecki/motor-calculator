use crate::error::Error;
use crate::error::ErrorKind::Underconstrained;

#[derive(Copy, Clone)]
pub enum Number {
    Input(f64),
    Output(f64),
    None,
}

impl Number {
    pub fn is_input(&self) -> bool {
        match self {
            Number::Input(_) => true,
            _ => false,
        }
    }

    pub fn is_output(&self) -> bool {
        match self {
            Number::Output(_) => true,
            _ => false,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            Number::None => true,
            _ => false,
        }
    }

    pub fn as_option(&self) -> Option<f64> {
        match self {
            Number::Input(v) => Some(*v),
            Number::Output(v) => Some(*v),
            Number::None => None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Calculation {
    //motor
    u: Number,
    i: Number,
    r_a: Number,
    p_in: Number,
    p_m: Number,
    p_m_v_el: Number,
    p_m_v_mech: Number,
    //gearing
    p_g: Number,
    m_g: Number,
    n_g: Number,
    p_g_v: Number,
}

impl Calculation {
    pub fn new() -> Calculation {
        Calculation {
            u: Number::None,
            i: Number::None,
            r_a: Number::None,
            p_in: Number::None,
            p_m: Number::None,
            p_m_v_el: Number::None,
            p_m_v_mech: Number::None,
            p_g: Number::None,
            m_g: Number::None,
            n_g: Number::None,
            p_g_v: Number::None,
        }
    }

    pub fn try_fill_missing(&self) -> crate::error::Result<Calculation> {
        let calc = self.clear_output();
        calc.try_fill_missing_motor()
    }

    pub fn try_fill_missing_motor(&self) -> crate::error::Result<Calculation> {
        Err(Error::new(
            Underconstrained,
            "temp",
        ))
    }

    pub fn clear_output(&self) -> Self {
        let mut calc = *self;

        if calc.u.is_output() { calc.u = Number::None; }
        if calc.i.is_output() { calc.i = Number::None; }
        if calc.r_a.is_output() { calc.r_a = Number::None; }
        if calc.p_in.is_output() { calc.p_in = Number::None; }
        if calc.p_m.is_output() { calc.p_m = Number::None; }
        if calc.p_m_v_el.is_output() { calc.p_m_v_el = Number::None; }
        if calc.p_m_v_mech.is_output() { calc.p_m_v_mech = Number::None; }
        if calc.p_g.is_output() { calc.p_g = Number::None; }
        if calc.m_g.is_output() { calc.m_g = Number::None; }
        if calc.n_g.is_output() { calc.n_g = Number::None; }
        if calc.p_g_v.is_output() { calc.p_g_v = Number::None; }

        calc
    }
}
