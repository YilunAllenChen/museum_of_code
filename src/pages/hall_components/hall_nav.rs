use yew::prelude::*;

use crate::{artifact::ExhibitionHall, pages::hall::HallMsg};

pub struct HallNav {
    active: bool,
}

#[derive(Properties, PartialEq)]
pub struct HallNavProps {
    pub active_hall_name: String,
    pub hall_cb: Callback<HallMsg>,
}

pub enum HallNavMsg {
    ToggleMenu(bool),
    GoToHall(Option<ExhibitionHall>),
}

impl Component for HallNav {
    type Message = HallNavMsg;
    type Properties = HallNavProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { active: false }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let menu = match self.active {
            false => html! {
                <div class="select-none z-30 animate-enter-bottom fixed left-16 bottom-4 left-20 ">
                <div class="flex-none rounded-full bg-cyan-500/20 p-1">
                    <button
                        class="w-12 h-12 bg-cyan-500 text-white rounded-full text-2xl flex items-center justify-center"
                        onclick={ctx.link().callback(|_| HallNavMsg::ToggleMenu(true))}>
                    {"🏛️"}
                    </button>
                </div>
                </div>
            },
            true => {
                let page_buttons = vec![
                    ExhibitionHall::HallOfHelloWorld,
                    ExhibitionHall::HallOfExpressiveness,
                    ExhibitionHall::HallOfSpeed,
                    ExhibitionHall::HallOfSafety,
                    ExhibitionHall::HallOfArt,
                    ExhibitionHall::HallOfTroll,
                    ExhibitionHall::Uncategorized,
                ].into_iter().map(|hall|{
                    let hall_name = hall.to_string();
                    html! {
                        <button
                            class="block w-full text-left md:pl-8 py-2 text:base md:text-xl text-gray-400 hover:text-gray-200 hover:font-semibold"
                            role="menuitem"
                            onclick={ctx.link().callback(move |_| HallNavMsg::GoToHall(Some(hall.clone())))}
                        >
                            {hall_name}
                        </button>
                    }
                }).collect::<Html>();

                html! {
                    <div class="relative z-40" aria-labelledby="modal-title" role="dialog" aria-modal="true">
                    <div class="fixed inset-0 bg-gray-500 bg-opacity-25 transition-opacity"></div>
                    <div class="select-none fixed inset-0 z-50 w-screen overflow-y-auto" onclick={
                        ctx.link().callback(|_| HallNavMsg::ToggleMenu(false))
                    }>
                      <div class="flex ease-in min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                        <div class="w-full overflow-hidden rounded-lg text-left sm:max-w-2xl">
                          <div class="bg-black px-4 py-4 md:p-8">
                            <div class="mx-4 md:mx-12">
                                <div class="text-lg text-center mb-4 text-gray-100"> {"🏛️ Hall Navigation"} </div>
                                <div class="text-base text-center text-gray-500 mb-2 md:mb-6"> {"Now in: "}{ctx.props().active_hall_name.clone()} </div>
                            </div>
                            <div class="mt-3 px-6 sm:mt-0 w-full divide-y divide-gray-700 space-y-1">
                                {page_buttons}
                            </div>

                            <div class="mx-auto md:max-w-md">
                                <button
                                    class="block w-full mt-10 text-center p-2 text-xl text-gray-300 bg-gray-800 rounded-lg"
                                    role="menuitem"
                                    onclick={ctx.link().callback(move |_| HallNavMsg::GoToHall(None))}
                                >
                                    {"View All Artifacts"}
                                </button>
                                <button
                                    type="button"
                                    class="text-center mt-4 md:mb-4 w-full rounded-md bg-red-100 px-3 py-2 text-sm text-black hover:bg-red-500"
                                    onclick={ctx.link().callback(|_| HallNavMsg::ToggleMenu(false))}
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
        html! {
            <>
            {menu}
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HallNavMsg::ToggleMenu(act) => {
                self.active = act;
                true
            }
            HallNavMsg::GoToHall(hall) => {
                self.active = false;
                ctx.props().hall_cb.emit(HallMsg::GoToHall(hall));
                true
            }
        }
    }
    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }
}
