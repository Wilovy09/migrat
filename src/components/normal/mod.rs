use freya::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn CButton(text: String, click: EventHandler<()>) -> Element {
    rsx!(
        Button{
            onclick: click,
            label {"{text}"}
        }
    )
}
