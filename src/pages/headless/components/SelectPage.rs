#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxycomp_headless::components::Select::Select;

#[inline_props]
pub fn SelectPage(cx: Scope, name: String) -> Element {
    cx.render(rsx! {
        section {
            id: "main_heading",
            h1 {
                class: "pb-6 text-4xl font-serif font-semibold",
                "Select"
            },
            hr {},
        },
        section {
            id: "description", 
            p {
                class: "text-xl",
                "The Select component typically displays a list with options and allows user to select one option.  
                Select consists of a label, a button which displays a selected value,
                and a listbox, displayed in a popover.
                Users can click/tap on the button to open the listbox popover.
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
                    Select {}
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
                            use dioxycomp_headless::components::Select::Select;
                            //... Some other code here
                            
                            pub fn HomePage(cx: Scope, name: String) -> Element {{    
                                p {{
                                    Select {{}},
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
