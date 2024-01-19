use leptos::{*, html::math};
use latex2mathml::{latex_to_mathml, DisplayStyle};

fn main() {

    let latex = r#"\erf ( x ) = \frac{ 2 }{ \sqrt{ \pi } } \int_0^x e^{- t^2} \, dt"#;
    let mathml = latex_to_mathml(latex, DisplayStyle::Block).unwrap();
    println!("{}", mathml);

    // mount_to_body(|| view! { <p>"Hello, world!"</p> })
    // mount_to_body(|| view! { <p>{mathml}</p> })
    mount_to_body(|| view! { <App/> })

}

#[component]
fn App() -> impl IntoView {
    // let (count, set_count) = create_signal(0);
    let latex = r#"\erf ( x ) = \frac{ 2 }{ \sqrt{ \pi } } \int_0^x e^{- t^2} \, dt"#;
    let mathml = latex_to_mathml(latex, DisplayStyle::Block).unwrap();

    view! {
        // <button
        //     on:click=move |_| set_count.update(|count| *count += 1);
        // >
        //     "Click me: "
        //     {move || count.get()}
        // </button>
        <Spacer/>
        <div class="border-style journal" inner_html=mathml/>
    }
}


#[component]
fn Spacer() -> impl IntoView {
    view! {
        <div class="spacer border-style">
            "spacer"
        </div>
    }
}