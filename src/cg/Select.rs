#[cfg(feature = "CgSelect")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgSelect")]
/// *This icon requires the feature* `CgSelect` *to be enabled*.
#[component]
pub fn Select(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M6 9.65685L7.41421 11.0711L11.6569 6.82843L15.8995 11.0711L17.3137 9.65685L11.6569 4L6 9.65685Z" fill="currentColor" /><path d="M6 14.4433L7.41421 13.0291L11.6569 17.2717L15.8995 13.0291L17.3137 14.4433L11.6569 20.1001L6 14.4433Z" fill="currentColor" /></svg>
   }
}