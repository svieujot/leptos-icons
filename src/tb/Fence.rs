#[cfg(feature = "TbFence")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbFence")]
/// *This icon requires the feature* `TbFence` *to be enabled*.
#[component]
pub fn Fence(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-fence" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M4 12v4h16v-4z" /><path d="M6 16v4h4v-4m0 -4v-6l-2 -2l-2 2v6" /><path d="M14 16v4h4v-4m0 -4v-6l-2 -2l-2 2v6" /></svg>
   }
}