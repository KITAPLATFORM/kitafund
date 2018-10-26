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

pub enum BalancesViewMsg {}

#[derive(Clone, PartialEq, Default)]
pub struct BalancesViewProps;

pub struct BalancesView;

impl Component for BalancesView {
    type Message = BalancesViewMsg;
    type Properties = BalancesViewProps;

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<BalancesView> for BalancesView {
    fn view(&self) -> Html<Self> {
        html! {
            <>
                <div>{"Balances"}</div>
                <div class="grid balances_view",>
                    <div>{"Asset"}</div>
                    <div>{"Amount"}</div>
                    <div>{"Price per one"}</div>
                    <div>{"Equivalent value"}</div>
                    <div>{"BTC"}</div>
                    <div>{"1.67"}</div>
                    <div>{"$6,045.67"}</div>
                    <div>{"$10096.2689"}</div>
                    <div>{"ETH"}</div>
                    <div>{"3.45"}</div>
                    <div>{"$245.34"}</div>
                    <div>{"$846.423"}</div>
                </div>
            </>
        }
    }
}
