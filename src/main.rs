use yew::prelude::*;
mod pages;
mod utils;
use pages::contact::Contact;
use pages::home::Home;
use pages::wip::WIP;

#[derive(Debug)]
pub enum Page {
    Home,
    Contact,
    WIP,
}

pub struct App {
    active_page: Page,
}

pub enum Msg {
    GoToPage(AttrValue),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            active_page: Page::Home,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GoToPage(childs_name) => {
                self.active_page = match childs_name.as_str() {
                    "Home" => Page::Home,
                    "Contribute" => Page::Contact,
                    _ => Page::WIP,
                };
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_clicked = ctx.link().callback(Msg::GoToPage);
        let content = match self.active_page {
            Page::Contact => html! {<Contact {on_clicked} />},
            Page::Home => html! {<Home {on_clicked} />},
            _ => html! {<WIP {on_clicked} />},
        };
        html! {content}
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
