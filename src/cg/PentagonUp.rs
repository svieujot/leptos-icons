#[cfg(feature = "CgPentagonUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPentagonUp")]
/// *This icon requires the feature* `CgPentagonUp` *to be enabled*.
#[component]
pub fn PentagonUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M12 16L17 18V8L12 6L7 8V18L12 16ZM9 15.0459L12 13.8459L15 15.0459V9.35407L12 8.15407L9 9.35407V15.0459Z" fill="currentColor" /></svg>
   }
}