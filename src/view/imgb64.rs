use std::io::Write;

use base64::{engine::general_purpose, write::EncoderWriter};
use leptos::prelude::*;
use leptos_meta::*;
use js_sys::{JsString, Object, Reflect, Uint8Array};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::*;

pub async fn img_base64(f: File) -> Result<String, JsString> {
    match f.stream().get_reader().dyn_into::<ReadableStreamDefaultReader>() {
        Ok(reader) => {
            let mut encoder = EncoderWriter::new(Vec::new(), &general_purpose::STANDARD);
            loop {
                let chunk = match JsFuture::from(reader.read()).await {
                    Ok(chunk) => {
                        match chunk.dyn_into::<Object>() {
                            Ok(chunk) => chunk,
                            Err(_) => Err("dyn to Object")?
                        }
                    },
                    Err(_) => Err(JsString::from("read chunk"))?
                };
                // ReadableStreamReadResult is somehow wrong. So go by hand. Might be a web-sys bug.
                match Reflect::get(&chunk, &"done".into()) {
                    Ok(done) => {
                        if done.is_truthy() {
                            break;
                        }
                        let chunk = match Reflect::get(&chunk, &"value".into()) {
                            Ok(chunk) => {
                                match chunk.dyn_into::<Uint8Array>() {
                                    Ok(chunk) => chunk,
                                    Err(_) => Err(JsString::from("bytes are bytes"))?
                                }
                            },
                            Err(_) => Err(JsString::from("Get chunk"))?
                        };
                         let uint8_array = Uint8Array::new(&chunk);
                        // 流式写入，io::Error 需要转为 JsValue
                        encoder
                            .write_all(&uint8_array.to_vec())
                            .map_err(|e| JsString::from(e.to_string().as_str()))?;
                    },
                    Err(_) => Err(JsString::from("Get done"))?
                };
            }
            let buffer = encoder
            .finish()
            .map_err(|e| JsString::from(e.to_string().as_str()))?;
            String::from_utf8(buffer).map_err(|_| JsString::from("Invalid UTF-8 in base64 buffer"))
        },
        Err(e) => Err(e.to_string())
    }
}


#[component]
pub fn ImgBase64() -> impl IntoView {
    // let (file, set_file) = signal("".to_string());
    let (result, set_result) = signal("".to_string());
    let (result_url, set_result_url) = signal(None);
    let (msg, set_msg) = signal(None::<String>);

    let ext_list = vec!["JPEG", "JPG", "PNG", "WEBP", "GIF", "SVG"];
    let input_file = move |ev: Event| {
        set_result_url.set(None);
        set_msg.set(None);
        if let Some(target) = ev.target() {
            if let Some(files) = target.unchecked_ref::<web_sys::HtmlInputElement>().files() {
                if let Some(file) = files.get(0) {
                    let file_name = file.name();
                    let file_names = file_name.rsplitn(2, ".").collect::<Vec<&str>>();
                    if file_names.len() > 1 {
                        let ext = file_names[0].to_uppercase();
                        if ext_list.contains(&ext.as_str()) {
                            let content_type = if ext == "SVG" {
                                "data:image/svg+xml;base64,".to_string()
                            } else {
                                format!("data:image/{};base64,", ext.to_lowercase())
                            };
                            wasm_bindgen_futures::spawn_local(async move {
                                match img_base64(file).await {
                                    Ok(base_str) => {
                                        let result_str = format!("{}{}", content_type, base_str);
                                        set_result.set(result_str)
                                    },
                                    Err(e) => set_msg.set(e.as_string())
                                };
                            });
                        } else {
                           set_msg.set(Some("Unsupported format".to_string()));
                        }
                    } else {
                        set_msg.set(Some("Unknown format".to_string()));
                    };
                }
            }
        }
    };

    let preview = move |_| {
        set_result_url.set(None);
        set_msg.set(None);
        if !result.get().is_empty() {
            if result.get().starts_with("data:image/") {
                set_result_url.set(Some(format!("<img src='{}' />", result.get())));
            } else {
                set_msg.set(Some("Invalid Data URL, e.g., data:image/jpg;base64,iVBORellipsis...==".to_string()));
            }
        }
    };

    let input_result = move |ev| {
        set_result.set(event_target_value(&ev));
    };

    view! {
        <Title text="图片 Base64 - ITIT.Work" />
        <section class="bg-white shadow-md p-6">
            <h2>"图片 Base64（支持 JPG / PNG / GIF / SVG / WEBP，单图上传）"</h2>
            <input on:change=input_file class="block w-full text-sm text-gray-900 border border-gray-300 cursor-pointer bg-gray-50" id="file_input" type="file" />
            <label for="result" class="block mt-5 mb-2 text-sm font-medium text-gray-900">"结果（可以直接粘贴 base64 预览）"</label>
            <textarea id="result" on:input=input_result rows="11" class="w-full border border-gray-300 p-2 focus:outline-none focus:border-blue-500" prop:value=result></textarea>
            <div class="flex mt-5 justify-end gap-1">
               <button on:click=preview type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-5 py-2.5 me-2 mb-2">预览</button>
            </div>
            <Show
                when=move || { result_url.get().is_some() }
                fallback=|| view! { }
            >
                <div class="p-4 flex justify-center">
                    <div inner_html=result_url></div>
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