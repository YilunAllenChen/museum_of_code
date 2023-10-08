use std::fmt::Display;

use yew::prelude::*;
mod pages;
mod utils;
use pages::{Contact, Home, Tour, Wip, Nav};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Page {
    Home,
    Contact,
    Tour,
    About,

    Wip,
}

impl Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Page::Home => "Home",
            Page::Contact => "Contact",
            Page::Tour => "Tour",
            Page::About => "About",
            Page::Wip => "WIP",
        };
        write!(f, "{}", name)
    }
}

pub struct App {
    active_page: Page,
}

pub enum Msg {
    GoToPage(Page),
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
                // self.active_page = match childs_name.as_str() {
                //     "Home" => Page::Home,
                //     "Contribute" => Page::Contact,
                //     _ => Page::WIP,
                // };
                self.active_page = childs_name;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_clicked = ctx.link().callback(Msg::GoToPage);
        let nav = html! {
            <Nav {on_clicked} />
        };
        let on_clicked = ctx.link().callback(Msg::GoToPage);
        let content = match self.active_page {
            Page::Contact => html! {<Contact {on_clicked} />},
            Page::Home => html! {<Home {on_clicked} />},
            Page::Tour => html! {<Tour {on_clicked} />},
            _ => html! {<Wip {on_clicked} />},
        };
        html! {
        <>
        {nav}
        {content}
        </>}
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
