#[tokio::main]
async fn main() {
    slang_ui::run(slang_template::App).await;
}
