use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::{common::{Footer, Header}, view::{Ai, Base64, Index, Json, Md5, Uuid, Qrcode, Yaml, Toml, Aes, Timestamp, CaseConverter, Hex, Password, Mac, FileHash}};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
                <script>
                    var _hmt = _hmt || [];
                    (function () {
                    var hm = document.createElement("script");
                    hm.src = "https://hm.baidu.com/hm.js?24b2d92a8adad2c7ba01f40e54c057a3";
                    var s = document.getElementsByTagName("script")[0];
                    s.parentNode.insertBefore(hm, s);
                    })();
                </script>
            </head>
            <body class="bg-gray-100 mx-5 lg:mx-32">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/itit-work-ssr.css"/>

        // content for this welcome page
        <Router>
            <div>
                <header>
                    <Header />
                </header>
                <main>
                    <Routes fallback=|| "Page not found.".into_view()>
                        <Route path=StaticSegment("") view=Index />
                        <Route path=StaticSegment("/ai") view=Ai />
                        <Route path=StaticSegment("/base64") view=Base64 />
                        <Route path=StaticSegment("/json") view=Json />
                        <Route path=StaticSegment("/md5") view=Md5 />
                        <Route path=StaticSegment("/uuid") view=Uuid />
                        <Route path=StaticSegment("/qrcode") view=Qrcode />
                        <Route path=StaticSegment("/yaml") view=Yaml />
                        <Route path=StaticSegment("/toml") view=Toml />
                        <Route path=StaticSegment("/aes") view=Aes />
                        <Route path=StaticSegment("/timestamp") view=Timestamp />
                        <Route path=StaticSegment("/case-converter") view=CaseConverter />
                        <Route path=StaticSegment("/hex") view=Hex />
                        <Route path=StaticSegment("/password") view=Password />
                        <Route path=StaticSegment("/mac") view=Mac />
                        <Route path=StaticSegment("/file-hash") view=FileHash />
                    </Routes>
                </main>
                <footer>
                    <Footer />
                </footer>
            </div>
        </Router>
    }
}
