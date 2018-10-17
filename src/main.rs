#[macro_use]
extern crate yew;

mod fragment;

use yew::prelude::*;
use self::fragment::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}