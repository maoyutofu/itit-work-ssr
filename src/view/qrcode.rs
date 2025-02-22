use leptos::prelude::*;
use leptos_meta::*;
use qrcode::render::svg;
use qrcode::QrCode;

#[component]
pub fn Qrcode() -> impl IntoView {
    let (data, set_data) = signal("".to_string());
    let (result, set_result) = signal(None);
    let (msg, set_msg) = signal(None);

    let input_data = move |ev| {
        set_data.set(event_target_value(&ev));
    };

    let generate = move |_| {
        match QrCode::new(data.get().as_bytes()).map(|code| {
            code.render::<svg::Color>()
                .min_dimensions(200, 200)
                .quiet_zone(false)
                .dark_color(svg::Color("#000000"))
                .light_color(svg::Color("#ffffff"))
                .build()
        }) {
            Ok(image) => {
                set_result.set(Some(image));
            }
            Err(e) => {
                set_msg.set(Some(e.to_string()));
            }
        };
    };

    view! {
        <Title text="二维码在线生成 - ITIT.Work" />
        <section class="bg-white shadow-md p-6">
            <h2 class="my-5">二维码在线生成</h2>
            <label for="number-input" class="block mb-2 text-sm font-medium text-gray-900">数据</label>
            <textarea on:input=input_data id="data" rows="11" class="w-full border border-gray-300 p-2 focus:outline-none focus:border-blue-500"></textarea>
            <div class="flex mt-5 justify-end gap-1">
                <button on:click=generate type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">生成</button>
            </div>
            <Show
                when=move || { result.get().is_some() }
                fallback=|| view! { }
            >
                <div class="p-4 flex justify-center">
                    <div inner_html=result></div>
                </div>
            </Show>

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
