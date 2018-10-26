// The MIT License (MIT)
// =====================

// Copyright (c) 2018 KITAPLATFORM

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use yew::{html, html_impl, prelude::*};

pub enum ButtonMsg {
    Clicked,
}

#[derive(Clone, PartialEq, Default)]
pub struct ButtonProps {
    pub title: String,
    pub on_click: Option<Callback<()>>,
}

pub struct Button {
    title: String,
    on_click: Option<Callback<()>>,
}

impl Component for Button {
    type Message = ButtonMsg;
    type Properties = ButtonProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            title: props.title,
            on_click: props.on_click,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ButtonMsg::Clicked => {
                if let Some(ref on_click) = self.on_click {
                    on_click.emit(())
                }
            },
        }

        false
    }
}

impl Renderable<Button> for Button {
    fn view(&self) -> Html<Self> {
        html! {
            <button type="button",
                    onclick=|_| ButtonMsg::Clicked,>
                {&self.title}
            </button>
        }
    }
}
