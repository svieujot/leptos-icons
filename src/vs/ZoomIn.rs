#[cfg(feature = "VsZoomIn")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsZoomIn")]
/// *This icon requires the feature* `VsZoomIn` *to be enabled*.
#[component]
pub fn ZoomIn(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M12.027 6.149a5.52 5.52 0 0 1-1.27 3.908l4.26 4.26-.7.71-4.26-4.27a5.52 5.52 0 1 1 1.97-4.608zm-5.45 4.888a4.51 4.51 0 0 0 3.18-1.32l-.04.02a4.51 4.51 0 0 0 1.36-3.2 4.5 4.5 0 1 0-4.5 4.5zm2.44-4v-1h-2v-2h-1v2h-2v1h2v2h1v-2h2z" /></svg>
   }
}