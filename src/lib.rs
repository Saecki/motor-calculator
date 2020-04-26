#![recursion_limit = "2048"]

use yew::prelude::*;

use crate::calc::calculation::Calculation;
use crate::calc::number::Num;

mod error;

pub mod calc {
    pub mod calculation;
    pub mod equation;
    pub mod number;
    pub mod operation;
}

#[derive(Clone, Debug)]
pub struct Model {
    pub link: ComponentLink<Self>,
    pub calc: Calculation,
}

#[derive(Clone, Debug)]
pub enum Msg {
    U(String),
    I(String),
    RA(String),
    PIn(String),
    PM(String),
    PMV(String),
    PMVEl(String),
    PMVMech(String),
    ETAM(String),
    MM(String),
    NM(String),
    PT(String),
    PTV(String),
    ETAT(String),
    MT(String),
    NT(String),
}

impl Msg {
    fn with(&self, str: impl Into<String>) -> Msg {
        match self {
            Msg::U(_) => Msg::U(str.into()),
            Msg::I(_) => Msg::I(str.into()),
            Msg::RA(_) => Msg::RA(str.into()),
            Msg::PIn(_) => Msg::PIn(str.into()),
            Msg::PM(_) => Msg::PM(str.into()),
            Msg::PMV(_) => Msg::PMV(str.into()),
            Msg::PMVEl(_) => Msg::PMVEl(str.into()),
            Msg::PMVMech(_) => Msg::PMVMech(str.into()),
            Msg::ETAM(_) => Msg::ETAM(str.into()),
            Msg::MM(_) => Msg::MM(str.into()),
            Msg::NM(_) => Msg::NM(str.into()),
            Msg::PT(_) => Msg::PT(str.into()),
            Msg::PTV(_) => Msg::PTV(str.into()),
            Msg::ETAT(_) => Msg::ETAT(str.into()),
            Msg::MT(_) => Msg::MT(str.into()),
            Msg::NT(_) => Msg::NT(str.into()),
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            calc: Calculation::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::U(s) => self.calc.u = Num::parse(s),
            Msg::I(s) => self.calc.i = Num::parse(s),
            Msg::RA(s) => self.calc.r_a = Num::parse(s),
            Msg::PIn(s) => self.calc.p_in = Num::parse(s),
            Msg::PM(s) => self.calc.p_m = Num::parse(s),
            Msg::PMV(s) => self.calc.p_m_l = Num::parse(s),
            Msg::PMVEl(s) => self.calc.p_m_l_el = Num::parse(s),
            Msg::PMVMech(s) => self.calc.p_m_l_mech = Num::parse(s),
            Msg::ETAM(s) => self.calc.eta_m = Num::parse(s),
            Msg::MM(s) => self.calc.m_m = Num::parse(s),
            Msg::NM(s) => self.calc.n_m = Num::parse(s),
            Msg::PT(s) => self.calc.p_t = Num::parse(s),
            Msg::PTV(s) => self.calc.p_t_l = Num::parse(s),
            Msg::ETAT(s) => self.calc.eta_t = Num::parse(s),
            Msg::MT(s) => self.calc.m_t = Num::parse(s),
            Msg::NT(s) => self.calc.n_t = Num::parse(s),
        }

        if let Ok(c) = self.calc.try_fill_missing() {
            self.calc = c;
        }

        true
    }


    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let u = self.field("u", "U", "", "V", Msg::U("".into()), self.calc.u);
        let i = self.field("i", "I", "", "A", Msg::I("".into()), self.calc.i);
        let r_a = self.field("r_a", "R", "A", "Ω", Msg::RA("".into()), self.calc.r_a);
        let p_in = self.field("p_in", "P", "In", "W", Msg::PIn("".into()), self.calc.p_in);
        let p_m = self.field("p_m", "P", "M", "W", Msg::PM("".into()), self.calc.p_m);
        let p_m_l = self.field("p_ml", "P", "ML", "W", Msg::PMV("".into()), self.calc.p_m_l);
        let p_m_l_el = self.field("p_ml_el", "P", "ML_el", "W", Msg::PMVEl("".into()), self.calc.p_m_l_el);
        let p_m_l_mech = self.field("p_ml_mech", "P", "ML_mech", "W", Msg::PMVMech("".into()), self.calc.p_m_l_mech);
        let eta_m = self.field("eta_m", "η", "M", "%", Msg::ETAM("".into()), self.calc.eta_m);
        let m_m = self.field("m_m", "M", "M", "Nm", Msg::MM("".into()), self.calc.m_m);
        let n_m = self.field("n_m", "n", "M", "rpm", Msg::NM("".into()), self.calc.n_m);
        let p_t = self.field("p_t", "P", "T", "W", Msg::PT("".into()), self.calc.p_t);
        let p_t_l = self.field("p_tl", "P", "TL", "W", Msg::PTV("".into()), self.calc.p_t_l);
        let eta_t = self.field("eta_t", "η", "T", "%", Msg::ETAT("".into()), self.calc.eta_t);
        let m_t = self.field("m_t", "M", "T", "Nm", Msg::MT("".into()), self.calc.m_t);
        let n_t = self.field("n_t", "n", "T", "rpm", Msg::NT("".into()), self.calc.n_t);

        html! {
            <div class="motorcalc">
                <header class="heading">
                    <h1>{ "motorcalc" }</h1>
                    <a class="github" href="https://github.com/saecki/motorcalc.git">{ "GitHub" }</a>
                </header>
                <div class="calc">
                    { p_in }
                    { u }
                    { i }
                    { r_a }
                    { p_m_l_el }
                    { p_m_l_mech }
                    { p_m_l }
                    { p_m }
                    { m_m }
                    { n_m }
                    { eta_m }
                    { p_t_l }
                    { p_t }
                    { m_t }
                    { n_t }
                    { eta_t }
                </div>
            </div>
        }
    }
}

impl Model {
    pub fn field(&self, id: &str, label: &str, sub_label: &str, unit: &str, msg_type: Msg, num: Num) -> Html {
        html! {
            <div class={ id } >
                <label for={ id }>{ label }<sub>{ sub_label }</sub>{ format!(" [{}]", unit) }</label>
                <input type="text"
                    id={ id }
                    oninput=self.link.callback(move |e: InputData| msg_type.with(e.value))
                    disabled={ num.is_output() }
                    />
                <span>{ if num.is_output() { num.display() } else { "".into() } }</span>
            </div>
        }
    }
}
