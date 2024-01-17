#![allow(non_snake_case)]

use dioxus::prelude::*;
pub use dioxycomp_headless::components::Button::Button;
pub use dioxycomp_headless::components::Button::ButtonProps;

#[inline_props]
pub fn ButtonPage(cx: Scope, name: String) -> Element {
    let bp = ButtonProps {
        id: Some("0"),
        label: Some("OK"),
        autofocus: Some(false),
        disabled: Some(false),
        name: Some(""),
        r#type: Some(""),
        value: Some(""),
        styles: Some("width:3em; height:2em; font-size: 1em; border:1px solid #fef"),
    };
    cx.render(rsx! {
        section {
            id: "main_heading",
            h1 {
                class: "pb-6 text-4xl font-serif font-semibold",
                "Button"
            },
            hr {},
        },
        section {
            id: "description", 
            p {
                class: "text-xl",
                "Buttons are one of the main components commonly used on a webpage.
                They are usually constructed from a clickable area, with a textual label or an icon.
                Button can also allows an action to be executed, when it is clicked or tapped (form submit, form reset, open menu etc.)."
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
                    Button {
                        button_props: bp,
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
                    class: "flex justify-start items-center w-full rounded-xl bg-neutral-900", 
                    pre {
                        class: "py-4 pl-16",
                        code {
                            class: "text-sm whitespace-pre-wrap",
                            span { class: "rs-keyword", "pub use "},
                            span { class: "rs-module", "dioxycomp_headless" },
                            span { class: "rs-operator", "::" },
                            span { class: "rs-module", "components"},
                            span { class: "rs-operator", "::" },
                            span { class: "rs-module", "Button"},
                            span { class: "rs-operator", "::" },
                            span { class: "rs-element", "Button"},
                            span { class: "rs-operator", ";" },
                            span { class: "", "\n" },
                            span { class: "rs-keyword", "pub use "},
                            span { class: "rs-module", "dioxycomp_headless" },
                            span { class: "rs-operator", "::" },
                            span { class: "rs-module", "components"},
                            span { class: "rs-operator", "::" },
                            span { class: "rs-module", "Button"},
                            span { class: "rs-operator", "::" },
                            span { class: "rs-module", "ButtonProps"},
                            span { class: "rs-operator", ";" },
                            span { class: "", "\n" },
                            span { class: "rs-comment", "//... Some other code here "},
                            span { class: "", "\n" },
                            span { class: "rs-keyword", "pub fn " },
                            span { class: "rs-function", "HomePage" },
                            span { class: "rs-operator", "(" },
                            span { class: "rs-fn-param", "cx: " },
                            span { class: "rs-fn-param-type", "Scope" },
                            span { class: "rs-operator", ", " },
                            span { class: "rs-fn-param", "name: " },
                            span { class: "rs-fn-param-type", "String" },
                            span { class: "rs-operator",")" },
                            span { class: "rs-operator", " -> " },
                            span { class: "rs-type", "Element " },
                            span { class: "rs-operator", "{{" },
                            span { class: "", "\n" },
                            span { class: "rs-keyword", "    let " },
                            span { class: "rs-variable-name", "bp " },
                            span { class: "rs-operator", "= " },
                            span { class: "rs-module", "ButtonProps " },
                            span { class: "rs-operator", "{{" },
                            span { class: "", "\n" },
                            span { class: "rs-props-name", "        id" },
                            span { class: "rs-operator", ": " },
                            span { class: "rs-option-type", "Some" },
                            span { class: "rs-operator", "(" },
                            span { class: "rs-operator", "\"" },
                            span { class: "rs-prop-value", "0" },
                            span { class: "rs-operator", "\"" },
                            span { class: "rs-operator", ")" },
                            span { class: "rs-operator", "," },
                            span { class: "", "\n" },
                            span { class: "rs-props-name", "        label" },
                            span { class: "rs-operator", ": " },
                            span { class: "rs-option-type", "Some" },
                            span { class: "rs-operator", "(" },
                            span { class: "rs-operator", "\"" },
                            span { class: "rs-prop-value", "OK" },
                            span { class: "rs-operator", "\"" },
                            span { class: "rs-operator", ")" },
                            span { class: "rs-operator", "," },
                            span { class: "", "\n" },
                            span { class: "rs-props-name", "        autofocus" },
                            span { class: "rs-operator", ": " },
                            span { class: "rs-option-type", "Some" },
                            span { class: "rs-operator", "(" },
                            span { class: "rs-operator", "\"" },
                            span { class: "rs-prop-value", "false" },
                            span { class: "rs-operator", "\"" },
                            span { class: "rs-operator", ")" },
                            span { class: "rs-operator", "," },
                            span { class: "", "\n" },
                            span { class: "rs-props-name", "        disabled" },
                            span { class: "rs-operator", ": " },
                            span { class: "rs-option-type", "Some" },
                            span { class: "rs-operator", "(" },
                            span { class: "rs-operator", "\"" },
                            span { class: "rs-prop-value", "false" },
                            span { class: "rs-operator", "\"" },
                            span { class: "rs-operator", ")" },
                            span { class: "rs-operator", "," },
                            span { class: "", "\n" },
                            span { class: "rs-props-name", "        name" },
                            span { class: "rs-operator", ": " },
                            span { class: "rs-option-type", "Some" },
                            span { class: "rs-operator", "(" },
                            span { class: "rs-operator", "\"" },
                            span { class: "rs-prop-value", "" },
                            span { class: "rs-operator", "\"" },
                            span { class: "rs-operator", ")" },
                            span { class: "rs-operator", "," },
                            span { class: "", "\n" },
                            span { class: "rs-props-name", "        r#type" },
                            span { class: "rs-operator", ": " },
                            span { class: "rs-option-type", "Some" },
                            span { class: "rs-operator", "(" },
                            span { class: "rs-operator", "\"" },
                            span { class: "rs-prop-value", "" },
                            span { class: "rs-operator", "\"" },
                            span { class: "rs-operator", ")" },
                            span { class: "rs-operator", "," },
                            span { class: "", "\n" },
                            span { class: "rs-props-name", "        value" },
                            span { class: "rs-operator", ": " },
                            span { class: "rs-option-type", "Some" },
                            span { class: "rs-operator", "(" },
                            span { class: "rs-operator", "\"" },
                            span { class: "rs-prop-value", "" },
                            span { class: "rs-operator", "\"" },
                            span { class: "rs-operator", ")" },
                            span { class: "rs-operator", "," },
                            span { class: "", "\n" },
                            span { class: "rs-props-name", "        styles" },
                            span { class: "rs-operator", ": " },
                            span { class: "rs-option-type", "Some" },
                            span { class: "rs-operator", "(" },
                            span { class: "rs-operator", "\"" },
                            span { class: "rs-prop-value", "" },
                            span { class: "rs-operator", "\"" },
                            span { class: "rs-operator", ")" },
                            span { class: "rs-operator", "," },
                            span { class: "", "\n" },
                            span { class: "rs-operator", "    }}" },
                            span { class: "", "\n" },
                            span { class: "rs-fn-param", "    cx" },
                            span { class: "rs-operator", "." },
                            span { class: "rs-fn-object-member", "render" },
                            span { class: "rs-operator", "(" },
                            span { class: "rs-fn-object-member", "rsx!" },
                            span { class: "rs-operator", "{{" },
                            span { class: "", "\n" },
                            span { class: "rs-child-element", "        p " },
                            span { class: "rs-operator", "{{" },
                            span { class: "", "\n" },
                            span { class: "rs-element", "            Button " },
                            span { class: "rs-operator", "{{ " },
                            span { class: "rs-props-name", "button_props" },
                            span { class: "rs-operator", ": " },
                            span { class: "rs-variable-name", "bp " },
                            span { class: "rs-operator", " }}" },
                            span { class: "rs-operator", "," },
                            span { class: "", "\n" },
                            span { class: "rs-operator", "        }}" },
                            span { class: "", "\n" },
                            span { class: "rs-operator", "    }}" },
                            span { class: "rs-operator", ")" },
                            span { class: "", "\n" },
                            span { class: "rs-operator", "}}" },
                        }
                    }
                }
            }
        }
    })
}
