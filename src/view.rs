use yew::prelude::*;

pub struct Model {
    link: ComponentLink<Self>,
    val1: Option<f64>,
    val2: Option<f64>,
    value: f64,
}

pub enum Msg {
    Val1(String),
    Val2(String),
    Add,
    Sub,
    Mul,
    Div,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            val1: Some(23423_f64),
            val2: Some(23423_f64),
            link,
            value: 0_f64,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if let (Some(a), Some(b)) = (self.val1, self.val2) {
            match msg {
                Msg::Val1(s) => self.val1 = if let Ok(v) = s.parse::<f64>().into() { Some(v) } else { None },
                Msg::Val2(s) => self.val2 = if let Ok(v) = s.parse::<f64>().into() { Some(v) } else { None },
                Msg::Add => self.value = a + b,
                Msg::Sub => self.value = a - b,
                Msg::Mul => self.value = a * b,
                Msg::Div => self.value = a / b,
            }

            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="motor-calculator">
                <input class="edit"
                    type="text"
                    oninput=self.link.callback(|e: InputData| Msg::Val1(e.value))
                    />
                <input class="edit"
                    type="text"
                    oninput=self.link.callback(|e: InputData| Msg::Val2(e.value))
                    />
                <button onclick=self.link.callback(|_| Msg::Add)>{ "+" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}
