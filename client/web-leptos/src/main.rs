use entertainarr_client_web_leptos::RootComponent;
use leptos::prelude::*;
use leptos_router::components::*;
use tracing_subscriber::fmt::writer::MakeWriterExt;

fn main() {
    console_error_panic_hook::set_once();

    use tracing_subscriber::fmt;
    use tracing_subscriber_wasm::MakeConsoleWriter;

    fmt()
        .with_writer(
            // To avoide trace events in the browser from showing their
            // JS backtrace, which is very annoying, in my opinion
            MakeConsoleWriter::default().with_max_level(tracing::Level::DEBUG),
        )
        // For some reason, if we don't do this in the browser, we get
        // a runtime error.
        .without_time()
        .with_ansi(false)
        .init();

    leptos::mount::mount_to_body(|| {
        view! {
            <Router>
                <RootComponent />
            </Router>
        }
    });
}
