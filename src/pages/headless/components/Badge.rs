//! # Badge Component
//!
//! Renders a simple badge

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxycomp_headless::components::Label::{Label, LabelProps};

pub enum BadgeKind {
    Dev,
    Beta,
    Final,
}

#[derive(Props, Clone, Default)]
pub struct BadgeProps<'a> {
    pub id: Option<&'a str>,
    pub value: Option<&'a str>,
    pub kind: Option<&'a BadgeKind>,
    pub class_name: Option<&'a str>,
    pub styling: Option<&'a str>,
}
#[derive(Props, Clone, Default)]
pub struct AllBadgeProps<'a> {
    pub badge_props: BadgeProps<'a>,
}

pub fn Badge<'a>(cx: Scope<'a, AllBadgeProps<'a>>) -> Element<'a> {
    //TODO currently supporting only Tailwind css case
    let mut badge_kind = match cx.props.badge_props.kind {
        Some(BadgeKind::Dev) => Some("bg-orange-800"),
        Some(BadgeKind::Beta) => Some("bg-sky-800"),
        Some(BadgeKind::Final) => Some("bg-emerald-800"),
        None => Some("bg-orange-800"),
    };

    let mut label_style = "color: white";
    let mut label_value = match cx.props.badge_props.kind {
        Some(BadgeKind::Dev) => Some("DEV"),
        Some(BadgeKind::Beta) => Some("BETA"),
        Some(BadgeKind::Final) => Some("FINAL"),
        None => Some("DEV"),
    };

    let lp = LabelProps {
        id: Some(cx.props.badge_props.id.as_ref().unwrap()),
        r#for: None,
        value: Some(label_value.as_ref().unwrap()),
        styles: Some(label_style),
    };

    cx.render(rsx! {
          span {
            id: cx.props.badge_props.id,
            class: cx.props.badge_props.class_name,
            style: cx.props.badge_props.styling,
                Label{ label_props: lp}
          }
    })
}
