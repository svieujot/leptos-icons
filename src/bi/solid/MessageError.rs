#[cfg(feature = "BiSolidMessageError")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMessageError")]
/// *This icon requires the feature* `BiSolidMessageError` *to be enabled*.
#[component]
pub fn MessageError(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 2H4c-1.103 0-2 .894-2 1.992v12.016C2 17.106 2.897 18 4 18h3v4l6.351-4H20c1.103 0 2-.894 2-1.992V3.992A1.998 1.998 0 0 0 20 2zm-7 13h-2v-2h2v2zm0-4h-2V5h2v6z" /></svg>
   }
}