use verifier::App;

#[tokio::main]
async fn main() {
    slang_ui::run(App).await;
}
