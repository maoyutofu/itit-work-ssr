use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Wx() -> impl IntoView {
    view! {
        <Title text="Markdown 转微信公众号排版 - ITIT.Work" />
        <script src="https://cdn.jsdelivr.net/npm/marked@9.1.6/marked.min.js"></script>
        <section class="bg-white shadow-md p-6">
            <h2 class="text-2xl font-bold mb-4">"Markdown 转微信公众号排版"</h2>
            <p class="text-gray-600 mb-4">"左侧编辑 Markdown，右侧实时预览，点击复制内容按钮即可复制"</p>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div class="flex flex-col h-[600px]">
                    <label class="font-semibold mb-2">"Markdown 编辑器"</label>
                    <textarea
                        id="md-input"
                        class="w-full flex-1 p-4 border border-gray-300 rounded-lg font-mono text-sm resize-none focus:outline-none focus:ring-2 focus:ring-blue-500"
                        placeholder="在此输入 Markdown 内容..."
                    ></textarea>
                </div>

                <div class="flex flex-col h-[600px]">
                    <div class="flex justify-between items-center mb-2 shrink-0 flex-wrap gap-2">
                        <label class="font-semibold">"微信公众号预览"</label>
                        <div class="flex items-center gap-3 flex-wrap">
                            <label class="flex items-center gap-1 text-sm">
                                "主题:"
                                <select id="theme-select" onchange="updatePreview()" class="border rounded px-1 py-0.5 text-sm">
                                    <option value="default">默认</option>
                                    <option value="blue">蓝色标题</option>
                                    <option value="green">绿色标题</option>
                                    <option value="purple">紫色标题</option>
                                    <option value="red">红色标题</option>
                                    <option value="gray">灰色标题</option>
                                </select>
                            </label>
                            <label class="flex items-center gap-1 text-sm">
                                "字体:"
                                <select id="font-select" onchange="updatePreview()" class="border rounded px-1 py-0.5 text-sm">
                                    <option value="sans-serif">无衬线</option>
                                    <option value="serif">衬线</option>
                                    <option value="monospace">等宽</option>
                                </select>
                            </label>
                            <label class="flex items-center gap-1 text-sm">
                                "字号:"
                                <select id="size-select" onchange="updatePreview()" class="border rounded px-1 py-0.5 text-sm">
                                    <option value="14">小</option>
                                    <option value="16" selected>中(推荐)</option>
                                    <option value="18">大</option>
                                    <option value="20">特大</option>
                                </select>
                            </label>
                            <button onclick="copyWxHtml()" type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium text-sm px-3 py-1.5">复制</button>
                        </div>
                    </div>
                    <div
                        id="wx-preview"
                        class="flex-1 overflow-auto p-4 border border-gray-300 rounded-lg bg-white"
                    ></div>
                </div>
            </div>

            <div class="mt-4 p-4 bg-gray-50 rounded-lg">
                <h3 class="font-semibold mb-2">"使用说明"</h3>
                <ul class="text-sm text-gray-600 space-y-1">
                    <li>"预览样式完全兼容微信公众号，可直接复制使用"</li>
                    <li>"代码块使用等宽字体，带灰色背景"</li>
                    <li>"链接为蓝色下划线样式"</li>
                    <li>"表格带边框和斑马纹背景"</li>
                    <li>"可切换主题（标题背景色）、字体（无衬线/衬线/等宽）、字号（小/中(推荐)/大/特大）"</li>
                </ul>
            </div>
        </section>

        <script>{r#"
            // Marked.js configuration
            if (typeof marked !== 'undefined') {
                marked.setOptions({
                    breaks: true,
                    gfm: true,
                });
            }

            // Theme configurations
            var themes = {
                default: { h1Bg: 'transparent', h2Bg: 'transparent', h3Bg: 'transparent', h4Bg: 'transparent' },
                blue: { h1Bg: '#e6f0ff', h2Bg: '#cce0ff', h3Bg: '#b3d1ff', h4Bg: '#99c2ff' },
                green: { h1Bg: '#e6f7e6', h2Bg: '#ccefcc', h3Bg: '#b3e6b3', h4Bg: '#99d999' },
                purple: { h1Bg: '#f0e6ff', h2Bg: '#e0ccff', h3Bg: '#d1b3ff', h4Bg: '#c299ff' },
                red: { h1Bg: '#ffe6e6', h2Bg: '#ffcccc', h3Bg: '#ffb3b3', h4Bg: '#ff9999' },
                gray: { h1Bg: '#f5f5f5', h2Bg: '#e8e8e8', h3Bg: '#dbdbdb', h4Bg: '#cecece' },
            };

            // Font families
            var fontFamilies = {
                'sans-serif': '-apple-system, BlinkMacSystemFont, "Segoe UI", "PingFang SC", "Hiragino Sans GB", "Microsoft YaHei", "Helvetica Neue", Helvetica, Arial, sans-serif',
                'serif': 'Georgia, "Times New Roman", "SimSun", serif',
                'monospace': '"SF Mono", Menlo, Monaco, Courier New, monospace',
            };


            function getCurrentSettings() {
                var theme = document.getElementById('theme-select') ? document.getElementById('theme-select').value : 'default';
                var font = document.getElementById('font-select') ? document.getElementById('font-select').value : 'sans-serif';
                var size = document.getElementById('size-select') ? document.getElementById('size-select').value : '16';
                return {
                    theme: themes[theme] || themes.default,
                    fontFamily: fontFamilies[font] || fontFamilies['sans-serif'],
                    fontSize: size + 'px',
                };
            }

            // WeChat-compatible styles
            function buildWxStyles(settings) {
                return {
                    base: 'font-family: ' + settings.fontFamily + '; font-size: ' + settings.fontSize + '; line-height: 1.8; color: #3e3e3e; max-width: 100%; word-wrap: break-word;',
                    h1: 'font-size: 28px; font-weight: bold; margin: 24px 0 16px 0; padding: 12px; border-bottom: 1px solid #e8e8e8; color: #333; background: ' + settings.theme.h1Bg + ';',
                    h2: 'font-size: 22px; font-weight: bold; margin: 20px 0 14px 0; padding: 10px; border-bottom: 1px solid #e8e8e8; color: #333; background: ' + settings.theme.h2Bg + ';',
                    h3: 'font-size: 18px; font-weight: bold; margin: 16px 0 10px 0; padding: 8px; color: #333; background: ' + settings.theme.h3Bg + ';',
                    h4: 'font-size: 16px; font-weight: bold; margin: 14px 0 8px 0; padding: 6px; color: #333; background: ' + settings.theme.h4Bg + ';',
                    p: 'margin: 12px 0; line-height: 1.8;',
                    a: 'color: #576b95; text-decoration: underline;',
                    ul: 'margin: 12px 0; padding-left: 30px;',
                    ol: 'margin: 12px 0; padding-left: 30px;',
                    li: 'margin: 6px 0; line-height: 1.8;',
                    blockquote: 'margin: 12px 0; padding: 12px 16px; background: #f8f8f8; border-left: 4px solid #576b95; color: #666;',
                    pre: 'margin: 12px 0; padding: 16px; overflow-x: auto; background: #f6f8fa; border-radius: 4px; border: 1px solid #e8e8e8; font-family: ' + fontFamilies.monospace + '; font-size: 14px;',
                    code: 'font-family: ' + fontFamilies.monospace + '; font-size: 14px; background: #f6f8fa; border-radius: 4px; margin: 0 4px; padding: 2px 6px; border: 1px solid #e8e8e8;',
                    inlineCode: 'font-family: ' + fontFamilies.monospace + '; font-size: 14px; background: #f6f8fa; border-radius: 4px; padding: 2px 6px; border: 1px solid #e8e8e8;',
                    hr: 'margin: 20px 0; border: none; border-top: 1px solid #e8e8e8;',
                    img: 'max-width: 100%; height: auto; margin: 12px 0; border-radius: 4px;',
                    table: 'width: 100%; border-collapse: collapse; margin: 12px 0; font-size: 14px;',
                    th: 'padding: 10px 14px; border: 1px solid #e8e8e8; text-align: left; background: #f6f8fa; font-weight: bold;',
                    td: 'padding: 10px 14px; border: 1px solid #e8e8e8; text-align: left;',
                    tr: 'background: #fafafa;',
                };
            }

            function applyWxStyles(container) {
                if (!container) return;
                var settings = getCurrentSettings();
                var wxStyles = buildWxStyles(settings);
                container.style.cssText = wxStyles.base;
                container.querySelectorAll('h1').forEach(function(el) { el.style.cssText = wxStyles.h1; });
                container.querySelectorAll('h2').forEach(function(el) { el.style.cssText = wxStyles.h2; });
                container.querySelectorAll('h3').forEach(function(el) { el.style.cssText = wxStyles.h3; });
                container.querySelectorAll('h4').forEach(function(el) { el.style.cssText = wxStyles.h4; });
                container.querySelectorAll('p').forEach(function(el) { el.style.cssText = wxStyles.p; });
                container.querySelectorAll('a').forEach(function(el) { el.style.cssText = wxStyles.a; });
                container.querySelectorAll('ul').forEach(function(el) { el.style.cssText = wxStyles.ul; });
                container.querySelectorAll('ol').forEach(function(el) { el.style.cssText = wxStyles.ol; });
                container.querySelectorAll('li').forEach(function(el) { el.style.cssText = wxStyles.li; });
                container.querySelectorAll('blockquote').forEach(function(el) { el.style.cssText = wxStyles.blockquote; });
                container.querySelectorAll('pre').forEach(function(el) { el.style.cssText = wxStyles.pre; });
                container.querySelectorAll('code').forEach(function(el) {
                    if (el.parentElement.tagName !== 'PRE') {
                        el.style.cssText = wxStyles.inlineCode;
                    }
                });
                container.querySelectorAll('hr').forEach(function(el) { el.style.cssText = wxStyles.hr; });
                container.querySelectorAll('img').forEach(function(el) { el.style.cssText = wxStyles.img; });
                container.querySelectorAll('table').forEach(function(el) { el.style.cssText = wxStyles.table; });
                container.querySelectorAll('th').forEach(function(el) { el.style.cssText = wxStyles.th; });
                container.querySelectorAll('td').forEach(function(el) { el.style.cssText = wxStyles.td; });
                container.querySelectorAll('tr:nth-child(even)').forEach(function(el) { el.style.cssText = wxStyles.tr; });
            }

            function updatePreview() {
                var input = document.getElementById('md-input');
                var preview = document.getElementById('wx-preview');
                if (!input || !preview) return;
                var md = input.value;
                if (typeof marked !== 'undefined') {
                    var html = marked.parse(md);
                    preview.innerHTML = html;
                    applyWxStyles(preview);
                } else {
                    preview.innerHTML = '<p style="color: #999;">加载中...</p>';
                }
            }

            function copyWxHtml() {
                var preview = document.getElementById('wx-preview');
                if (!preview) return;
                var html = preview.innerHTML;
                var text = preview.innerText;
                if (navigator.clipboard && navigator.clipboard.write) {
                    var clipboardItem = new ClipboardItem({
                        'text/html': new Blob([html], { type: 'text/html' }),
                        'text/plain': new Blob([text], { type: 'text/plain' }),
                    });
                    navigator.clipboard.write([clipboardItem]).then(function() {
                        alert('已复制到剪贴板!');
                    }).catch(function() {
                        alert('复制失败，请手动复制');
                    });
                } else {
                    navigator.clipboard.writeText(html).then(function() {
                        alert('已复制到剪贴板!');
                    }).catch(function() {
                        alert('复制失败，请手动复制');
                    });
                }
            }

            document.addEventListener('DOMContentLoaded', function() {
                var input = document.getElementById('md-input');
                if (input) {
                    input.value = '# 微信公众号排版示例\n\n这是一段**加粗文字**和*斜体文字*的示例。\n\n## 功能特点\n\n- 支持 Markdown 语法\n- 实时预览效果\n- 微信公众号标准样式\n- 可直接复制使用\n\n## 代码示例\n\n```javascript\nfunction hello() {\n    console.log("Hello, World!");\n}\n```\n\n> 这是一段引用文字，用于强调重要内容。\n\n### 链接\n\n[点击访问我的博客](https://example.com)\n\n## 表格示例\n\n| 名称 | 描述 |\n|------|------|\n| 项目一 | 这是一个示例 |\n| 项目二 | 另一个示例 |\n\n---\n\n**最后更新时间：** 2026年4月';
                    input.addEventListener('input', updatePreview);
                    updatePreview();
                }
            });
        "#}</script>
    }
}
