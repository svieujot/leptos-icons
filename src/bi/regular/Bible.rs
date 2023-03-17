#[cfg(feature = "BiRegularBible")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularBible")]
/// *This icon requires the feature* `BiRegularBible` *to be enabled*.
#[component]
pub fn Bible(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6 22h15v-2H6.012C5.55 19.988 5 19.805 5 19s.55-.988 1.012-1H21V4c0-1.103-.897-2-2-2H6c-1.206 0-3 .799-3 3v14c0 2.201 1.794 3 3 3zM5 8V5c0-.805.55-.988 1-1h13v12H5V8z" /><path d="M11 14h2v-4h2V8h-2V6h-2v2H9v2h2z" /></svg>
   }
}