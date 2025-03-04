use perseus::{prelude::*, state::GlobalStateCreator};
use serde::{Deserialize, Serialize};

pub fn get_global_state_creator() -> GlobalStateCreator {
    GlobalStateCreator::new()
        .build_state_fn(get_build_state)
        .request_state_fn(get_request_state)
        .amalgamate_states_fn(amalgamate_states)
}

#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "AppStateRx")]
pub struct AppState {
    pub test: String,
}

// Global state will be generated for each locale in your app (but we don't
// worry about that in this example)
#[engine_only_fn]
async fn get_build_state(_locale: String) -> RenderFnResult<AppState> {
    Ok(AppState {
        test: "Hello from the build process!".to_string(),
    })
}

// This will be executed every time there's a request to any page in your app
// (you should avoid doing heavy work here if possible). Note that using *only*
// request-time global state generation, without anything at build-time, would
// prevent your app from accessing global state during the build process, so be
// certain that's what you want if you go down that path.
#[engine_only_fn]
async fn get_request_state(_locale: String, _req: Request) -> RenderFnResultWithCause<AppState> {
    Ok(AppState {
        test: "Hello from the server!".to_string(),
    })
}

// You can even combine build state with request state, just like in a template!
#[engine_only_fn]
async fn amalgamate_states(
    _locale: String,
    build_state: AppState,
    request_state: AppState,
) -> RenderFnResultWithCause<AppState> {
    Ok(AppState {
        test: format!(
            "Message from the builder: '{}' Message from the server: '{}'",
            build_state.test, request_state.test,
        ),
    })
}
