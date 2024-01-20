use leptos::{*, html::div};
use latex2mathml::{replace, latex_to_mathml, DisplayStyle};


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
        <div class="journal-para dbg" inner_html=replace(content).unwrap() />
        <Spacer vskip="0.5em"/>
    }
}


#[component]
fn Eq(
    #[prop(default = r#""#)]
    content: &'static str,
) -> impl IntoView {
    view! {
        <div class="journal-eq dbg" inner_html=latex_to_mathml(content, DisplayStyle::Block).unwrap()/>
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


macro_rules! eq {
    ($s: expr) => {
        Eq( EqProps { content: ($s) })
    }
}


macro_rules! title {
    ($s: expr) => {
        Title( TitleProps { title: ($s) })
    }
}


macro_rules! vskip {
    ($s: expr) => {
        Spacer( SpacerProps { vskip: ($s) })
    }
}


macro_rules! par {
    ($s: expr) => {
        Para( ParaProps { content: ($s) })
    }
}


macro_rules! base {
    ($($s: tt)*) => {
        div()
        .attr("class", "dbg journal")
        .child( ($($s)*) )
    };
}



#[component]
fn App() -> impl IntoView { base!(
    vskip!("150px"),
    title!("Rusty notes."),
    par!(r#"Let $A$ and $B$ be two sets. The $\textit{Cartesian \; Product}$ is defined as the set of two-tuples contining every combination of elements from both sets. Formally, we express this as"#),
    eq!(r#"A \times B = \{(a, b) \; \colon \; a \in A, \, b \in B\}"#),
    par!(r#"A $\textit{relation}$ between $A$ and $B$ is a subset $R \subseteq A \times B$. If $(a, b) \in R$, then "$a$ is related to $b$" and we can write $aRb$."#),
    par!(r#"A function $\phi \, \colon \, X \rightarrow Y$ is a relation on $X \times Y$ where elements are written as $(x, y)$ and $y = \phi(x)$. In this form, $X$ is the $\textit{domain}$, and $Y$ is the $\textit{codomain}$. The $\textit{range}$ is defined as $\phi[X] := \{\phi(x) \, \colon \, x \in X\}$."#),
    par!(r#"The number of elements $m$ in the set X represents the $\textit{cardinality}$ of $X$ and is represented by $|X|$."#),
    par!(r#"To demonstrate two sets $X$ and $Y$ have the same cardinality, we must map each element of $X$ to each element of $Y$. Such pairings/mappings are called a $\textit{one-to-one\;correspondance}$ (also called $\textit{injective}$ in some texts)."#),
    par!(r#"A function $\phi \, \colon \, X \rightarrow Y$ is injective if $ \forall x_1, x_2 \in X, \, \phi(x_1) = \phi(x_2)$, then $x_1 = x_2$. A function is $\textit{onto}$ (also reffered to as $\textit{susrjective}$ in some texts) if the range of $\phi$ is $Y$. That is, $\phi[X] = Y$. A function which is both surjective and injective is called $\textit{bijective}$."#),
    par!(r#"Bijective functions exhibit a property such that they have an inverse."#),
    par!(r#"$X$ and $Y$ have the same cardinality when $\exists \phi \, \colon \, X \rightarrow Y$ such that $\phi$ is both surjective and injective."#),
    par!(r#"An equivalence relation $R$ on a set $S$ is a relation (so $R \subseteq S \times S$), such that, $\forall x, y, z \in S$"#)
)}


fn main() {
    mount_to_body(|| view! { <App/> })
}