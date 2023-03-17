#[cfg(feature = "BiRegularFont")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularFont")]
/// *This icon requires the feature* `BiRegularFont` *to be enabled*.
#[component]
pub fn Font(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m11.307 4-6 16h2.137l1.875-5h6.363l1.875 5h2.137l-6-16h-2.387zm-1.239 9L12.5 6.515 14.932 13h-4.864z" /></svg>
   }
}