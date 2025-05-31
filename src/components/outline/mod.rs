use freya::prelude::*;

/*
pub struct ButtonTheme {
    pub font_theme: FontTheme,
    pub background: Cow<'static, str>,
    pub hover_background: Cow<'static, str>,
    pub border_fill: Cow<'static, str>,
    pub focus_border_fill: Cow<'static, str>,
    pub shadow: Cow<'static, str>,
    pub margin: Cow<'static, str>,
    pub corner_radius: Cow<'static, str>,
    pub width: Cow<'static, str>,
    pub height: Cow<'static, str>,
    pub padding: Cow<'static, str>,
}
*/

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum Variants {
    Default,
    Secondary,
    Destructive,
}

#[allow(non_snake_case)]
#[component]
pub fn COutlineButton(
    text: String,
    click: Option<EventHandler<()>>,
    variant: Option<Variants>,
) -> Element {
    match variant {
        None | Some(Variants::Default) => {
            rsx!(
                OutlineButton{
                  theme: theme_with!(ButtonTheme {
                    font_theme: theme_with!(FontTheme {
                      color: "#000000".into()
                    }),
                    background: "#e5e5e5".into(),
                    border_fill: "#e5e5e5".into(),
                    focus_border_fill: "#cfcfcf00".into(),
                    hover_background: "#cfcfcf".into(),
                    padding: "10".into(),
                  }),
                  onclick: click,
                  label {"{text}"}
                }
            )
        }
        Some(Variants::Secondary) => {
            rsx!(
                OutlineButton{
                  theme: theme_with!(ButtonTheme {
                    font_theme: theme_with!(FontTheme {
                      color: "#FFFFFF".into()
                    }),
                    background: "#262626".into(),
                    border_fill: "#262626".into(),
                    hover_background: "#202020".into(),
                    focus_border_fill: "#202020".into(),
                    padding: "10".into(),
                  }),
                  onclick: click,
                  label {"{text}"}
                }
            )
        }
        Some(Variants::Destructive) => {
            rsx!(
                OutlineButton{
                  theme: theme_with!(ButtonTheme {
                    font_theme: theme_with!(FontTheme {
                      color: "#ff0000".into()
                    }),
                    background: "#ffe5e5".into(),
                    border_fill: "#ffe5e5".into(),
                    focus_border_fill: "#ffcfcf00".into(),
                    hover_background: "#ffcfcf".into(),
                    padding: "10".into(),
                  }),
                  onclick: click,
                  label {"{text}"}
                }
            )
        }
    }
}
