#[cfg(feature = "BiRegularJoystickAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularJoystickAlt")]
/// *This icon requires the feature* `BiRegularJoystickAlt` *to be enabled*.
#[component]
pub fn JoystickAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><circle cx="15" cy="13" r="1" /><circle cx="17" cy="11" r="1" /><path d="M10 9H8v2H6v2h2v2h2v-2h2v-2h-2z" /><path d="M15 5H9a7 7 0 0 0-7 7 7 7 0 0 0 7 7h6a7 7 0 0 0 7-7 7 7 0 0 0-7-7zm0 12H9A5 5 0 1 1 9 7h6a5 5 0 1 1 0 10z" /></svg>
   }
}