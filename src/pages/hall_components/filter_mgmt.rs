use yew::*;

use crate::{html_utils::render_text_tag, pages::HallMsg};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub hall_msg_cb: Callback<HallMsg>,
    pub available_tags: Vec<String>,
    pub filter_tags: Vec<String>,
}

#[function_component]
pub fn FilterMgmtMenu(props: &Props) -> Html {
    let show_menu = use_state(|| !props.filter_tags.is_empty());

    let toggle_popup = {
        let show_popup = show_menu.clone();
        Callback::from(move |_| show_popup.set(!*show_popup))
    };

    let menu_btn = html! {
        <div class="select-none z-30 animate-enter-bottom fixed left-36 bottom-4">
            <div class="flex gap-x-2">
                <div class="flex-none rounded-full bg-indigo-500/20 p-1">
                    <button
                        class="w-12 h-12 bg-indigo-500 text-white rounded-full text-2xl flex items-center justify-center"
                        onclick={toggle_popup.clone()}>
                    {"🏷️"}
                    </button>
                </div>
            </div>
        </div>
    };

    let tag_to_toggle_button = |tag: &String| {
        let tagc = tag.clone();
        html! {
            <div
                onclick={props.hall_msg_cb.clone().reform(move |_| HallMsg::ToggleTag(tagc.clone()))}
            >
            { Html::from_html_unchecked( render_text_tag(tag).into()) }
            </div>
        }
    };

    let enable_tag = props
        .available_tags
        .iter()
        .filter(|tag| !props.filter_tags.contains(tag))
        .map(tag_to_toggle_button)
        .collect::<Html>();

    let clear_individual_tags = match props.filter_tags.len() {
        0 => html! {
            <p class="text-gray-700">{"No active filters"}</p>
        },
        _ => props
            .filter_tags
            .iter()
            .filter(|tag| props.available_tags.contains(tag))
            .map(tag_to_toggle_button)
            .collect::<Html>(),
    };

    let menu = html! {
        <div class="relative z-40">
            <div class="fixed inset-0 bg-gray-500 bg-opacity-25 transition-opacity"></div>
            <div
                class="select-none fixed inset-0 z-50 w-screen overflow-y-auto overflow-x-auto"
                onclick={toggle_popup.clone()}
            >
                <div class="flex ease-in min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                    <div class="w-full bg-black bg-opacity-80 text-white overflow-hidden rounded-lg text-center sm:max-w-2xl"
                     onclick={|e: MouseEvent| e.stop_propagation()}>
                        <p class="text-2xl font-semibold mt-4">
                            {"Filter Management"}
                        </p>


                        <p class="text-xl mt-8">
                            {"Active Filters"}
                        </p>
                        <div class="px-4 flex flex-wrap justify-center gap-x-2 gap-y-2 mt-4">
                            {clear_individual_tags}
                        </div>

                        <p class="text-xl mt-8">
                            {"Available Filters"}
                        </p>
                        <div class="px-4 flex flex-wrap justify-center gap-x-2 gap-y-2 mt-4">
                            {enable_tag}
                        </div>

                        <div class="flex justify-center m-10 gap-x-4">
                            <p class="text-center text-xl w-1/2 md:w-1/4 rounded-lg p-2 bg-indigo-500 font-semibold"
                                onclick={props.hall_msg_cb.clone().reform(|_| HallMsg::ClearTags)}
                            >
                                {"Clear All"}
                            </p>
                            <p class="text-center text-xl w-1/2 md:w-1/4 rounded-lg p-2 bg-red-500 font-semibold"
                                onclick={toggle_popup}
                            >
                                {"Close"}
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    };

    match *show_menu {
        false => html! {
            {menu_btn}
        },
        true => html! {
            {menu}
        },
    }
}
