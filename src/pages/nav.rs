use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::Route;

pub struct Nav {
    show_sidebar: bool,
}

#[derive(Properties, PartialEq)]
pub struct NavProps {}

pub enum Msg {
    ToggleSidebar,
    SelectPage(Route),
}

impl Component for Nav {
    type Message = Msg;
    type Properties = NavProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            show_sidebar: false,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleSidebar => {
                self.show_sidebar = !self.show_sidebar;
                true
            }
            Msg::SelectPage(page) => {
                self.show_sidebar = false;
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&page);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.show_sidebar {
            html! {
                <div class="select-none z-30 enter-exit-bottom fixed bottom-1 left-1 md:bottom-4 md:left-4">
                <div class="flex-none rounded-full bg-blue-500/20 p-1">
                  <button
                      class="w-12 h-12 bg-blue-500 text-white rounded-full flex items-center text-xl justify-center"
                      onclick={ctx.link().callback(|_| Msg::ToggleSidebar)}
                  >
                  {"🧭"}
                  </button>
                </div>
                </div>
            }
        } else {
            let nav_buttons = vec![
                Route::Home,
                Route::Contact,
                Route::ExhibitionHall,
                Route::About,
            ]
            .into_iter()
            .map(|page| {
                let page_name = page.to_string();
                html! {
                    <button
                        onclick={ctx.link().callback(move |_| Msg::SelectPage(page.clone()))}
                        class=r#"
                            rounded-md w-full bg-cyan-500 my-2.5 px-3.5 py-2.5
                            text-sm font-semibold text-white shadow-sm hover:bg-cyan-300
                        "#
                    >
                    {page_name}
                    </button>
                }
            });

            html! {
                <div class="relative z-40" aria-labelledby="modal-title" role="dialog" aria-modal="true">
                <div class="fixed inset-0 bg-gray-500 bg-opacity-25 transition-opacity"></div>
                <div class="fixed inset-0 z-50 w-screen overflow-y-auto">
                  <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                    <div class="enter-exit-bottom relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg">
                      <div class="bg-white px-4 pb-4 pt-5 sm:p-6 sm:pb-4">
                          <div class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left">
                            <h3 class="text-base font-semibold leading-6 text-gray-900" id="modal-title">
                              {"Navigate"}
                            </h3>
                            <div class="mt-2">
                              <p class="text-sm text-gray-500">
                                {"Navigate to any page easily from here."}
                              </p>
                            </div>
                            {for nav_buttons}

                            <button
                              type="button"
                              class="inline-flex mt-20 w-full justify-center rounded-md bg-red-200 px-3 py-2 text-sm font-semibold text-black shadow-sm hover:bg-red-300"
                              onclick={ctx.link().callback(|_| Msg::ToggleSidebar)}
                            >
                              {"❌"}
                            </button>
                            <div class="flex justify-center">
                                <a class="inline-flex mt-6 text-center rounded-full bg-emerald-700 px-3.5 py-2.5 text-xs text-white hover:bg-emerald-500"
                                    href="https://github.com/sponsors/YilunAllenChen"
                                >{"Enjoy the Museum? Become a Sponsor!"}</a>
                            </div>
                          </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            }
        }
    }
}
