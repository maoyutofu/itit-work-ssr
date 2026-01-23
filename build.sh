# 构建前端（如果有独立 client 部分）
cargo leptos build --release   # 或 trunk build --release

# 构建 server 为 wasm
cargo build --release \
  --target wasm32-unknown-unknown \
  --no-default-features \
  --features ssr

# 本地开发
wrangler dev --local --persist

# 正式部署
wrangler deploy