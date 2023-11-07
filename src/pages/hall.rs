use yew::prelude::*;

use crate::artifact::{ArticleComponent, BuiltYaml, ExhibitionHall};
use crate::html_utils::scroll::try_scroll_to;
use crate::pages::hall_components::HallNav;

pub struct HallComponent {
    active_hall: Option<ExhibitionHall>,
    menu_active: bool,
}

#[derive(Properties, PartialEq)]
pub struct HallProps {}

pub enum HallMsg {
    GoToHall(Option<ExhibitionHall>),
}

static HALLROOT: &str = "hall_entrance";

impl Component for HallComponent {
    type Message = HallMsg;
    type Properties = HallProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            active_hall: None,
            menu_active: false,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HallMsg::GoToHall(hall) => {
                self.active_hall = hall;
                self.menu_active = false;
                try_scroll_to(HALLROOT);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // iterate through all files under ../artifacts/ with .yaml extension at compile time
        let yaml = include_str!("../artifacts/build/compiled.yaml");
        let built_yaml: BuiltYaml = serde_yaml::from_str(yaml).unwrap();

        let hall_name = match &self.active_hall {
            Some(hall) => hall.to_string(),
            None => "The Everything Hall".to_string(),
        };
        let desc = match &self.active_hall {
            Some(hall) => hall.desc(),
            None => "You're currently viewing all available artifacts. Select a hall (🏛️) at bottom left to view its artifacts.",
        };

        let mut loaded_articles = built_yaml.artifacts;
        loaded_articles.sort_by(|a, b| a.language.cmp(&b.language));
        loaded_articles.sort_by(|a, b| a.status.cmp(&b.status));
        let articles = loaded_articles
            .into_iter()
            .filter(|article| match (&article.hall, &self.active_hall) {
                (Some(hall), Some(active_hall)) => hall == active_hall,
                (_, None) => true,
                (None, _) => false,
            })
            .map(|article| {
                html! {
                    <ArticleComponent article={article}/>
                }
            })
            .collect::<Vec<Html>>();

        let emitter = ctx.link().clone();
        let nav_cb = Callback::from(move |msg| emitter.send_message(msg));

        html! {
            <>
            <HallNav active_hall_name={hall_name.clone()} hall_cb={nav_cb}/>

            // menu block for mobile devices
            <div class="z-10 fixed w-full h-20 bottom-0 bg-black/80 md:hidden"/>

            <div class="ease-in bg-black h-full">
                <div id={HALLROOT} class="text-center">
                    <div class="py-6 text-white text-lg md:text-xl lg:text-2xl">
                        {hall_name}
                    </div>
                    <div class="mx-auto px-4 py-6 max-w-xl text-gray-200 text-sm md:text-base">
                        {desc}
                    </div>
                </div>
                <ul role="list" class="select-none text-white px-4 md:px-40 md:py-10 divide-y divide-gray-800">
                    {articles}
                </ul>

                <div class="my-20 mx-12 text-center text-gray-300 space-y-4">
                    <p>{"You've reached the end of the this room."}</p>
                    <p>{"Hope you enjoyed your visit!"}</p>
                    <p>{"If you want to see more, check out the other halls (🏛️)!"}</p>
                </div>

                <div class="mt-36 mb-20 text-center text-gray-600">
                {"Museum of Code © 2023, All Rights Reserved"}
                </div>
            </div>
            </>
        }
    }
}
