#[cfg(feature = "IoWineSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoWineSharp")]
/// *This icon requires the feature* `IoWineSharp` *to be enabled*.
#[component]
pub fn WineSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M453,112V66.33H60.75V112L235.88,288V406H124.75v42H389V406H277.88V288Zm-336.65-3.67h281l-37.81,38H154.16Z" /></svg>
   }
}