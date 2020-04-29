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

/// An enum representing the messages sent from the UI.
#[derive(Clone, Debug)]
pub enum Msg {
    Calc(&'static str, String),
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
            Msg::Calc(id, s) => {
                match id {
                    "u" => self.calc.u = Num::parse(s),
                    "i" => self.calc.i = Num::parse(s),
                    "r_a" => self.calc.r_a = Num::parse(s),
                    "p_in" => self.calc.p_in = Num::parse(s),
                    "p_m" => self.calc.p_m = Num::parse(s),
                    "p_m_l" => self.calc.p_m_l = Num::parse(s),
                    "p_m_l_el" => self.calc.p_m_l_el = Num::parse(s),
                    "p_m_l_mech" => self.calc.p_m_l_mech = Num::parse(s),
                    "eta_m" => self.calc.eta_m = Num::parse(s),
                    "m_m" => self.calc.m_m = Num::parse(s),
                    "n_m" => self.calc.n_m = Num::parse(s),
                    "p_t" => self.calc.p_t = Num::parse(s),
                    "p_t_l" => self.calc.p_t_l = Num::parse(s),
                    "eta_t" => self.calc.eta_t = Num::parse(s),
                    "m_t" => self.calc.m_t = Num::parse(s),
                    "n_t" => self.calc.n_t = Num::parse(s),
                    "i_t" => self.calc.i_t = Num::parse(s),
                    _ => (),
                }

                if let Ok(c) = self.calc.try_fill_missing() {
                    self.calc = c;
                }
            }
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
            self.calc.u);

        let i = self.field(
            "i", "I", "", "A",
            "Current",
            self.calc.i);

        let r_a = self.field(
            "r_a", "R", "A", "Ω",
            "Armature resistance",
            self.calc.r_a);

        let p_in = self.field(
            "p_in", "P", "In", "W",
            "Input power",
            self.calc.p_in);

        let p_m = self.field(
            "p_m", "P", "M", "W",
            "Motor power",
            self.calc.p_m);

        let p_m_l = self.field(
            "p_m_l", "P", "ML", "W",
            "Motor power loss",
            self.calc.p_m_l);

        let p_m_l_el = self.field(
            "p_m_l_el", "P", "ML_el", "W",
            "Electrical motor power loss",
            self.calc.p_m_l_el);

        let p_m_l_mech = self.field(
            "p_m_l_mech", "P", "ML_mech", "W",
            "Mechanical motor power loss",
            self.calc.p_m_l_mech);

        let eta_m = self.field(
            "eta_m", "η", "M", "%",
            "Motor efficiency",
            self.calc.eta_m);

        let m_m = self.field(
            "m_m", "M", "M", "Nm",
            "Motor torque",
            self.calc.m_m);

        let n_m = self.field(
            "n_m", "n", "M", "rpm",
            "Motor speed",
            self.calc.n_m);

        let i_t = self.field(
            "i_t", "i", "", "%",
            "Transmission ratio",
            self.calc.i_t);

        let p_t = self.field(
            "p_t", "P", "T", "W",
            "Transmission power",
            self.calc.p_t);

        let p_t_l = self.field(
            "p_t_l", "P", "TL", "W",
            "Transmission power loss",
            self.calc.p_t_l);

        let eta_t = self.field(
            "eta_t", "η", "T", "%",
            "Transmission efficiency",
            self.calc.eta_t);

        let m_t = self.field(
            "m_t", "M", "T", "Nm",
            "Transmission torque",
            self.calc.m_t);

        let n_t = self.field(
            "n_t", "n", "T", "rpm",
            "Transmission speed",
            self.calc.n_t);

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
    /// Returns html representing an input field it's label and a output text span.
    pub fn field(&self, id: &'static str, label: &str, sub_label: &str, unit: &str, description: &str, num: Num) -> Html {
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
                        oninput=self.link.callback(move |e: InputData| Msg::Calc(id, e.value))
                        disabled={ num.is_output() }
                        />
                    <span class="display">{ if num.is_output() { num.display(self.significant_figures) } else { "".into() } }</span>
                </div>
            </div>
        }
    }
}
