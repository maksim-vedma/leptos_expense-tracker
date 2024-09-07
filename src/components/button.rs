use leptos::*;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Flat,
    Outlined,
    #[default]
    Filled,
}

impl ButtonVariant {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Flat => " shadow text-[--clr-btn] hocus:shadow-md",
            Self::Outlined => "border-[--clr-btn] border-2 bg-transparent text-[--clr-btn] hocus:text-white hocus:bg-[--clr-btn]",
            Self::Filled => "bg-[--clr-btn] text-white hocus:text-black",
        }
    }
}

impl Display for ButtonVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[allow(unused)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonColor {
    #[default]
    Accent,
    Primary,
    Secondary,
    Tertiary,
    Info,
    Success,
    Danger,
}

impl ButtonColor {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Accent => "steelblue",
            Self::Primary => "blue",
            Self::Secondary => "grey",
            Self::Tertiary => "orangered",
            Self::Info => "lightblue",
            Self::Success => "green",
            Self::Danger => "red",
        }
    }
}

impl Display for ButtonColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ButtonSize {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "text-xs",
            Self::Medium => "text-md",
            Self::Large => "text-2xl",
        }
    }
}

impl Display for ButtonSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
const BTN_STYLE: &str =
    "inline-grid items-center px-[1em] py-[.5em] gap-[1em] text-balance transition disabled:opacity-50 disabled:grayscale";

#[component]
pub fn Button(
    #[prop(into)] label: MaybeSignal<String>,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] color: ButtonColor,
    #[prop(optional)] size: ButtonSize,
    #[prop(optional)] full: bool,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <button
            class=&format!("{} {} {}", BTN_STYLE, variant, size)
            class=("w-full", move || full)
            style=("--clr-btn", color.as_str())
            style=("text-wrap", "balance")
            id=id
            disabled=disabled
        >
            {label}
        </button>
    }
}
