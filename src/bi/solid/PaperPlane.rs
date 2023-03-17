#[cfg(feature = "BiSolidPaperPlane")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidPaperPlane")]
/// *This icon requires the feature* `BiSolidPaperPlane` *to be enabled*.
#[component]
pub fn PaperPlane(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m2.6 13.083 3.452 1.511L16 9.167l-6 7 8.6 3.916a1 1 0 0 0 1.399-.85l1-15a1.002 1.002 0 0 0-1.424-.972l-17 8a1.002 1.002 0 0 0 .025 1.822zM8 22.167l4.776-2.316L8 17.623z" /></svg>
   }
}