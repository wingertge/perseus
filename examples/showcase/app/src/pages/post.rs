use serde::{Serialize, Deserialize};
use sycamore::prelude::{template, component, GenericNode, Template as SycamoreTemplate};
use perseus::template::Template;

#[derive(Serialize, Deserialize)]
pub struct PostPageProps {
    title: String,
    content: String,
}

#[component(PostPage<G>)]
pub fn post_page(props: PostPageProps) -> SycamoreTemplate<G> {
    let title = props.title;
    let content = props.content;
	template! {
		h1 {
            (title)
        }
        p {
            (content)
        }
	}
}

pub fn get_page<G: GenericNode>() -> Template<G> {
    Template::new("post")
        .build_paths_fn(Box::new(get_static_paths))
        .build_state_fn(Box::new(get_static_props))
        .incremental_path_rendering(true)
        .template(template_fn())
}

pub fn get_static_props(path: String) -> String {
    let path_vec: Vec<&str> = path.split('/').collect();
    let title_slug = path_vec[0];
    // This is just an example
    let title = urlencoding::decode(title_slug).unwrap();
    let content = format!("This is a post entitled '{}'. Its original slug was '{}'.", title, title_slug);

    serde_json::to_string(
        &PostPageProps {
            title: title.to_string(),
            content
        }
    ).unwrap()
}
// TODO
pub fn get_static_paths() -> Vec<String> {
    vec![
        "test".to_string()
    ]
}

pub fn template_fn<G: GenericNode>() -> perseus::template::TemplateFn<G> {
    Box::new(|props: Option<String>| template! {
        PostPage(
            serde_json::from_str::<PostPageProps>(&props.unwrap()).unwrap()
        )
    })
}