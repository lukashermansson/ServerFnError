use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/server-fn-broken.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}
#[server()]
pub async fn test() -> Result<(), ServerFnError> {
    Err(ServerFnError::new("Not found"))
}
/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
let (count, _set_count) = create_signal(0);
 let _resource = create_resource(count, |_| async move { test().await });

    view! {
        <h1>"Welcome to Leptos!"</h1>
    }
}
