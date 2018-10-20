use yew::prelude::*;

pub struct Model {

}

pub enum Msg {
    Dashboard,
    Redeem,
    Send,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Dashboard => { true },
            _ => { false },
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
                <div class="grid-container",>
                    <header>{ self.view_header() }</header>
                    <div class="item-logo",><strong>{ "Kita•Fund" }</strong></div>
                    { self.view_menu() }
                    <main>{ "Main" }</main>
                </div>
        }
    }
}

impl Model {
    fn view_menu(&self) -> Html<Self> {
        html! {
            <>
                <div class="item-menu",>
                    <ul>
                        <li>
                            <button class="active", onclick=|_| Msg::Dashboard,><b>{ "Dashboard" }</b></button>
                        </li>
                        <li>
                            <button class="active", onclick=|_| Msg::Redeem,><b>{ "Redeem" }</b></button>
                        </li>
                        <li>
                            <button class="active", onclick=|_| Msg::Send,><b>{ "Send" }</b></button>
                        </li>
                    </ul>
                </div>
            </>
        }
    }

    fn view_login_name(&self) -> Html<Self> {
        html! {
            <>
                <div>
                    <button style="display: inline-block; text-align: left;",>
                        { "username" }
                    </button>
                </div>
            </>
        }
    }
    fn view_header(&self) -> Html<Self> {
        html! {
            <>
                <div style="display: inline-block; float: right; padding-right: 5px; padding-left: 5px;",>
                    <a style="display: inline-block; text-align: left; float: right; padding-right: 5px; padding-left: 5px;",>
                        { "username" }
                    </a>
                </div>
                <div style="display: inline-block; float: right; padding-right: 5px; padding-left: 5px; border: 1px solid rgb(163, 226, 178); background-color: rgb(235, 252, 239); font-size: 18px; color: rgb(36, 102, 52); border-radius: 17.5px; font-family: SansProRegular;",>
                    <span>
                        { "0.01" }
                            <strong> { "USD" }</strong>
                    </span>
                </div>
                <div style="display: inline-block; float: right; padding-right: 10px; padding-left: 10px;",>
                    { "Your balance" }
                </div>
</>
        }
    }

    fn view_main_content(&self) -> Html<Self> {
        html! {
            <>
                <div class="grid-block main-content",>
                    <div class="grid-container", style="padding-top: 15px;",>
                        <div class="grid-content", style="overflow-y: visible;",>
                            <div class="content-block small-12",>
                                <div class="generic-bordered-box",>
                                    <div class="block-content-header",>
                                        <div class="",>
                                            { "Balances" }
                                        </div>
                                    </div>
                                    { self.view_table_of_main_content() }
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }

    fn view_table_of_main_content(&self) -> Html<Self> {
        html! {
            <table class="table",>
                <thead>
                    <tr>
                        <th>
                            <span>{ "Asset" }</span>
                        </th>
                        <th>
                            <span>{ "Amount" }</span>
                        </th>
                        <th>
                            <span>{ "Price per one" }</span>
                        </th>
                        <th>
                            <span>{ "Equivalent Value" }</span>
                        </th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <th>
                            <span>
                                <span>{ "SPIES" }</span>
                            </span>
                        </th>
                        <th>
                            <span>
                                <span>{ "0.0001" }</span>
                            </span>
                        </th>
                        <th>
                            <span>
                                <span>{ "$53.51" }</span>
                            </span>
                        </th>
                        <th>
                            <span>
                                <span>{ "$0.01" }</span>
                            </span>
                        </th>
                    </tr>
                </tbody>
            </table>
        }
    }
}