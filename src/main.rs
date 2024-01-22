use leptos::{*, html::div};
use latex2mathml::replace;


#[component]
fn Title(
    #[prop(default = "")]
    title: &'static str
) -> impl IntoView {
    view! {
        <div class="title dbg">{title}</div>
        <Spacer vskip="1em"/>
    }
}


#[component]
fn Para(
    #[prop(default = r#""#)]
    contents: &'static str,
) -> impl IntoView {

    view! {
        <div class="journal-para dbg" inner_html={replace(contents).unwrap()}/>
        <Spacer vskip="0.5em"/>
    }
}


#[component]
fn Spacer(
    #[prop(default = "1em")]
    vskip: &'static str,
) -> impl IntoView {
    view! {
        <div style:height=vskip class="spacer dbg">
            // "journal-spacer"
        </div>
    }
}


macro_rules! title {
    ($s: expr) => {
        Title( TitleProps { title: ($s) })
    }
}


macro_rules! space {
    ($s: expr) => {
        Spacer( SpacerProps { vskip: ($s) })
    }
}


macro_rules! par {
    ($s: expr) => {
        Para( ParaProps { contents: include_str!($s) })
    }
}


macro_rules! doc {
    ($($s: tt)*) => {
        div()
        .attr("class", "dbg journal")
        .child( ($($s)*) )
    };
}



#[component]
fn App() -> impl IntoView { 
    doc!(
        space!("150px"),
        title!("Rusty notes."),
        par!("abstractalgebra-171.tex"),
    )
}


fn main() {
    mount_to_body(|| view! { <App/> })
}