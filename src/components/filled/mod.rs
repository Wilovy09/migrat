use freya::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn CFilledButton(text: String, click: EventHandler<()>) -> Element {
    rsx!(
        FilledButton{
            onclick: click,
            label {"{text}"}
        }
    )
}
