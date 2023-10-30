use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::Route;

pub struct Contact;

#[derive(Properties, PartialEq)]
pub struct ContactProps {}

impl Component for Contact {
    type Message = ();
    type Properties = ContactProps;

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
        let navigator = ctx.link().navigator().unwrap();
        let nav = |page| Callback::from(move |_| navigator.push(&page));

        html! {
            <div class="ease-in bg-black h-full">
                <div class="mx-auto max-w-2xl py-32 sm:py-48 lg:py-56 bg-opacity-50">
                    <div class="text-center">
                        <h1 class="text-4xl font-bold tracking-tight text-gray-100 sm:text-4xl">{"Be A Part of MoCo!"}</h1>
                        <div class="p-2 mt-10 flex flex-col items-center justify-center gap-x-6">
                            <a class="rounded-md w-1/2 bg-blue-700 px-3.5 py-2.5 text-lg text-white shadow-sm hover:bg-blue-500"
                                href="mailto:allenchenyilun1999@gmail.com"
                            >{"Email Me"}</a>
                            <p class="pt-1text-gray-400 text-xs md:text-sm">{"@allenchenyilun1999@gmail.com"}</p>
                        </div>
                        <div class="p-2 mt-4 flex items-center justify-center gap-x-6 mb-20">
                            <a class="rounded-md w-1/2 bg-emerald-700 px-3.5 py-2.5 text-lg text-white shadow-sm hover:bg-emerald-500"
                                href="https://github.com/sponsors/YilunAllenChen"
                            >{"Sponsor Us!"}</a>
                        </div>
                        <button
                            onclick={nav(Route::Home)}
                            class="rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm text-white shadow-sm hover:bg-indigo-500">
                            {"Go Back"}
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
