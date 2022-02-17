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

    let mut media = Vec::new();

    if props.data.media.is_some() {
        for media_link in props.data.media.as_ref().unwrap() {
            // if the link is an image
            if media_link.starts_with("/img") {
                media.push(html! {
                    <img src={media_link.to_string()} />
                });
            }

            // if the link is a youtube video
            if media_link.contains("youtube.com") {
                media.push(html! {
                    <div class="video-container">
                        <iframe src={media_link.to_string()} />
                    </div>
                });
            }
        }
    }

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
                        {for media}
                    }

                    if props.data.button.is_some() {
                        <a class="button" href={String::from(props.data.button.as_ref().unwrap()[0].as_str())} target="_blank">
                            {&props.data.button.as_ref().unwrap()[1]}
                        </a>
                    }
                </div>
            </div>
        </section>
    }
}
