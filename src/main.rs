use components::outline::{COutlineButton, Variants};
use freya::prelude::*;

mod components;

fn main() {
    launch_with_props(app, "Migrat", (500.0, 500.0))
}

fn get_theme(preferred_theme: PreferredTheme) -> Theme {
    match preferred_theme {
        PreferredTheme::Dark => DARK_THEME,
        PreferredTheme::Light => LIGHT_THEME,
    }
}

fn app() -> Element {
    let preferred_theme = use_preferred_theme();
    let mut current_theme = use_init_theme(|| get_theme(*preferred_theme.peek()));

    use_effect(move || {
        let theme = get_theme(preferred_theme());
        if theme != *current_theme.peek() {
            current_theme.set(theme)
        }
    });

    rsx!(
        Body {
            rect {
                height: "fill",
                width: "fill",
                main_align: "center",
                cross_align: "center",
                direction: "horizontal",
                COutlineButton {
                    text: "Button",
                    variant: Variants::Default
                }
            }
        }
    )
}
