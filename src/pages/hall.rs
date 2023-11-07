use yew::prelude::*;

use crate::artifact::{ArticleComponent, BuiltYaml, ExhibitionHall};
use crate::html_utils::render_text_tag;
use crate::html_utils::scroll::try_scroll_to;
use crate::pages::hall_components::HallNav;

pub struct HallComponent {
    active_hall: Option<ExhibitionHall>,
    menu_active: bool,
    filter_tags: Vec<String>,
}

#[derive(Properties, PartialEq)]
pub struct HallProps {}

pub enum HallMsg {
    GoToHall(Option<ExhibitionHall>),
    ToggleTag(String),
    ClearTags,
}

static HALLROOT: &str = "hall_entrance";

impl Component for HallComponent {
    type Message = HallMsg;
    type Properties = HallProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            active_hall: None,
            menu_active: false,
            filter_tags: vec![],
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
            HallMsg::ToggleTag(tag) => {
                if self.filter_tags.contains(&tag) {
                    self.filter_tags.retain(|t| t != &tag);
                } else {
                    self.filter_tags.push(tag);
                }
                try_scroll_to(HALLROOT);
                true
            }
            HallMsg::ClearTags => {
                self.filter_tags = vec![];
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
            None => "You're currently viewing all artifacts. Select a hall (🏛️), or click on the tags to filter the artifacts."
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
            .filter(|article| match &self.filter_tags.len() {
                0 => true,
                _ => {
                    let mut article_tags = article.tags.clone();
                    article_tags.push(article.language.to_string());
                    self.filter_tags
                        .iter()
                        .all(|tag| article_tags.contains(tag))
                }
            })
            .map(|article| {
                let emitter = ctx.link().clone();
                let hall_cb = Callback::from(move |msg| emitter.send_message(msg));
                html! {
                    <ArticleComponent {hall_cb}  article={article}/>
                }
            })
            .collect::<Vec<Html>>();

        let tips = match articles.len() {
            0 => html! {
                <div class="my-20 mx-12 text-center text-gray-300 space-y-4">
                    <p>{"This room seems to be empty 🤔..."}</p>
                    <p>{"Did you forget to clear your filter (🏷️)?"}</p>
                </div>
            },
            _ => html! {
                <div class="my-20 mx-12 text-center text-gray-300 space-y-4">
                    <p>{"You've reached the end of the this room."}</p>
                    <p>{"Hope you enjoyed your visit!"}</p>
                    <p>{"If you want to see more, check out the other halls (🏛️)!"}</p>
                </div>
            },
        };

        let emitter = ctx.link().clone();
        let nav_cb = Callback::from(move |msg| emitter.send_message(msg));

        let clear_filter_button = match self.filter_tags.len() {
            0 => html! {},
            _ => {
                let clear_individual_tags = self
                    .filter_tags
                    .iter()
                    .map(|tag| {
                        let tagc= tag.clone();
                        html! {
                            <div
                                onclick={ctx.link().callback(move |_| HallMsg::ToggleTag(tagc.clone()))}
                            >
                            {
                                Html::from_html_unchecked(
                                    render_text_tag(tag).into()
                                )
                            }
                            </div>
                        }
                    })
                    .collect::<Html>();
                html! {
                    <div class="select-none z-30 animate-enter-bottom fixed left-36 bottom-4">
                    <div class="flex gap-x-2">
                        <div class="flex-none rounded-full bg-indigo-500/20 p-1">
                            <button
                                class="w-12 h-12 bg-indigo-500 text-white rounded-full text-2xl flex items-center justify-center"
                                onclick={ctx.link().callback(|_| HallMsg::ClearTags)}>
                            {"🏷️"}
                            </button>
                        </div>
                        <div class="grid grid-rows-2 grid-flow-col gap-0">
                            {clear_individual_tags}
                        </div>
                    </div>
                    </div>
                }
            }
        };

        html! {
            <>
            <HallNav active_hall_name={hall_name.clone()} hall_cb={nav_cb}/>
            {clear_filter_button}

            <div class="z-10 fixed w-full h-20 bottom-0 bg-black/80"/>

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

                {tips}

                <div class="mt-36 mb-20 text-center text-gray-600">
                {"Museum of Code © 2023, All Rights Reserved"}
                </div>
            </div>
            </>
        }
    }
}
