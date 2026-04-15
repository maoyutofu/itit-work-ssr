
use leptos::prelude::*;
use leptos_meta::*;

use crate::util;
use crate::AI_JSON;


#[component]
pub fn Ai() -> impl IntoView {
    let data = util::get_data(AI_JSON);
    view! {
        <Title text="AI 大模型导航 - ITIT.Work" />
        {data.into_iter()
            .map(|s| view! { 
                <section class="my-5 bg-white shadow-md p-6">
                    <h2>{s.name}</h2>
                    <div
                        class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5  2xl:grid-cols-6 gap-5 mt-5">
                        {
                            s.items.into_iter().map(|item| view! {
                                <a href={item.url} target="_blank">
                                    <div
                                        class="flex items-center h-20 px-3 lg:px-5 bg-white hover:translate-y-[-1px] hover:shadow-lg transition-transform duration-300">
                                        <img class="w-10 h-10" src={item.icon} alt={item.name.clone()} />
                                        <div class="ml-3 truncate">
                                            <h3 class="text-sm truncate">{item.name}</h3>
                                            <p class="text-xs text-gray-400 truncate">{item.desc}</p>
                                        </div>
                                    </div>
                                </a>
                            }).collect_view()
                        }
                    </div>
                </section>
            })
            .collect_view()}
    }
}
