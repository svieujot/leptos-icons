#[cfg(feature = "BiRegularCodeBlock")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCodeBlock")]
/// *This icon requires the feature* `BiRegularCodeBlock` *to be enabled*.
#[component]
pub fn CodeBlock(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 3H4c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2zM4 19V7h16l.002 12H4z" /><path d="M9.293 9.293 5.586 13l3.707 3.707 1.414-1.414L8.414 13l2.293-2.293zm5.414 0-1.414 1.414L15.586 13l-2.293 2.293 1.414 1.414L18.414 13z" /></svg>
   }
}