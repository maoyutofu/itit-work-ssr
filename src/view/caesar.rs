use leptos::prelude::*;
use leptos_meta::*;
use crate::util;

#[component]
pub fn Caesar() -> impl IntoView {
    let (shift, set_shift) = signal(3);
    let data = RwSignal::new("".to_string());
    let result = RwSignal::new("".to_string());

    let shift_input = move |ev| {
        if let Ok(val) = event_target_value(&ev).parse::<i32>() {
            set_shift.set(val);
        }
    };

    let input_data = move |ev| {
        data.set(event_target_value(&ev))
    };

    let encrypt = move |_| {
        let result_str = util::caesar_cipher(&data.get(), shift.get());
        result.set(result_str);
    };

    let decrypt = move |_| {
        let result_str = util::caesar_cipher(&data.get(), shift.get());
        result.set(result_str);
    };

    view! {
        <Title text="恺撒密码 - ITIT.Work" />
        <section class="bg-white shadow-md p-6">
        <h2>恺撒密码</h2>
        <label for="shift" class="w-24 text-sm font-medium text-gray-900 mb-2">"偏移量（正数加密，负数解密）"</label>
        <input id="shift" type="number" on:input=shift_input prop:value=shift class="bg-gray-50 border border-gray-300 text-gray-900 text-sm focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5" />
        <label for="data" class="block mb-2 text-sm font-medium text-gray-900">数据</label>
        <textarea on:input=input_data id="data" rows="11" class="w-full border border-gray-300 p-2 focus:outline-none focus:border-blue-500"></textarea>
        <div class="flex mt-5 justify-end gap-1">
            <button on:click=encrypt type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">加密</button>
            <button on:click=decrypt type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">解密</button>
        </div>
        <label for="result" class="block mb-2 text-sm font-medium text-gray-900">结果</label>
        <textarea id="result" rows="11"  class="w-full border border-gray-300 p-2 focus:outline-none focus:border-blue-500" prop:value=result></textarea>
        </section>
    }
}
