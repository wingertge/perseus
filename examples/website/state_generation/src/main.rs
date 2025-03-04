use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use sycamore::prelude::*;

#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new().template(
        Template::new("post")
            .template_with_state(post_page)
            .build_paths_fn(get_build_paths)
            .build_state_fn(get_build_state)
            // Reload every blog post every day, in case it's changed
            .revalidate_after(Duration::new(60 * 60 * 24, 0))
            // If the user requests a page we haven't created yet, still
            // pass it to `get_build_state()` and cache the output for
            // future users (lazy page building)
            .incremental_generation(),
    )
}

// EXCERPT_START
#[perseus::template]
fn post_page<'a, G: Html>(cx: Scope<'a>, props: PostRx<'a>) -> View<G> {
    view! { cx,
        h1 { (props.title.get()) }
        p { (props.author.get()) }
        div(
            dangerously_set_inner_html = &props.content.get()
        )
    }
}
// EXCERPT_END

#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "PostRx")]
struct Post {
    title: String,
    author: String,
    content: String,
}

// EXCERPT_START
// This function will be run for each path under `/post/` to generate its state
#[engine_only_fn]
async fn get_build_state(
    StateGeneratorInfo { path, .. }: StateGeneratorInfo<()>,
) -> RenderFnResultWithCause<Post> {
    let raw_post = match get_post_for_path(path) {
        Ok(post) => post,
        // If the user sends us some bogus path with incremental generation,
        // return a 404 appropriately
        Err(err) => blame_err!(client, 404, err),
    };
    let html_content = parse_markdown(raw_post.content);
    let props = Post {
        title: raw_post.title,
        author: raw_post.author,
        content: html_content,
    };
    Ok(props)
}
#[engine_only_fn]
async fn get_build_paths() -> RenderFnResult<BuildPaths> {
    Ok(BuildPaths {
        // These will all become URLs at `/post/<name>`
        paths: vec![
            "welcome".to_string(),
            "really-popular-post".to_string(),
            "foobar".to_string(),
        ],
        // Perseus supports helper state, but we don't need it here
        extra: ().into(),
    })
}
// EXCERPT_END

// SNIP
fn get_post_for_path(_path: String) -> Result<Post, std::io::Error> {
    unimplemented!()
}
fn parse_markdown(_content: String) -> String {
    unimplemented!()
}
