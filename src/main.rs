use leptos::{*, html::math};
use latex2mathml::{latex_to_mathml, DisplayStyle};

fn main() {

    let latex = r#"\sin ( x ) = \frac{ 2 }{ \sqrt{ \pi } } \int_0^x e^{- t^2} \, dt"#;
    let mathml = latex_to_mathml(latex, DisplayStyle::Block).unwrap();
    println!("{}", mathml);

    // mount_to_body(|| view! { <p>"Hello, world!"</p> })
    // mount_to_body(|| view! { <p>{mathml}</p> })
    mount_to_body(|| view! { <App/> })

}

#[component]
fn App() -> impl IntoView {
    // let (count, set_count) = create_signal(0);
    // let latex = r#"\sin x = \frac{ (2) }{ \sqrt{ \pi } } \int_0^x (e^{- t^2}) \, dt"#;
    // let mathml = latex_to_mathml(latex, DisplayStyle::Block).unwrap();

    let latex = r#"M = \left( Q, \Sigma, \Gamma, \delta, q_0, q_{\textit{acc}}, q_{\textit{rej}} \right)"#;
    let mathml = latex2mathml::latex_to_mathml(latex, DisplayStyle::Block).unwrap();

    // BUt we coudl also write the following interpretation:
    //     $$\sum_{i=0}^N \frac{a}{b}$$
    //     where $\delta: Q \times \Gamma \implies Q \times \Gamma \times \{L, R\}$.

    view! {
        <div class="dbg journal">
            <Spacer vskip="150px"/>
            <Title/>
            <Spacer vskip="1em"/>
            <div class="journal-para dbg" inner_html=latex2mathml::replace("A Turing Machine $M$ has the following definition").unwrap() />
            <Spacer vskip="0.5em"/>
            <div class="journal-eq dbg" inner_html=mathml/>
        </div>
    }
}


#[component]
fn Title() -> impl IntoView {
    view! {
        <div class="title dbg">
            "Rusty notes."
        </div>
    }
}


#[component]
fn Spacer(
    #[prop(default = "1em")]
    vskip: &'static str,
) -> impl IntoView {
    view! {
        // <div style:height=format!("{}px", vskip) class="journal-spacer dbg">
        <div style:height=vskip class="spacer dbg">
            // "journal-spacer"
        </div>
    }
}