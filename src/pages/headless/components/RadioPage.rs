#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxycomp_headless::components::Label::Label;
use dioxycomp_headless::components::Radio::Radio;

#[component]
pub fn RadioPage(name: String) -> Element {
    rsx! {
        section {
            id: "main_heading",
            h1 {
                class: "pb-6 text-4xl font-serif font-semibold",
                "Radio"
            },
            hr {},
        },
        section {
            id: "description",
            p {
                class: "text-xl",
                "The Radio button typically allows user to select an item from a list of individual items, or mark one individual item as selected.
                They consist of a visual select indicator and a label.
                "
            },
        }
        section {
            id: "examples",
            h2 {
                class: "pb-4 text-2xl font-serif font-semibold",
                "Example"
            },
            hr {},
            div {
                class: "flex flex-wrap flex-row my-8 p-4 h-40 bg-neutral-800 rounded-xl",
                div {
                    class: "w-full h-8 pl-4 pb-12 text-white/40 border-b-2 border-white/30 ",
                    "Preview: "
                }
                div {
                    class: "flex justify-center items-center w-full h-24",
                    Radio { on_click: move |event| {}},
                    Label {
                        r#for: "radio-n",
                        style: "padding-left: 1rem",
                        "Select"
                    }
                }
            },
            div {
                class: "flex flex-wrap flex-row gap-4 p-4 h-auto bg-neutral-800 rounded-xl",
                div {
                    class: "w-full h-8 pl-4 pb-12 text-white/40 border-b-2 border-white/30 ",
                    "Code: "
                },
                div {
                    class: "flex justify-start items-center w-full rounded-xl bg-zinc-900",
                    pre {
                        class: "py-4 pl-8",
                        code {
                            class: "text-sm whitespace-pre-wrap",
                            span { class: "rs-keyword", "pub use "},
                            span { class: "rs-module", "dioxycomp_headless" },
                            span { class: "rs-operator", "::" },
                            span { class: "rs-module", "components"},
                            span { class: "rs-operator", "::" },
                            span { class: "rs-module", "Radio"},
                            span { class: "rs-operator", "::" },
                            span { class: "rs-element", "Radio"},
                            span { class: "rs-operator", ";" },
                            span { class: "", "\n" },
                            span { class: "rs-comment", "//... Some other code here "},
                            span { class: "", "\n" },
                            span { class: "rs-keyword", "pub fn " },
                            span { class: "rs-function", "HomePage" },
                            span { class: "rs-operator", "(" },
                            span { class: "rs-fn-param", "name: " },
                            span { class: "rs-fn-param-type", "String" },
                            span { class: "rs-operator",")" },
                            span { class: "rs-operator", " -> " },
                            span { class: "rs-type", "Element " },
                            span { class: "rs-operator", "{{" },
                            span { class: "", "\n" },
                            span { class: "rs-fn-object-member", "    rsx! " },
                            span { class: "rs-operator", "{{" },
                            span { class: "", "\n" },
                            span { class: "rs-child-element", "        p " },
                            span { class: "rs-operator", "{{" },
                            span { class: "", "\n" },
                            span { class: "rs-element", "            Radio " },
                            span { class: "rs-operator", "{{ " },
                            span { class: "rs-props-name", "on_click" },
                            span { class: "rs-operator", ": " },
                            span { class: "rs-operator", "move " },
                            span { class: "rs-operator", "|" },
                            span { class: "rs-variable-name", "event" },
                            span { class: "rs-operator", "|" },
                            span { class: "rs-operator", " {{}}" },
                            span { class: "rs-operator", " }}" },
                            span { class: "rs-operator", ", " },
                            span { class: "", "\n" },
                            span { class: "rs-operator", "        }}" },
                            span { class: "", "\n" },
                            span { class: "rs-operator", "    }}" },
                            span { class: "", "\n" },
                            span { class: "rs-operator", "}}" },
                        }
                    }
                }
            }
        }
    }
}
