use yew::prelude::*;

pub struct Model {

}

pub enum Msg {
    Dashboard,
    Redeem,
    Send,
    Exchange
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
            <>
                <main id="content",>
                    <div class="lightTheme", data-reactroot="",>
                        <div id="content-wrapper",>
                            <div>
                                <div class="grid-frame vertical",>
                                    { self.view_header() }
                                    <nav class="menu",>{ self.view_menu() }</nav>
                                </div>
                            </div>
                        </div>
                    </div>
                </main>
            </>
        }
    }
}

impl Model {
    fn view_menu(&self) -> Html<Self> {
        html! {
            <>
                <div class="grid-block",>
                    <div class="grid-block",>
                        <div class="grid-block page-layout",>
                            <div class="show-for-medium-up hide-for-small-only grid-block shrink left-column no-padding", style="min-width: 200px;",>
                                <div class="grid-block vertical account-left-panel no-padding no-overflow",>
                                    <div class="grid-block",>
                                        <div class="grid-content no-padding", style="overflow-x: hidden;",>
                                            { self.view_left_menu() }
                                        </div>
                                    </div>
                                </div>
                            </div>
                            { self.view_main_content() }
                        </div>
                    </div>
                </div>
            </>
        }
    }

    fn view_login_name(&self) -> Html<Self> {
        html! {
            <>
                </div>
                <div class="grid-block shrink overflow-visible account-drop-down",>
                    <div class="action-sheet-container", data-closable="true",>
                        <div>
                            <div>
                                <a class="button", style="margin-right: 50px; padding: 0px 9px 0px 0px; color: rgb(70, 70, 70); line-height: 60px; font-family: SansProRegular;",>
                                    { "s-bryyleva" }
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
    fn view_header(&self) -> Html<Self> {
        html! {
            <>
                <div class="header menu-group primary",>
                    <div class="grid-block",>
                        <ul class="menu-bar",>
                            <li>
                                <a>{ "Kita fund" }</a>
                            </li>
                        </ul>
                    </div>
                    <div class="grid-block show_for-medium shrink",>
                        <div class="grp-menu-items-group header-right-menu",>
                            <div style="width: 47px; height: 26px; margin: 17px 10px 0px 0px; font-size: 14px; color: rgb(87, 87, 87); font-family: SansProRegular;",>
                                { "Your balance" }
                            </div>
                            <div class="grp-menu-item", style="height: 35px; margin-top: 12.5px; padding: 10px 18px; line-height: 60px; border: 1px solid rgb(163, 226, 178); background-color: rgb(235, 252, 239); font-size: 18px; color: rgb(36, 102, 52); border-radius: 17.5px; font-family: SansProRegular;",>
                                <span>
                                    { "0.01" }
                                     <strong> { "USD" }</strong>
                                </span>
                            </div>
                            <div style="background-color: rgb(235, 235, 235); width: 1px; height: 30px; margin: 15px;",>
                                { self.view_login_name() }
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }

    fn view_left_menu(&self) -> Html<Self> { // .block-list.dashboard-panel
        html! {
            <>
                <section class="block-list dashboard-panel",>
                    <ul class="account-left-menu", style="margin-botom: 0px; font-family: SansProSemiBold;",>
                        <li>
                            <button class="active", onclick=|_| Msg::Dashboard,>{ "Dashboard" }</button>
                        </li>
                        <li>
                            <button class="active", onclick=|_| Msg::Redeem,>{ "Redeem" }</button>
                        </li>
                        <li>
                            <button class="active", onclick=|_| Msg::Send,>{ "Send" }</button>
                        </li>
                        <li>
                            <button class="active", onclick=|_| Msg::Exchange,>{ "Exchange" }</button>
                        </li>
                    </ul>
                </section>
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