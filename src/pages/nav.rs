use yew::prelude::*;

use crate::Page;

pub struct Nav {
    show_sidebar: bool,
}

#[derive(Properties, PartialEq)]
pub struct NavProps {
    pub on_clicked: Callback<Page>,
}

pub enum Msg {
    ToggleSidebar,
    SelectPage(Page),
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
                ctx.props().on_clicked.emit(page);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.show_sidebar {
            html! {
                <div class="z-50 fixed bottom-4 right-4">

                <div class="flex-none rounded-full bg-blue-500/20 p-1">
                  <button
                      class="w-16 h-16 bg-blue-500 text-white rounded-full flex items-center justify-center"
                      onclick={ctx.link().callback(|_| Msg::ToggleSidebar)}
                  >
                  <svg
                      xmlns="http://www.w3.org/2000/svg"
                      class="h-9 w-9 transform"
                      fill="none"
                      viewBox="0 0 60 60"
                      stroke="currentColor"
                  >
                  <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M44.394,13.091L8.633,29.503c-0.399,0.184-0.632,0.605-0.574,1.041s0.393,0.782,0.826,0.854l15.833,2.653l1.809,14.95   c0.054,0.438,0.389,0.791,0.824,0.865c0.057,0.01,0.113,0.015,0.169,0.015c0.375,0,0.726-0.211,0.896-0.556l17.291-34.882   c0.188-0.38,0.117-0.837-0.178-1.141S44.776,12.914,44.394,13.091z M28.11,45.438l-1.496-12.369   c-0.054-0.44-0.391-0.793-0.828-0.866l-13.362-2.239L42.66,16.087L28.11,45.438z"
                  />
                  </svg>
                  </button>
                </div>
                </div>
            }
        } else {
            let nav_buttons = vec![Page::Home, Page::Contact, Page::Tour, Page::About]
            .into_iter()
            .map(|page| {
                html! {
                    <button
                        onclick={ctx.link().callback(move |_| Msg::SelectPage(page))}
                        class="rounded-md w-full bg-indigo-600 my-2.5 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                    >
                    {page.to_string()}
                    </button>
                }
            });

            html! {
                <div class="relative z-10" aria-labelledby="modal-title" role="dialog" aria-modal="true">
                <div class="fixed inset-0 bg-gray-500 bg-opacity-25 transition-opacity"></div>
                <div class="fixed inset-0 z-10 w-screen overflow-y-auto">
                  <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                    <div class="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg">
                      <div class="bg-white px-4 pb-4 pt-5 sm:p-6 sm:pb-4">
                        <div class="sm:flex sm:items-start">
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
                              class="inline-flex mt-20 w-full justify-center rounded-md bg-red-100 px-3 py-2 text-sm font-semibold text-black shadow-sm hover:bg-red-500"
                              onclick={ctx.link().callback(|_| Msg::ToggleSidebar)}
                            >
                              {"❌"}
                            </button>
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
