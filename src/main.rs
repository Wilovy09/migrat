use freya::prelude::*;

fn main() {
    launch(app)
}

fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx!(
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            background: "#D0D0D0",
            color: "white",
            shadow: "0 4 20 5 rgb(0,0,0,80)",
            label {  
                font_size: "75", 
                font_weight: "bold", 
                "{count()}"
            }
        }
        rect { 
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            direction: "horizontal",
            MyButton {
                text: "Decrease",
                click: move |_| count += 1
            }
            MyButton {
                text: "Decrease",
                click: move |_| count -= 1
            }
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn MyButton(text: String, click: EventHandler<()>) -> Element {
    rsx!(
        Button{
            onclick: click,
            label {"{text}"}
        }
    )
}