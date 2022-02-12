use yew::{function_component, html, virtual_dom::VNode, Children, Properties};

use crate::data::Data;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    pub data: Data,
}

#[function_component(Entry)]
pub fn entry(props: &Props) -> Html {
    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");

    let description = document.create_element("p").unwrap();
    description.set_inner_html(&props.data.description);

    html! {
        <section class="entry">
            <div class="icon">
                if props.data.category.eq("community") {
                    <i class="fas fa-flag" />
                } else if props.data.category.eq("development") {
                    <i class="fas fa-cog" />
                }
            </div>

            <div class="item">
                <div class="date">
                    <div>
                        {&props.data.date}
                    </div>
                </div>

                <div class="content">
                    <h2>{&props.data.title}</h2>
                    {VNode::VRef(description.into())}
                    if props.data.media.is_some() {
                        {for props.data.media.as_ref().unwrap().iter()}
                    }
                </div>
            </div>
        </section>
    }
}
