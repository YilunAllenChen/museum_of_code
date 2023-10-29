use std::fmt::Display;

use log::info;
use yew::prelude::*;
use yew_router::prelude::*;

mod code;
mod html_utils;
mod pages;
use pages::{Contact, HallComponent, Home, Nav, Wip};

use crate::pages::About;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/contact")]
    Contact,
    #[at("/exhibition")]
    ExhibitionHall,
    #[at("/about")]
    About,
    #[at("/*_path")]
    Wip { _path: String },
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {<Home />},
        Route::Contact => html! {<Contact/>},
        Route::About => html! {<About />},
        Route::ExhibitionHall => html! {<HallComponent />},
        Route::Wip { _path } => html! {<Wip />},
    }
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct App {
    active_page: Route,
}

pub enum Msg {
    GoToPage(Route),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            active_page: Route::Home,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GoToPage(childs_name) => {
                self.active_page = childs_name;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Nav />
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("Logging initialized");
    yew::Renderer::<App>::new().render();
}
