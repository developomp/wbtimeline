use yew::{function_component, html, Children, Properties};

use crate::data::Data;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    pub data: Data,
}

#[function_component(Entry)]
pub fn entry(props: &Props) -> Html {
    html! {
        <section class="entry">
            <div class="marker" />

            <div class="item">
                <div class="date">
                    {&props.data.date}
                </div>

                <div class="content">
                    <h2>{&props.data.title}</h2>
                    <p>{&props.data.description}</p>
                    if props.data.media.is_some() {
                        {for props.data.media.as_ref().unwrap().iter()}
                    }
                </div>
            </div>
        </section>
    }
}
