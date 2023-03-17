#[cfg(feature = "BiSolidVolumeLow")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidVolumeLow")]
/// *This icon requires the feature* `BiSolidVolumeLow` *to be enabled*.
#[component]
pub fn VolumeLow(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 17h2.697L14 21.868V2.132L6.697 7H4c-1.103 0-2 .897-2 2v6c0 1.103.897 2 2 2zM16 7v10c1.225-1.1 2-3.229 2-5s-.775-3.9-2-5z" /></svg>
   }
}