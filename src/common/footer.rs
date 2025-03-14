use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <section class="my-5">
            <h2>友情连接</h2>
            <div class="mt-5 px-5 py-5 bg-white">
                <ul class="flex items-start list-disc">
                    <li class="text-xs">
                        <a href="https://jizhong.plus/" target="_blank">"Jizhong's blog"</a>
                    </li>
                </ul>
            </div>
        </section>
        <div class="flex gap-2">
            <div class="text-sm">"©" 2024 <a href="https://itit.work/" class="ml-2">ITIT.Work</a></div>
            <div class="text-sm"> 由 <a href="https://www.rust-lang.org/">Rust</a>"、"<a href="https://leptos.dev/">Leptos</a> 强力驱动</div>
            <div class="text-sm">
                <a class="vp-repo-link" href="https://github.com/maoyutofu/itit-work" target="_blank" rel="noopener noreferrer" aria-label="GitHub">
                    <svg xmlns="http://www.w3.org/2000/svg" class="icon github-icon w-4 h-4" viewBox="0 0 1024 1024" fill="currentColor" aria-label="github icon" style="width:1.25rem;height:1.25rem;vertical-align:middle;"><path d="M511.957 21.333C241.024 21.333 21.333 240.981 21.333 512c0 216.832 140.544 400.725 335.574 465.664 24.49 4.395 32.256-10.07 32.256-23.083 0-11.69.256-44.245 0-85.205-136.448 29.61-164.736-64.64-164.736-64.64-22.315-56.704-54.4-71.765-54.4-71.765-44.587-30.464 3.285-29.824 3.285-29.824 49.195 3.413 75.179 50.517 75.179 50.517 43.776 75.008 114.816 53.333 142.762 40.79 4.523-31.66 17.152-53.377 31.19-65.537-108.971-12.458-223.488-54.485-223.488-242.602 0-53.547 19.114-97.323 50.517-131.67-5.035-12.33-21.93-62.293 4.779-129.834 0 0 41.258-13.184 134.912 50.346a469.803 469.803 0 0 1 122.88-16.554c41.642.213 83.626 5.632 122.88 16.554 93.653-63.488 134.784-50.346 134.784-50.346 26.752 67.541 9.898 117.504 4.864 129.834 31.402 34.347 50.474 78.123 50.474 131.67 0 188.586-114.73 230.016-224.042 242.09 17.578 15.232 33.578 44.672 33.578 90.454v135.85c0 13.142 7.936 27.606 32.854 22.87C862.25 912.597 1002.667 728.747 1002.667 512c0-271.019-219.648-490.667-490.71-490.667z"></path></svg>
                </a>
            </div>
        </div>
    }
}
