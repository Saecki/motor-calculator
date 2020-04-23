use yew;

use crate::view::Model;

mod view;
mod error;
mod calc;
mod equation;

fn main() {
    yew::start_app::<Model>();
}