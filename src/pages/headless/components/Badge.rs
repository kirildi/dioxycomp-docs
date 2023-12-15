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
    let mut badge_kind = match cx.props.badge_props.kind.as_ref().unwrap() {
        BadgeKind::Dev => "bg-orange-800",
        BadgeKind::Beta => "bg-sky-800",
        BadgeKind::Final => "bg-emerald-800",
    };

    let mut label_style = "color: white";
    let mut label_value = match cx.props.badge_props.kind.as_ref().unwrap() {
        BadgeKind::Dev => "DEV",
        BadgeKind::Beta => "BETA",
        BadgeKind::Final => "FINAL",
    };

    let lp = LabelProps {
        id: Some(cx.props.badge_props.id.as_ref().unwrap()),
        r#for: None,
        value: Some(label_value),
        class_name: Some(""),
        styles: Some(label_style),
    };

    cx.render(rsx! {
        span {
        id: cx.props.badge_props.id,
        class: "{cx.props.badge_props.class_name.unwrap()} {badge_kind}",
        style: cx.props.badge_props.styling,
            Label {
                label_props: Some(lp),
            }
        }
    })
}
