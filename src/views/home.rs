#![allow(non_snake_case)]

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_query;
use leptos_router::params::Params;

use crate::components::heading::{Header, Heading};
use crate::components::section::Section;

#[derive(Debug, Clone, PartialEq, Params)]
struct KanjiSearch {
    q: Option<String>,
}

pub fn Home() -> impl IntoView {
    let query = use_query::<KanjiSearch>();
    let kanji = move || {
        query
            .read()
            .as_ref()
            .ok()
            .and_then(|query| query.q.clone())
            .unwrap_or_default()
    };

    view! {
        <Title text="Home" />

        <Section class="items-center">
            <Header class="text-center">
                <Heading>"Kanji Diagram"</Heading>
            </Header>
            <form method="get" class="sticky top-12 md:top-2 flex flex-col gap-y-4 pb-4">
                <div class="flex flex-col gap-y-2 justify-center items-center">
                    <div class="flex flex-row gap-x-2">
                        <input
                            type="text"
                            name="q"
                            id="search"
                            class="bg-white dark:bg-zinc-800 min-w-[50vw] min-h-10 outline-none border rounded-sm border-zinc-500 px-2 py-1"
                        />
                        <input
                            type="submit"
                            value="Search"
                            class="flex flex-row justify-center items-center gap-x-2 \
                            px-4 py-2 min-w-24 lg:min-w-30 min-h-8 lg:min-h-10 rounded-sm \
                            text-sm font-medium \
                            whitespace-nowrap hover:cursor-pointer \
                            text-black dark:text-white \
                            bg-zinc-200 hover:bg-zinc-300 \
                            dark:bg-zinc-700 hover:dark:bg-zinc-800"
                        />
                    </div>
                </div>
            </form>

            <ul class="flex flex-col gap-y-4 w-full max-w-[100vw]">
                {kanji()
                    .chars()
                    .map(|c| {
                        view! {
                            <li class="min-h-24 overflow-x-auto text-center md:text-left whitespace-nowrap">
                                <h2 class="font-noto-sans-jp font-medium text-2xl pb-4">{c}</h2>
                                <div class="h-24 overflow-x-auto whitespace-nowrap">
                                    <img
                                        class="h-full w-auto object-contain block max-w-none"
                                        src=format!("./kanji/{:05x}.png", u32::from(c))
                                    />
                                </div>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </Section>
    }
}
