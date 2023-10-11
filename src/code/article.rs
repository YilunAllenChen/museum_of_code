use yew::prelude::*;

use crate::html_utils::tags::Tag;
use log::info;

use super::lang::Language;

#[derive(Clone, PartialEq)]
pub enum EntryStatus {
    OnExhibit,
    StagedForExhibit,
    Maintenance,
}

pub struct Article {
    show: bool,
}

pub enum ArticleMsg {
    Toggle,
}

#[derive(Properties, PartialEq)]
pub struct ArticleProps {
    pub title: String,
    pub language: Language,
    pub status: EntryStatus,
    pub tags: Vec<Tag>,
    pub code: String,
    pub desc: String,
}

impl Component for Article {
    type Message = ArticleMsg;
    type Properties = ArticleProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { show: false }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ArticleMsg::Toggle => {
                info!("toggling {} -> {}", self.show, !self.show);
                self.show = !self.show;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let dot_and_text = match ctx.props().status {
            EntryStatus::OnExhibit => {
                html! {
                    <div class="mt-1 flex items-center gap-x-1.5">
                      <p class="text-xs leading-5 text-gray-200">{"On Exhibit"}</p>
                      <div class="flex-none rounded-full bg-emerald-500/20 p-1">
                        <div class="h-1.5 w-1.5 rounded-full bg-emerald-500"></div>
                      </div>
                    </div>
                }
            }
            EntryStatus::StagedForExhibit => {
                html! {
                    <div class="mt-1 flex items-center gap-x-1.5">
                      <p class="text-xs leading-5 text-gray-200">{"Staged"}</p>
                      <div class="flex-none rounded-full bg-yellow-500/20 p-1">
                        <div class="h-1.5 w-1.5 rounded-full bg-yellow-500"></div>
                      </div>
                    </div>
                }
            }
            EntryStatus::Maintenance => {
                html! {
                    <div class="mt-1 flex items-center gap-x-1.5">
                      <p class="text-xs leading-5 text-gray-200">{"Maintenance"}</p>
                      <div class="flex-none rounded-full bg-red-500/20 p-1">
                        <div class="h-1.5 w-1.5 rounded-full bg-red-500"></div>
                      </div>
                    </div>
                }
            }
        };

        let tags: Html = ctx
            .props()
            .tags
            .iter()
            .map(|tag| tag.to_html())
            .map(|html_str| Html::from_html_unchecked(html_str.into()))
            .collect();

        let rendered = match self.show {
            true => {
                let content = match ctx.props().status {
                    EntryStatus::OnExhibit => html! {
                      <>
                      <div class="bg-gray-800 text-xs sm:text-sm md:text-lg text-gray-300 p-1 rounded-md justify-left items-left">
                          <pre class="py-4 px-1 sm:px-4">
                              {Html::from_html_unchecked(ctx.props().code.clone().into())}
                          </pre>
                      </div>
                      <pre class="my-4">
                      {ctx.props().desc.clone()}
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

                html! {
                    <div class="relative z-10" aria-labelledby="modal-title" role="dialog" aria-modal="true">
                    <div class="fixed inset-0 bg-gray-500 bg-opacity-25 transition-opacity"></div>
                    <div class="fixed inset-0 z-10 w-screen overflow-y-auto">
                      <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                        <div class="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg">
                          <div class="bg-black px-4 pb-4 pt-5 sm:p-6 sm:pb-4">
                            <div class="sm:flex sm:items-start">
                              <div class="mt-3 sm:ml-4 sm:mt-0 text-left">
                                <h3 class="text-lg leading-6 font-medium text-gray-100" id="modal-title">
                                  {ctx.props().title.clone()}
                                </h3>
                                <div class="my-4">
                                  {tags.clone()}
                                </div>
                                {content}
                                <button
                                  type="button"
                                  class="inline-flex mt-20 w-full justify-center rounded-md bg-red-100 px-3 py-2 text-sm font-semibold text-black shadow-sm hover:bg-red-500"
                                  onclick={ctx.link().callback(|_| {
                                    info!("clicked");
                                    ArticleMsg::Toggle
                                  })}
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
            false => html! {},
        };

        html! {
            <>
              <li class="flex justify-between gap-x-6 py-5"
                  onclick={ctx.link().callback(|_| {
                    info!("opening up");
                    ArticleMsg::Toggle
                  })}
              >
                  <div class="flex min-w-0 gap-x-4">
                      {ctx.props().language.icon()}
                      <div class="min-w-0 flex-auto">
                      <p class="text-sm font-semibold leading-6 text-gray-100">{ctx.props().title.clone()}</p>
                      <p class="mt-1 truncate text-xs leading-5 text-gray-300">{tags}</p>
                      </div>
                  </div>
                  <div class="shrink-0 flex flex-col items-end">
                      {dot_and_text}
                  </div>
              </li>
              {rendered}
            </>
        }
    }
}
