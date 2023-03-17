#[cfg(feature = "TbCircleChevronsDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbCircleChevronsDown")]
/// *This icon requires the feature* `TbCircleChevronsDown` *to be enabled*.
#[component]
pub fn CircleChevronsDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-circle-chevrons-down" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M15 9l-3 3l-3 -3" /><path d="M15 13l-3 3l-3 -3" /><path d="M12 3a9 9 0 1 0 0 18a9 9 0 0 0 0 -18z" /></svg>
   }
}