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
                            span { class: "rs_keyword", "pub use "},
                            span { class: "rs_module", "dioxycomp_headless" },
                            span { class: "rs_operator", "::" },
                            span { class: "rs_module", "components"},
                            span { class: "rs_operator", "::" },
                            span { class: "rs_module", "Button"},
                            span { class: "rs_operator", "::" },
                            span { class: "rs_element", "Button"},
                            span { class: "rs_operator", ";" },
                            span { class: "", "\n" },
                            span { class: "rs_keyword", "pub use "},
                            span { class: "rs_module", "dioxycomp_headless" },
                            span { class: "rs_operator", "::" },
                            span { class: "rs_module", "components"},
                            span { class: "rs_operator", "::" },
                            span { class: "rs_module", "Button"},
                            span { class: "rs_operator", "::" },
                            span { class: "rs_module", "ButtonProps"},
                            span { class: "rs_operator", ";" },
                            span { class: "", "\n" },
                            span { class: "rs_comment", "//... Some other code here "},
                            span { class: "", "\n" },
                            span { class: "rs_keyword", "pub fn " },
                            span { class: "rs_function", "HomePage" },
                            span { class: "rs_operator", "(" },
                            span { class: "rs_fn_param", "cx: " },
                            span { class: "rs_fn_param_type", "Scope" },
                            span { class: "rs_operator", ", " },
                            span { class: "rs_fn_param", "name: " },
                            span { class: "rs_fn_param_type", "String" },
                            span { class: "rs_operator",")" },
                            span { class: "rs_operator", " -> " },
                            span { class: "rs_type", "Element " },
                            span { class: "rs_operator", "{{" },
                            span { class: "", "\n" },
                            span { class: "rs_let", "    let " },
                            span { class: "rs_variable_name", "bp " },
                            span { class: "rs_operator", "= " },
                            span { class: "rs_module", "ButtonProps " },
                            span { class: "rs_operator", "{{" },
                            span { class: "", "\n" },
                            span { class: "rs_props_name", "        id" },
                            span { class: "rs_operator", ": " },
                            span { class: "rs_option_type", "Some" },
                            span { class: "rs_operator", "(" },
                            span { class: "rs_operator", "\"" },
                            span { class: "rs_prop_value", "0" },
                            span { class: "rs_operator", "\"" },
                            span { class: "rs_operator", ")" },
                            span { class: "rs_operator", "," },
                            span { class: "", "\n" },
                            span { class: "rs_props_name", "        label" },
                            span { class: "rs_operator", ": " },
                            span { class: "rs_option_type", "Some" },
                            span { class: "rs_operator", "(" },
                            span { class: "rs_operator", "\"" },
                            span { class: "rs_prop_value", "OK" },
                            span { class: "rs_operator", "\"" },
                            span { class: "rs_operator", ")" },
                            span { class: "rs_operator", "," },
                            span { class: "", "\n" },
                            span { class: "rs_props_name", "        autofocus" },
                            span { class: "rs_operator", ": " },
                            span { class: "rs_option_type", "Some" },
                            span { class: "rs_operator", "(" },
                            span { class: "rs_operator", "\"" },
                            span { class: "rs_prop_value", "false" },
                            span { class: "rs_operator", "\"" },
                            span { class: "rs_operator", ")" },
                            span { class: "rs_operator", "," },
                            span { class: "", "\n" },
                            span { class: "rs_props_name", "        disabled" },
                            span { class: "rs_operator", ": " },
                            span { class: "rs_option_type", "Some" },
                            span { class: "rs_operator", "(" },
                            span { class: "rs_operator", "\"" },
                            span { class: "rs_prop_value", "false" },
                            span { class: "rs_operator", "\"" },
                            span { class: "rs_operator", ")" },
                            span { class: "rs_operator", "," },
                            span { class: "", "\n" },
                            span { class: "rs_props_name", "        name" },
                            span { class: "rs_operator", ": " },
                            span { class: "rs_option_type", "Some" },
                            span { class: "rs_operator", "(" },
                            span { class: "rs_operator", "\"" },
                            span { class: "rs_prop_value", "" },
                            span { class: "rs_operator", "\"" },
                            span { class: "rs_operator", ")" },
                            span { class: "rs_operator", "," },
                            span { class: "", "\n" },
                            span { class: "rs_props_name", "        r#type" },
                            span { class: "rs_operator", ": " },
                            span { class: "rs_option_type", "Some" },
                            span { class: "rs_operator", "(" },
                            span { class: "rs_operator", "\"" },
                            span { class: "rs_prop_value", "" },
                            span { class: "rs_operator", "\"" },
                            span { class: "rs_operator", ")" },
                            span { class: "rs_operator", "," },
                            span { class: "", "\n" },
                            span { class: "rs_props_name", "        value" },
                            span { class: "rs_operator", ": " },
                            span { class: "rs_option_type", "Some" },
                            span { class: "rs_operator", "(" },
                            span { class: "rs_operator", "\"" },
                            span { class: "rs_prop_value", "" },
                            span { class: "rs_operator", "\"" },
                            span { class: "rs_operator", ")" },
                            span { class: "rs_operator", "," },
                            span { class: "", "\n" },
                            span { class: "rs_props_name", "        styles" },
                            span { class: "rs_operator", ": " },
                            span { class: "rs_option_type", "Some" },
                            span { class: "rs_operator", "(" },
                            span { class: "rs_operator", "\"" },
                            span { class: "rs_prop_value", "" },
                            span { class: "rs_operator", "\"" },
                            span { class: "rs_operator", ")" },
                            span { class: "rs_operator", "," },
                            span { class: "", "\n" },
                            span { class: "rs_operator", "    }}" },
                            span { class: "", "\n" },
                            span { class: "rs_fn_param", "    cx" },
                            span { class: "rs_operator", "." },
                            span { class: "rs_fn_object_member", "render" },
                            span { class: "rs_operator", "(" },
                            span { class: "rs_fn_object_member", "rsx!" },
                            span { class: "rs_operator", "{{" },
                            span { class: "", "\n" },
                            span { class: "rs_child_element", "        p " },
                            span { class: "rs_operator", "{{" },
                            span { class: "", "\n" },
                            span { class: "rs_element", "            Button " },
                            span { class: "rs_operator", "{{ " },
                            span { class: "rs_props_name", "button_props" },
                            span { class: "rs_operator", ": " },
                            span { class: "rs_variable_name", "bp " },
                            span { class: "rs_operator", " }}" },
                            span { class: "rs_operator", "," },
                            span { class: "", "\n" },
                            span { class: "rs_operator", "        }}" },
                            span { class: "", "\n" },
                            span { class: "rs_operator", "    }}" },
                            span { class: "rs_operator", ")" },
                            span { class: "", "\n" },
                            span { class: "rs_operator", "}}" },
                        }
                    }
                }
            }
        }
    })
}
