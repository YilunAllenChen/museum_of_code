use yew::prelude::*;

use crate::code::ArticleComponent;
use crate::code::BuiltYaml;
use crate::code::ExhibitionHall;
use crate::html_utils::scroll::try_scroll_to;

pub struct HallComponent {
    active_hall: Option<ExhibitionHall>,
    menu_active: bool,
}

#[derive(Properties, PartialEq)]
pub struct HallProps {}

pub enum HallMsg {
    ToggleMenu,
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
            HallMsg::ToggleMenu => {
                self.menu_active = !self.menu_active;
                true
            }
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

        let menu = match self.menu_active {
            false => html! {},
            true => {
                let page_buttons = vec![
                    ExhibitionHall::HallOfHelloWorld,
                    ExhibitionHall::HallOfExpressiveness,
                    ExhibitionHall::HallOfSpeed,
                    ExhibitionHall::HallOfTroll,
                    ExhibitionHall::Uncategorized,
                ].into_iter().map(|hall|{
                    let hall_name = hall.to_string();
                    html! {
                        <button
                            class="block w-full text-left md:pl-8 my-2 py-2 text:base md:text-xl text-gray-400"
                            role="menuitem"
                            onclick={ctx.link().callback(move |_| HallMsg::GoToHall(Some(hall.clone())))}
                        >
                            {hall_name}
                        </button>
                    }
                }).collect::<Html>();

                html! {
                    <div class="relative z-40" aria-labelledby="modal-title" role="dialog" aria-modal="true">
                    <div class="fixed inset-0 bg-gray-500 bg-opacity-25 transition-opacity"></div>
                    <div class="fixed inset-0 z-50 w-screen overflow-y-auto">
                      <div class="flex enter-exit-bottom min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                        <div class="w-full overflow-hidden rounded-lg text-left sm:max-w-2xl">
                          <div class="bg-black px-4 pb-4 pt-5 sm:p-6 sm:pb-4">
                            <div class="md:mx-12">
                                <div class="text-lg text-gray-300">
                                    {"Now in: "}{hall_name.clone()}
                                </div>
                                <div class="text-sm text-gray-500 mb-4">
                                    {desc}
                                </div>
                            </div>
                            <div class="mt-3 sm:mt-0 w-full divide-y divide-gray-700 space-y-1">
                                {page_buttons}
                            </div>

                            <div class="mx-auto md:max-w-md">
                                <button
                                    class="block w-full mt-10 text-center p-2 text-xl text-gray-300 bg-gray-800 rounded-lg"
                                    role="menuitem"
                                    onclick={ctx.link().callback(move |_| HallMsg::GoToHall(None))}
                                >
                                    {"View All Artifacts"}
                                </button>
                                <button
                                    type="button"
                                    class="text-center mt-4 md:mb-4 w-full rounded-md bg-red-100 px-3 py-2 text-sm text-black hover:bg-red-500"
                                    onclick={ctx.link().callback(|_| HallMsg::ToggleMenu)}
                                    >
                                {"❌"}
                                </button>
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                }
            }
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

        let menu_button = match self.menu_active {
            true => html! {""},
            false => html!(
                <div class="select-none z-30 enter-exit-bottom fixed bottom-1 left-16 md:bottom-4 md:left-20 ">
                <div class="flex-none rounded-full bg-cyan-500/20 p-1">
                    <button
                        class="w-12 h-12 bg-cyan-500 text-white rounded-full text-xl flex items-center justify-center"
                        onclick={ctx.link().callback(|_| HallMsg::ToggleMenu)}>
                    {"🏛️"}
                    </button>
                </div>
                </div>
            ),
        };

        html! {
            <>
            {menu}
            {menu_button}

            // menu block for mobile devices
            <div class="z-10 fixed w-full h-16 bottom-0 bg-black/80 md:hidden"/>

            <div class="ease-in bg-black h-full">
                <div id={HALLROOT} class="text-center">
                    <div class="py-6 text-white text-lg md:text-xl lg:text-2xl">
                        {hall_name}
                    </div>
                    <div class="mx-auto px-4 py-6 max-w-xl text-gray-200 text-sm md:text-base">
                        {desc}
                    </div>
                </div>
                <ul role="list" class="text-white px-4 md:px-40 md:py-10 divide-y divide-gray-800">
                    {articles}
                </ul>

                <div class="my-20 mx-12 text-center text-gray-300 space-y-4">
                    <p>{"You've reached the end of the this room."}</p>
                    <p>{"Hope you enjoyed your visit!"}</p>
                    <p>{"If you want to see more, check out the other halls!"}</p>
                </div>

                <div class="mt-36 mb-20 text-center text-gray-600">
                {"Museum of Code © 2023, All Rights Reserved"}
                </div>
            </div>
            </>
        }
    }
}
