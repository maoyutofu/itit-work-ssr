use std::collections::HashMap;

use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Yaml() -> impl IntoView {
    let (data, set_data) = signal("".to_string());
    let (result, set_result) = signal("".to_string());
    let (msg, set_msg) = signal(None);

    let input_data = move |ev| {
        set_data.set(event_target_value(&ev));
    };

    let to_json =
        move |_| match serde_yml::from_str::<HashMap<String, serde_yml::Value>>(&data.get()) {
            Ok(value) => match serde_json::to_string_pretty(&value) {
                Ok(json_str) => {
                    set_result.set(json_str);
                    set_msg.set(None);
                }
                Err(e) => set_msg.set(Some(e.to_string())),
            },
            Err(e) => set_msg.set(Some(e.to_string())),
        };
    let to_yaml =
        move |_| match serde_json::from_str::<HashMap<String, serde_json::Value>>(&data.get()) {
            Ok(value) => match serde_yml::to_string(&value) {
                Ok(json_str) => {
                    set_result.set(json_str);
                    set_msg.set(None);
                }
                Err(e) => set_msg.set(Some(e.to_string())),
            },
            Err(e) => set_msg.set(Some(e.to_string())),
        };

    view! {
        <Title text="YAML 与 JSON 在线转换 - itit.work" />
        <section class="my-5">
        <h2 class="my-5">YAML 与 JSON 在线转换</h2>
        <label for="data" class="block mb-2 text-sm font-medium text-gray-900">数据</label>
        <textarea on:input=input_data id="data" rows="11" class="block p-2.5 w-full text-sm border-none text-white-900 bg-white-50 focus:ring-blue-500 focus:border-blue-500"></textarea>
        <div class="flex mt-5 justify-end gap-1">
            <button on:click=to_json type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">转成 JSON</button>
            <button on:click=to_yaml type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">转成 YAML</button>
        </div>
        <label for="result" class="block mb-2 text-sm font-medium text-gray-900">结果</label>
        <textarea id="result" rows="11" readonly class="block p-2.5 w-full text-sm border-none text-white-900 bg-white-50 focus:ring-blue-500 focus:border-blue-500" prop:value=result></textarea>
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
