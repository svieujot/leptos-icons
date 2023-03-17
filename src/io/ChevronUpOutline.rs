#[cfg(feature = "IoChevronUpOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoChevronUpOutline")]
/// *This icon requires the feature* `IoChevronUpOutline` *to be enabled*.
#[component]
pub fn ChevronUpOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="112 328 256 184 400 328" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:48px" /></svg>
   }
}