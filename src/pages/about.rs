use yew::prelude::*;

use crate::Page;

pub struct About;

#[derive(Properties, PartialEq)]
pub struct AboutProps {
    pub on_clicked: Callback<Page>,
}

impl Component for About {
    type Message = ();
    type Properties = AboutProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div class="bg-black h-full">
            <div class="relative isolate px-6 pt-14 lg:px-8">
                <div class="mx-auto max-w-2xl py-16 lg:py-56 bg-opacity-50">
                    <figure class="bg-slate-100 rounded-xl p-8 bg-slate-800">
                    <div class="pt-6 text-center space-y-4">
                      <blockquote>
                        <p class="text-lg font-medium text-slate-100">
                          {"“Some code is so exquisite, it belongs in a digital museum, celebrated like timeless art.”"}
                        </p>
                      </blockquote>
                      <figcaption class="font-medium">
                        <div class="text-sky-500 my-4">
                            {"Allen Chen"}
                            <div class="text-xs text-slate-600">
                                {"(or perhaps ChatGPT)"}
                            </div>
                        </div>
                        <div class="text-slate-400">
                            {"Chief Curator, The Museum of Code"}
                        </div>
                        <button
                            onclick={ctx.props().on_clicked.reform(|_| Page::Home)}
                            class="rounded-md mt-10 bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
                            {"Home"}
                        </button>
                      </figcaption>
                    </div>
                  </figure>


                </div>
            </div>
        </div>
        }
    }
}
