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

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <div class="bg-black h-full">
            <div class="relative isolate px-6 pt-14 lg:px-8">
                <div class="mx-auto max-w-2xl py-16 lg:py-56 bg-opacity-50">

                    <figure class="bg-slate-100 rounded-xl p-8 dark:bg-slate-800">
                    <img class="w-36 h-36 rounded-full mx-auto" src="https://avatars.githubusercontent.com/u/32376517?v=4" alt="" width="384" height="512"/>
                    <div class="pt-6 text-center space-y-4">
                      <blockquote>
                        <p class="text-lg font-medium text-slate-400">
                          {"“Some code is so exquisite, it belongs in a digital museum, celebrated like timeless art.”"}
                        </p>
                      </blockquote>
                      <figcaption class="font-medium">
                        <div class="text-sky-500">
                            {"Allen Chen"}
                        </div>
                        <div class="text-slate-500">
                            {"Chief Curator, The Museum of Code"}
                        </div>
                      </figcaption>
                    </div>
                  </figure>


                </div>
            </div>
        </div>
        }
    }
}
