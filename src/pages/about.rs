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
                <div class="mx-auto max-w-2xl py-32 sm:py-48 lg:py-56 bg-opacity-50">
                    {"hi."}

                </div>
            </div>
        </div>
        }
    }
}
