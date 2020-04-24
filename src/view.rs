use yew::prelude::*;

use crate::calculation::Calculation;
use crate::number::Num;

pub struct Model {
    link: ComponentLink<Self>,
    calc: Calculation,
}

pub enum Msg {
    U(String),
    I(String),
    RA(String),
    PIn(String),
    PM(String),
    PMV(String),
    PMVEl(String),
    PMVMech(String),
    MM(String),
    NM(String),
    PT(String),
    PTV(String),
    MT(String),
    NT(String),
}

impl Model {
    fn calculate(&mut self) {
        if let Ok(c) = self.calc.try_fill_missing() {
            self.calc = c;
        }
    } }
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
            Msg::PMV(s) => self.calc.p_m_v = Num::parse(s),
            Msg::PMVEl(s) => self.calc.p_m_v_el = Num::parse(s),
            Msg::PMVMech(s) => self.calc.p_m_v_mech = Num::parse(s),
            Msg::MM(s) => self.calc.m_m = Num::parse(s),
            Msg::NM(s) => self.calc.n_m = Num::parse(s),
            Msg::PT(s) => self.calc.p_t = Num::parse(s),
            Msg::PTV(s) => self.calc.p_t_v = Num::parse(s),
            Msg::MT(s) => self.calc.m_t = Num::parse(s),
            Msg::NT(s) => self.calc.n_t = Num::parse(s),
        }

        self.calculate();

        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="motor-calculator">
                <label for="u">{ "U" }</label>
                <input class="edit"
                    style="margin: auto;"
                    type="text"
                    id="u"
                    oninput=self.link.callback(|e: InputData| Msg::U(e.value))
                    />
                <span>{ self.calc.u.display() }</span>
                <p/>
                <label for="i">{ "I" }</label>
                <input class="edit"
                    type="text"
                    style="margin: auto;"
                    id="i"
                    oninput=self.link.callback(|e: InputData| Msg::I(e.value))
                    />
                <span>{ self.calc.i.display() }</span>
                <p/>
                <label for="r_a">{ "R" }<sub>{ "A"}</sub></label>
                <input class="edit"
                    type="text"
                    id="r_a"
                    oninput=self.link.callback(|e: InputData| Msg::RA(e.value))
                    />
                <span>{ self.calc.r_a.display() }</span>
                <p/>
                <label for="p_in">{ "P" }<sub>{ "in"}</sub></label>
                <input class="edit"
                    type="text"
                    id="p_in"
                    oninput=self.link.callback(|e: InputData| Msg::PIn(e.value))
                    />
                <span>{ self.calc.p_in.display() }</span>
                <p/>
                <label for="p_m">{ "P" }<sub>{ "M"}</sub></label>
                <input class="edit"
                    type="text"
                    id="p_m"
                    oninput=self.link.callback(|e: InputData| Msg::PM(e.value))
                    />
                <span>{ self.calc.p_m.display() }</span>
                <p/>
                <label for="p_mv">{ "P" }<sub>{ "MV"}</sub></label>
                <input class="edit"
                    type="text"
                    id="p_mv"
                    oninput=self.link.callback(|e: InputData| Msg::PMV(e.value))
                    />
                <span>{ self.calc.p_m_v.display() }</span>
                <p/>
                <label for="p_mv_el">{ "P" }<sub>{ "MV_el"}</sub></label>
                <input class="edit"
                    type="text"
                    id="p_mv_el"
                    oninput=self.link.callback(|e: InputData| Msg::PMVEl(e.value))
                    />
                <span>{ self.calc.p_m_v_el.display() }</span>
                <p/>
                <label for="p_mv_mech">{ "P" }<sub>{ "MV_mech"}</sub></label>
                <input class="edit"
                    type="text"
                    id="p_mv_mech"
                    oninput=self.link.callback(|e: InputData| Msg::PMVMech(e.value))
                    />
                <span>{ self.calc.p_m_v_mech.display() }</span>
                <p/>
                <label for="m_m">{ "M" }<sub>{ "M"}</sub></label>
                <input class="edit"
                    type="text"
                    id="m_m"
                    oninput=self.link.callback(|e: InputData| Msg::MM(e.value))
                    />
                <span>{ self.calc.m_m.display() }</span>
                <p/>
                <label for="n_m">{ "N" }<sub>{ "M"}</sub></label>
                <input class="edit"
                    type="text"
                    id="n_m"
                    oninput=self.link.callback(|e: InputData| Msg::NM(e.value))
                    />
                <span>{ self.calc.n_m.display() }</span>
                <p/>
                <label for="p_t">{ "P" }<sub>{ "T"}</sub></label>
                <input class="edit"
                    type="text"
                    id="p_t"
                    oninput=self.link.callback(|e: InputData| Msg::PT(e.value))
                    />
                <span>{ self.calc.p_t.display() }</span>
                <p/>
                <label for="p_t_v">{ "P" }<sub>{ "TV"}</sub></label>
                <input class="edit"
                    type="text"
                    id="p_t_v"
                    oninput=self.link.callback(|e: InputData| Msg::PTV(e.value))
                    />
                <span>{ self.calc.p_t_v.display() }</span>
                <p/>
                <label for="m_t">{ "M" }<sub>{ "T"}</sub></label>
                <input class="edit"
                    type="text"
                    id="m_t"
                    oninput=self.link.callback(|e: InputData| Msg::MT(e.value))
                    />
                <span>{ self.calc.m_t.display() }</span>
                <p/>
                <label for="n_t">{ "N" }<sub>{ "T"}</sub></label>
                <input class="edit"
                    type="text"
                    id="n_t"
                    oninput=self.link.callback(|e: InputData| Msg::NT(e.value))
                    />
                <span>{ self.calc.n_t.display() }</span>
            </div>
        }
    }
}
