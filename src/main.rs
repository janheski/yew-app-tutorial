use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Gra \"Bóg jest dobry\"" }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}
