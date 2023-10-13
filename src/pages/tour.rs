use yew::prelude::*;

use crate::code::{Article, ArticleProps, MetaYaml};
use crate::Page;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BuiltYaml {
    pub meta: MetaYaml,
    pub artifacts: Vec<ArticleProps>,
}

pub struct Tour;

#[derive(Properties, PartialEq)]
pub struct TourProps {
    pub on_clicked: Callback<Page>,
}

impl Component for Tour {
    type Message = ();
    type Properties = TourProps;

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
        // iterate through all files under ../artifacts/ with .yaml extension at compile time
        let yaml = include_str!("../artifacts/build/compiled.yaml");
        let built_yaml: BuiltYaml = serde_yaml::from_str(yaml).unwrap();

        let mut article_props = built_yaml.artifacts;
        article_props.sort_by(|a, b| a.status.cmp(&b.status));
        let articles = article_props
            .into_iter()
            .map(|props| {
                html! {
                    <Article
                    title={props.title}
                    language={props.language}
                    status={props.status}
                    tags={props.tags}
                    code={props.code}
                    desc={props.desc}
                    />
                }
            })
            .collect::<Vec<Html>>();

        html! {
            <div class="bg-black h-full">
                <ul role="list" class="text-white px-4 md:px-40 md:py-10 divide-y divide-gray-800">
                    {articles}
                </ul>
            </div>
        }
    }
}
