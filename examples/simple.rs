use rscx::{component, html, props};
use rscx_mdx::mdx::{Mdx, MdxComponentProps, MdxProps};

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
        <Mdx source=source handler=handle />
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

/// handle is called everytime a custom component is encountered.
/// The props are standardized in the MdxComponentProps struct and can be later parsed and used
/// to render the component.
async fn handle(name: String, props: MdxComponentProps) -> String {
    match name.as_str() {
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
