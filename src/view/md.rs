use leptos::prelude::*;
use leptos_meta::*;
use pulldown_cmark::{html, Parser};

#[component]
pub fn Md() -> impl IntoView {
    let (content, set_content) = signal(
        r#"# 标题

这是一个 **Markdown** 编辑器示例。

## 功能特点

- 左侧编辑
- 右侧实时预览
- 支持常用 Markdown 语法

## 代码示例

```rust
fn main() {
    println!("Hello, Markdown!");
}
```

> 这是一段引用文本

| 表格 | 示例 |
|------|------|
| 第一行 | 数据 |
| 第二行 | 内容 |

[访问 GitHub](https://github.com)
"#.to_string(),
    );

    let html_content = move || {
        let text = content.get();
        let parser = Parser::new(&text);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    };

    view! {
        <Title text="Markdown 编辑器 - ITIT.Work" />
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/github-markdown-css@5.8.1/github-markdown-light.min.css" />
        <section class="bg-white shadow-md p-6">
            <h2>Markdown 编辑器</h2>
            <div class="flex flex-col lg:flex-row gap-4 mt-5">
                <div class="w-full lg:w-1/2">
                    <label for="md-editor" class="block mb-2 text-sm font-medium text-gray-900">编辑</label>
                    <textarea
                        id="md-editor"
                        class="w-full lg:h-[calc(100vh-16rem)] border border-gray-300 p-3 focus:outline-none focus:border-blue-500 font-mono text-sm resize-none"
                        on:input=move |ev| {
                            set_content.set(event_target_value(&ev));
                        }
                        prop:value=content
                    ></textarea>
                </div>
                <div class="w-full lg:w-1/2">
                    <label class="block mb-2 text-sm font-medium text-gray-900">预览</label>
                    <div
                        id="md-preview"
                        class="markdown-body w-full lg:h-[calc(100vh-16rem)] border border-gray-300 p-3 overflow-auto"
                        inner_html=html_content
                    ></div>
                </div>
            </div>
        </section>
    }
}
