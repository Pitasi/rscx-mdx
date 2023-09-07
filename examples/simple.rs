use async_trait::async_trait;

use rsc_mdx::mdx::{Handler, Mdx, MdxComponentProps, MdxProps};
use rscx::{component, html, props};

#[tokio::main]
async fn main() {
    let source = r#"---
title: "Hello, world!"
---

# Hello, world!

This is a **markdown** file with some *content*, but also custom RSCx components!

<CustomTitle />

<Layout>

## subtitle

</Layout>

"#;

    let res = html! {
        <MyMdx source=source.into() />
    };

    println!("{}", res);
    // output ->
    //
    // <h1>Hello, world!</h1>
    // <p>This is a <strong>markdown</strong> file with some <em>content</em>, but also custom RSCx components!</p>
    // <h1>Some custom title!</h1>
    // <div class="layout">
    //     <h2>subtitle</h2>
    // </div>
}

/// MyHandler implements the Handler trait and will be used for rendering any custom HTML
/// component.
pub struct MyHandler {}

#[async_trait]
impl Handler for MyHandler {
    /// handle is called everytime a custom component is encountered.
    /// The props are standardized in the MdxComponentProps struct and can be later parsed and used
    /// to render the component.
    async fn handle(&self, name: &str, props: MdxComponentProps) -> String {
        match name {
            "CustomTitle" => html! {
                <CustomTitle />
            },
            "Layout" => html! {
                <Layout>
                    {props.children}
                </Layout>
            },
            _ => String::new(),
        }
    }
}

#[props]
pub struct MyMdxProps {
    source: String,
}

#[component]
/// MyMdx is a convenient wrapper for <Mdx /> with our custom handler initialized.
async fn MyMdx(props: MyMdxProps) -> String {
    let h = MyHandler {};
    html! {
        <Mdx source=props.source handler=Box::new(h) />
    }
}

// below we define some custom components that can be used in the markdown file

#[component]
fn CustomTitle() -> String {
    html! {
        <h1>Some custom title!</h1>
    }
}

#[props]
pub struct LayoutProps {
    children: String,
}

#[component]
fn Layout(props: LayoutProps) -> String {
    html! {
        <div class="layout">
            {props.children}
        </div>
    }
}
