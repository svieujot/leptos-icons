#[cfg(feature = "FaSolidL")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidL")]
/// *This icon requires the feature* `FaSolidL` *to be enabled*.
#[component]
pub fn L(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512"><path d="M64 32c17.7 0 32 14.3 32 32V416H288c17.7 0 32 14.3 32 32s-14.3 32-32 32H64c-17.7 0-32-14.3-32-32V64c0-17.7 14.3-32 32-32z" /></svg>
   }
}