use async_recursion::async_recursion;
use async_trait::async_trait;
use html_parser::{Dom, Element};
use rscx::{component, props};
use std::{collections::HashMap, future::Future, sync::Arc};

use crate::markdown::parse;

#[props]
pub struct MdxProps {
    #[builder(setter(into))]
    pub source: String,
    #[builder(setter(into))]
    pub handler: Box<dyn Handler + Send + Sync>,
}

impl<F, Fut> From<F> for Box<(dyn Handler + Send + Sync + 'static)>
where
    F: Fn(String, MdxComponentProps) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = String> + Send + Sync + 'static,
{
    fn from(handler: F) -> Self {
        Box::new(handler)
    }
}

#[component]
/// Renders a markdown source into a RSCx component.
/// Custom components can be used in the markdown source.
pub fn Mdx(props: MdxProps) -> String {
    let (_fm, html) = parse(&props.source).expect("invalid mdx");
    // TODO: we could expose frontmatter in the context so components can use its value

    let dom = Dom::parse(&html).expect("invalid html");
    let handler = Arc::new(props.handler);

    let mut root_views = vec![];
    for node in dom.children {
        if let Some(el) = node.element() {
            root_views.push(process_element(el, handler.clone()).await);
        }
    }

    root_views.join("")
}

/// Standardized props of a custom component.
pub struct MdxComponentProps {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attributes: HashMap<String, Option<String>>,
    pub children: String,
}

#[async_trait]
/// Handler is in charge of rendering custom components.
pub trait Handler {
    async fn handle(&self, component_name: String, props: MdxComponentProps) -> String;
}

#[async_trait]
impl<T, Fut> Handler for T
where
    T: Fn(String, MdxComponentProps) -> Fut + Sync,
    Fut: Future<Output = String> + Send + Sync,
{
    async fn handle(&self, component_name: String, props: MdxComponentProps) -> String {
        (self)(component_name, props).await
    }
}

#[async_recursion]
pub async fn process_element(
    el: &Element,
    custom_handler: Arc<Box<dyn Handler + Send + Sync>>,
) -> String {
    let mut child_views = vec![];
    for child in &el.children {
        match child {
            html_parser::Node::Element(el_child) => {
                child_views.push(process_element(el_child, custom_handler.clone()).await);
            }
            html_parser::Node::Text(text) => {
                child_views.push(text.clone());
            }
            _ => {}
        }
    }

    // Custom elements
    if is_component_tag_name(&el.name) {
        let cmp = custom_handler.handle(
            el.name.clone(),
            MdxComponentProps {
                id: el.id.clone(),
                classes: el.classes.clone(),
                attributes: el.attributes.clone(),
                children: child_views.join(""),
            },
        );
        return cmp.await;
    }

    // HTML elements
    el.source_span.text.clone()
}

fn is_component_tag_name(name: &str) -> bool {
    name.starts_with(|c: char| c.is_ascii_uppercase())
}
