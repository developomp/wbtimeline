use yew::{html, Component, Context, Html};

use crate::components::Header;

use crate::state::State;

pub struct App {
    state: State,
}

pub enum Msg {
    ToggleSomething,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let state = State {
            is_something: false,
        };

        App { state }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleSomething => {
                self.state.toggle_something();

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                <Header />

                <button onclick={ctx.link().callback(|_| Msg::ToggleSomething)}>
                    { self.view_something() }
                </button>
            </div>
        }
    }
}

impl App {
    fn view_something(&self) -> Html {
        html! {
            { self.state.is_something }
        }
    }
}
