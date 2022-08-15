use std::borrow::Cow;

#[derive(Clone, PartialEq, Debug)]
pub struct Theme {
    pub text: Cow<'static, str>,
    pub background: Cow<'static, str>,
    pub accent: Cow<'static, str>,
    pub primary: Cow<'static, str>,
    pub warning: Cow<'static, str>,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            text: "var(--theme-text)".into(),
            background: "var(--theme-background)".into(),
            accent: "var(--theme-accent)".into(),
            primary: "var(--theme-primary)".into(),
            warning: "var(--theme-warning)".into(),
        }
    }
}
