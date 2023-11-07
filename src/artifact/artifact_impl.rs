use yew::prelude::*;

use crate::{
    artifact::artifact_model::EntryStatus,
    html_utils::{render_text_tag, scroll::try_scroll_to},
    pages::HallMsg,
};

use super::artifact_model::Article;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct ArticleComponent {
    show: bool,
}

pub enum ArticleMsg {
    Toggle(bool),
}

#[derive(Properties, Debug)]
pub struct ArticleProps {
    pub article: Article,
    pub hall_cb: Callback<HallMsg>,
}

impl PartialEq for ArticleProps {
    fn eq(&self, other: &Self) -> bool {
        self.article == other.article
    }
}

impl Component for ArticleComponent {
    type Message = ArticleMsg;
    type Properties = ArticleProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { show: false }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ArticleMsg::Toggle(tf) => {
                self.show = tf;

                // Scroll to the article if it is toggled on
                if tf {
                    let article_id = ctx.props().article.title.clone();
                    try_scroll_to(&article_id);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (dot_color, text) = match ctx.props().article.status {
            EntryStatus::OnExhibit => ("emerald", "On Exhibit"),
            EntryStatus::StagedForExhibit => ("yellow", "Staged"),
            EntryStatus::Maintenance => ("red", "Maintenance"),
        };
        let outer_dot_class = format!("rounded-full bg-{dot_color}-500/20 p-1");
        let inner_dot_class = format!("h-1.5 w-1.5 rounded-full bg-{dot_color}-500");
        let dot_and_text = html! {
            <div class="">
              <div class="items-center gap-x-1.5 flex">
                <div class="text-xs flex-none leading-5 text-gray-400">{text}</div>
                <div class={outer_dot_class}>
                  <div class={inner_dot_class}/>
                </div>
              </div>
            </div>
        };

        let tags: Html = ctx
            .props()
            .article
            .tags
            .clone()
            .into_iter()
            .map(|tag| {
                let tag_clone = tag.clone();
                let emitter = ctx.props().hall_cb.clone();
                html! {
                <span
                    class="pointer-events-auto"
                    onclick={move |e: MouseEvent| {
                        e.stop_propagation();
                        emitter.emit(HallMsg::ToggleTag(tag_clone.clone()))}}
                >
                    {Html::from_html_unchecked(
                        render_text_tag(&tag).into()
                    )}
                </span>
                }
            })
            .collect();

        let emitter = ctx.props().hall_cb.clone();
        let language_clone = ctx.props().article.language.clone();
        let language_tag = html! {
          <span
            class="pointer-events-auto"
            onclick={move |e: MouseEvent| {
                e.stop_propagation();
                emitter.emit(HallMsg::ToggleTag(language_clone.to_string().into()))
            }}
          >
            {Html::from_html_unchecked(
                ctx.props().article.language.to_tag().into()
            )}
          </span>
        };

        // actual article
        let rendered = match self.show {
            true => {
                let content = match ctx.props().article.status {
                    EntryStatus::OnExhibit => html! {
                      <>
                        <div class="bg-gray-800 text-xs sm:text-sm text-gray-300 p-1 rounded-md justify-left items-left">
                            <pre class="py-2 md:py-4 px-1 sm:px-4">
                                {Html::from_html_unchecked(ctx.props().article.code.clone().into())}
                            </pre>
                        </div>
                        <pre class="my-4 font-sans">
                        {Html::from_html_unchecked(ctx.props().article.desc.clone().into())}
                        </pre>
                      </>
                    },
                    EntryStatus::StagedForExhibit => html! {
                      <>
                      {"This artifact is staged for exhibit but is not yet ready for public viewing."}
                      </>
                    },
                    EntryStatus::Maintenance => html! {
                      <>
                      {"This artifact is under maintenance. Check back later!"}
                      </>
                    },
                };

                let x = html! {
                  <button
                    type="button"
                    class="inline-flex w-full justify-center rounded-md bg-red-200 p-2 text-sm font-semibold text-black shadow-sm hover:bg-red-500"
                    onclick={ctx.link().callback(|_| ArticleMsg::Toggle(false))}
                  >
                    {"❌"}
                  </button>
                };

                html! {
                    <div class="relative top-0 z-10" aria-labelledby="modal-title" role="dialog" aria-modal="true">
                    <div class="fixed inset-0 bg-gray-500 bg-opacity-25 transition-opacity"
                          onclick={ctx.link().callback(|_| ArticleMsg::Toggle(false))}
                    ></div>
                    <div class="inset-0 z-10 w-100 overflow-y-auto">
                      <div class="flex items-end justify-center text-center sm:items-center sm:p-0">
                        <div class="ease-in overflow-x-hidden relative transform overflow-hidden rounded-lg text-left shadow-xl transition-all w-full">
                          <div class="bg-black px-4 pb-4 pt-5 sm:p-6 sm:pb-4">
                            <div class="justify-between">
                              <div class="mt-3 sm:ml-4 sm:mt-0 text-left">
                                <h3 class="text-lg leading-6 font-medium text-gray-100" id="modal-title">
                                  <div class="flex">
                                    <div class="mr-auto self-center">
                                      {ctx.props().article.title.clone()}
                                    </div>
                                    <div class="w-1/8">
                                      {x.clone()}
                                    </div>
                                  </div>
                                </h3>
                                <div class="my-4 leading-8">
                                  {Html::from_html_unchecked(ctx.props().article.language.to_tag().into())}
                                  {tags.clone()}
                                </div>
                                {content}
                                <div class="mt-20"/>
                                {x}
                              </div>
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                }
            }
            false => html! {},
        };

        html! {
            <>
              <li
                  id={ctx.props().article.title.clone()}
                  class="py-5 w-full"
                  onclick={ctx.link().callback(|_| ArticleMsg::Toggle(true))}
              >
                  <div class="flex gap-x-2 md:gap-x-4">
                    {Html::from_html_unchecked(ctx.props().article.language.icon().into())}
                    <div class="w-full">
                      <div class="space-y-1">
                        <div class="flex">
                          <p class="text-sm flex-auto leading-6 text-gray-100">{ctx.props().article.title.clone()}</p>
                          <div class="">
                              {dot_and_text}
                          </div>
                        </div>
                        <div class="flex-none overflow-hidden">
                          <div class="flex w-72 lg:w-96">
                            <p class="truncate flex-auto">
                                {language_tag}
                                {tags}
                            </p>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
              </li>
              {rendered}
            </>
        }
    }
}
