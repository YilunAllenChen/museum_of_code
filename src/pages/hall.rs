use yew::prelude::*;

use crate::code::ArticleComponent;
use crate::code::BuiltYaml;
use crate::code::ExhibitionHall;
use crate::Page;

pub struct HallComponent {
    active_hall: Option<ExhibitionHall>,
    menu_active: bool,
}

#[derive(Properties, PartialEq)]
pub struct HallProps {
    pub on_clicked: Callback<Page>,
}

pub enum HallMsg {
    ToggleMenu,
    GoToHall(Option<ExhibitionHall>),
}

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
            None => "All Artifacts".to_string(),
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
                            class="block w-full text-center px-4 my-2 py-2 text-xl leading-5 text-gray-400 hover:bg-gray-300 hover:text-gray-900 focus:outline-none focus:bg-gray-100 focus:text-gray-900"
                            role="menuitem"
                            onclick={ctx.link().callback(move |_| HallMsg::GoToHall(Some(hall.clone())))}
                        >
                            {hall_name}
                        </button>
                    }
                }).collect::<Html>();

                html! {
                    <div class="relative z-10" aria-labelledby="modal-title" role="dialog" aria-modal="true">
                    <div class="fixed inset-0 bg-gray-500 bg-opacity-25 transition-opacity"></div>
                    <div class="fixed inset-0 z-10 w-screen overflow-y-auto">
                      <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                        <div class="relative w-full transform overflow-hidden rounded-lg bg-white text-center shadow-xl transition-all sm:my-8 sm:max-w-lg">
                          <div class="bg-black px-4 pb-4 pt-5 sm:p-6 sm:pb-4">
                            <div class="sm:flex sm:items-start">
                              <div class="mt-3 sm:ml-4 sm:mt-0 w-full">
                              {page_buttons}
                              </div>
                            </div>
                            <button
                                class="block w-full mt-10 text-center px-4 my-2 py-2 text-xl leading-5 text-gray-400 hover:bg-gray-300 hover:text-gray-900 focus:outline-none focus:bg-gray-100 focus:text-gray-900"
                                role="menuitem"
                                onclick={ctx.link().callback(move |_| HallMsg::GoToHall(None))}
                            >
                                {"All Artifacts"}
                            </button>
                            <button
                                type="button"
                                class="text-center mt-10 w-full rounded-md bg-red-100 px-3 py-2 text-sm font-semibold text-black shadow-sm hover:bg-red-500"
                                onclick={ctx.link().callback(|_| HallMsg::ToggleMenu)}
                                >
                            {"❌"}
                            </button>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                }
            }
        };
        let mut loaded_articles = built_yaml.artifacts;
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

        html! {
            <div class="bg-black h-full">
                {menu}
                <h1 class="text-center text-white text-4xl px-8 md:px-40">
                <button class={format!("w-full text-white text-center py-6 my-4 text-4xl rounded-lg ring-1 ring-inset bg-gray-500/20 text-gray-400 ring-gray-500/80",)}
                    onclick={ctx.link().callback(|_| HallMsg::ToggleMenu)}>
                    {hall_name}
                </button>
                <p class="text-base text-gray-300 py-5 px-8">{match &self.active_hall {
                    Some(hall) => hall.desc(),
                    None => r#"All Artifacts (click on "All Artifacts" to filter by hall)"#,
                }}
                </p>
                </h1>

                <ul role="list" class="text-white px-4 md:px-40 md:py-10 divide-y divide-gray-800">
                    {articles}
                </ul>
            </div>
        }
    }
}
