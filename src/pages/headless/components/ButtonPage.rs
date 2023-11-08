#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxycomp_headless::components::Button::Button;

#[inline_props]
pub fn ButtonPage(cx: Scope, name: String) -> Element {
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
                    Button {}
                }
            },
            div {
                class: "flex flex-wrap flex-row p-4 h-96 bg-neutral-800 rounded-xl",
                div {
                    class: "w-full h-8 pl-4 pb-12 text-white/40 border-b-2 border-white/30 ",
                    "Code: "
                },
                div {
                    class: "flex justify-start items-center", 
                    pre {
                        code {
                            class: "text-base",
                            "
                            use dioxycomp_headless::components::Button::Button;
                            //... Some other code here
                            
                            pub fn HomePage(cx: Scope, name: String) -> Element {{    
                                p {{
                                    Button {{}},
                                }}
                            }} 
                            "
                        }
                    }
                }
            }
        }
   })
}
