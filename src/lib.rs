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
    pub significant_figures: usize,
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
    IT(String),
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
            Msg::IT(_) => Msg::IT(str.into()),
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
            significant_figures: 10,
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
            Msg::IT(s) => self.calc.i_t = Num::parse(s),
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
        let u = self.field(
            "u", "U", "", "V",
            "Voltage",
            Msg::U("".into()), self.calc.u);

        let i = self.field(
            "i", "I", "", "A",
            "Current",
            Msg::I("".into()), self.calc.i);

        let r_a = self.field(
            "r_a", "R", "A", "Ω",
            "Armature resistance",
            Msg::RA("".into()), self.calc.r_a);

        let p_in = self.field(
            "p_in", "P", "In", "W",
            "Input power",
            Msg::PIn("".into()), self.calc.p_in);

        let p_m = self.field(
            "p_m", "P", "M", "W",
            "Motor power",
            Msg::PM("".into()), self.calc.p_m);

        let p_m_l = self.field(
            "p_ml", "P", "ML", "W",
            "Motor power loss",
            Msg::PMV("".into()), self.calc.p_m_l);

        let p_m_l_el = self.field(
            "p_ml_el", "P", "ML_el", "W",
            "Electrical motor power loss",
            Msg::PMVEl("".into()), self.calc.p_m_l_el);

        let p_m_l_mech = self.field(
            "p_ml_mech", "P", "ML_mech", "W",
            "Mechanical motor power loss",
            Msg::PMVMech("".into()), self.calc.p_m_l_mech);

        let eta_m = self.field(
            "eta_m", "η", "M", "%",
            "Motor efficiency",
            Msg::ETAM("".into()), self.calc.eta_m);

        let m_m = self.field(
            "m_m", "M", "M", "Nm",
            "Motor torque",
            Msg::MM("".into()), self.calc.m_m);

        let n_m = self.field(
            "n_m", "n", "M", "rpm",
            "Motor speed",
            Msg::NM("".into()), self.calc.n_m);

        let i_t = self.field(
            "i_t", "i", "", "%",
            "Transmission ratio",
            Msg::IT("".into()), self.calc.i_t);

        let p_t = self.field(
            "p_t", "P", "T", "W",
            "Transmission power",
            Msg::PT("".into()), self.calc.p_t);

        let p_t_l = self.field(
            "p_tl", "P", "TL", "W",
            "Transmission power loss",
            Msg::PTV("".into()), self.calc.p_t_l);

        let eta_t = self.field(
            "eta_t", "η", "T", "%",
            "Transmission efficiency",
            Msg::ETAT("".into()), self.calc.eta_t);

        let m_t = self.field(
            "m_t", "M", "T", "Nm",
            "Transmission torque",
            Msg::MT("".into()), self.calc.m_t);

        let n_t = self.field(
            "n_t", "n", "T", "rpm",
            "Transmission speed",
            Msg::NT("".into()), self.calc.n_t);

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
                    { i_t }
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
    pub fn field(&self, id: &str, label: &str, sub_label: &str, unit: &str, description: &str, msg_type: Msg, num: Num) -> Html {
        html! {
            <div class={ id } >
                <label for={ id }
                    title={ description }>
                    { label }<sub>{ sub_label }</sub>{ format!(" [{}]", unit) }
                </label>
                <div class="input-output">
                    <input class="edit"
                        type="text"
                        id={ id }
                        oninput=self.link.callback(move |e: InputData| msg_type.with(e.value))
                        disabled={ num.is_output() }
                        />
                    <span class="display">{ if num.is_output() { num.display(self.significant_figures) } else { "".into() } }</span>
                </div>
            </div>
        }
    }
}
