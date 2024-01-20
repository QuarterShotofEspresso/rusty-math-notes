use leptos::{*, html::math};
use latex2mathml::{latex_to_mathml, DisplayStyle};


// Personal Reference: https://github.com/osanshouo/latex2mathml/blob/master/src/token.rs


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
    content: &'static str,
) -> impl IntoView {
    view! {
        <div class="journal-para dbg" inner_html=latex2mathml::replace(content).unwrap() />
        <Spacer vskip="0.5em"/>
    }
}


#[component]
fn Eq(
    #[prop(default = r#""#)]
    content: &'static str,
) -> impl IntoView {
    view! {
        <div class="journal-eq dbg" inner_html=latex2mathml::latex_to_mathml(content, DisplayStyle::Block).unwrap()/>
        <Spacer vskip="0.6em"/>
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



// #[component]
// fn SpacerTitle (
//     #[prop(default = "1em")]
//     vskip: &'static str,
//     #[prop(default =)]
// )


// #[macro_export]
// macro_rules! par {
//     ($s: expr) => {
//         Para(content: $s)
//         //  view! {<Para content=$s/>}
//     }
// }



#[component]
fn App() -> impl IntoView {
    // BUt we coudl also write the following interpretation:
    //     $$\sum_{i=0}^N \frac{a}{b}$$
    //     where $\delta: Q \times \Gamma \implies Q \times \Gamma \times \{L, R\}$.

    view! {
        <div class="dbg journal">
            <Spacer vskip="150px"/>
            <Title title="Rusty notes."/>
            <Para content=r#"Let $A$ and $B$ be two sets. The $\textit{Cartesian \; Product}$ is defined as the set of two-tuples contining every combination of elements from both sets. Formally, we write"#/>
            // par!("A Turing Machine $M$ has the following definition")
            <Eq content=r#"A \times B = \{(a, b) \; \colon \; a \in A, \, b \in B\}"#/>
            <Para content=r#"A $\textit{relation}$ between $A$ and $B$ is a subset $R \subseteq A \times B$. If $(a, b) \in R$, then "$a$ is related to $b$" and we can write $aRb$."#/>
            <Para content=r#"A function $\Phi \colon X \rightarrow Y$ is a relation on $X \times Y$ where elements are written as $(x, y)$ and $y = \Phi(x)$. In this form, $X$ is the $\textit{domain}$, and $Y$ is the $\textit{codomain}$. The $\textit{range}$ is defined as $\Phi[X] := \{\Phi(x) \colon x \in X\}$."#/>
        </div>
    }
}


fn main() {
    mount_to_body(|| view! { <App/> })
}