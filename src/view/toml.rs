use std::collections::HashMap;

use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Toml() -> impl IntoView {
    let (data, set_data) = signal("".to_string());
    let (result, set_result) = signal("".to_string());
    let (msg, set_msg) = signal(None);

    let input_data = move |ev| {
        set_data.set(event_target_value(&ev));
    };

    let to_json = move |_| match toml::from_str::<HashMap<String, toml::Value>>(&data.get()) {
        Ok(value) => match serde_json::to_string_pretty(&value) {
            Ok(json_str) => {
                set_result.set(json_str);
                set_msg.set(None);
            }
            Err(e) => set_msg.set(Some(e.to_string())),
        },
        Err(e) => set_msg.set(Some(e.to_string())),
    };
    let to_toml =
        move |_| match serde_json::from_str::<HashMap<String, serde_json::Value>>(&data.get()) {
            Ok(value) => match toml::to_string(&value) {
                Ok(json_str) => {
                    set_result.set(json_str);
                    set_msg.set(None);
                }
                Err(e) => set_msg.set(Some(e.to_string())),
            },
            Err(e) => set_msg.set(Some(e.to_string())),
        };

    view! {
        <Title text="TOML 与 JSON 在线转换 - itit.work" />
        <section class="bg-white shadow-md p-6">
        <h2 class="my-5">TOML 与 JSON 在线转换</h2>
        <label for="data" class="block mb-2 text-sm font-medium text-gray-900">数据</label>
        <textarea on:input=input_data id="data" rows="11" class="w-full border border-gray-300 p-2 focus:outline-none focus:border-blue-500"></textarea>
        <div class="flex mt-5 justify-end gap-1">
            <button on:click=to_json type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">转成 JSON</button>
            <button on:click=to_toml type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">转成 TOML</button>
        </div>
        <label for="result" class="block mb-2 text-sm font-medium text-gray-900">结果</label>
        <textarea id="result" rows="11" readonly class="w-full border border-gray-300 p-2 focus:outline-none focus:border-blue-500" prop:value=result></textarea>
        <Show
            when=move || { msg.get().is_some() }
            fallback=|| view! { }
        >
             <div class="p-4 mt-5 bg-yellow-100 text-yellow-800">
                <p>{msg}</p>
            </div>
        </Show>
        </section>
    }
}
