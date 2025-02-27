use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Md5() -> impl IntoView {
    let (data, set_data) = signal("".to_string());
    let (result, set_result) = signal("".to_string());

    let input_data = move |ev| {
        set_data.set(event_target_value(&ev));
    };

    let compute = move |_| {
        let origin_str = data.get();
        let compute_md5 = md5::compute(origin_str);
        let md5_str = format!("{:x}", compute_md5);
        let md5_str_16 = md5_str[8..24].to_string();
        let result_str = format!(
            "16 位小写：{}\n16 位大写：{}\n32 位小写：{}\n32 位大写：{}",
            md5_str_16,
            md5_str_16.to_uppercase(),
            md5_str,
            md5_str.to_uppercase()
        );
        set_result.set(result_str);
    };

    view! {
        <Title text="MD5 在线计算 - ITIT.Work" />
        <section class="bg-white shadow-md p-6">
        <h2>MD5 在线计算</h2>
        <label for="data" class="block mb-2 text-sm font-medium text-gray-900">数据</label>
        <textarea on:input=input_data id="data" rows="11" class="w-full border border-gray-300 p-2 focus:outline-none focus:border-blue-500"></textarea>
        <div class="flex mt-5 justify-end gap-1">
            <button on:click=compute type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">计算</button>
        </div>
        <label for="result" class="block mb-2 text-sm font-medium text-gray-900">结果</label>
        <textarea id="result" rows="11" readonly class="w-full border border-gray-300 p-2 focus:outline-none focus:border-blue-500" prop:value=result></textarea>
        </section>
    }
}
