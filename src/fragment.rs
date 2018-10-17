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
                { self.view_header() }
                <nav class="menu",>{ self.view_menu() }</nav>
                
            </>
        }
    }
}

impl Model {
    fn view_menu(&self) -> Html<Self> {
        html! {
            <>
                <ul>
                    <li>
                        <button onclick=|_| Msg::Dashboard,>{ "Dashboard" }</button>
                    </li>
                    <li>
                        <button onclick=|_| Msg::Redeem,>{ "Redeem" }</button>
                    </li>
                    <li>
                        <button onclick=|_| Msg::Send,>{ "Send" }</button>
                    </li>
                    <li>
                        <button onclick=|_| Msg::Exchange,>{ "Exchange" }</button>
                    </li>
                </ul>
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
                    
                    </div>
                </div>
            </>
        }
    }
}