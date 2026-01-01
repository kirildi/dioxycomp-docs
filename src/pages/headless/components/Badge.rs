//! # Badge Component
//!
//! Renders a simple badge

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxycomp_headless::components::Label::{Label, LabelProps};

#[derive(Clone, PartialEq, Default, Copy)]
pub enum BadgeKind {
    #[default]
    Dev,
    Beta,
    Final,
}

#[derive(PartialEq, Props, Clone)]
pub struct BadgeProps {
    pub id: Option<String>,
    pub value: Option<String>,
    pub kind: Option<BadgeKind>,
    pub class_name: Option<String>,
    pub styling: Option<String>,
}
#[component]
pub fn Badge(props: BadgeProps) -> Element {
    //TODO currently supporting only Tailwind css case
    let mut badge_kind = match props.kind.unwrap() {
        BadgeKind::Dev => "bg-orange-800",
        BadgeKind::Beta => "bg-sky-800",
        BadgeKind::Final => "bg-emerald-800",
    };

    let mut label_style = String::from("color: white");
    let mut label_value = match props.kind.to_owned().unwrap_or_default() {
        BadgeKind::Dev => String::from("DEV"),
        BadgeKind::Beta => String::from("BETA"),
        BadgeKind::Final => String::from("FINAL"),
    };

    let lp: LabelProps = LabelProps {
        id: Some(props.id.to_owned().unwrap()),
        r#for: None,
        value: Some(label_value),
        class_name: Some(String::from("")),
        styles: Some(label_style),
    };

    rsx! {
        span {
        id: props.id.to_owned().unwrap(),
        class: "{props.class_name.to_owned().unwrap()} {badge_kind}",
        style: props.styling,
            //Label { },
        }
    }
}
