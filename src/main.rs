#![recursion_limit = "2048"]

use yew;

use crate::view::Model;

mod calculation;
mod equation;
mod error;
mod number;
mod operation;
mod view;

fn main() {
    yew::start_app::<Model>();
}