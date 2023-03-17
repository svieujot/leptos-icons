#[cfg(feature = "AiFilledWindows")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiFilledWindows")]
/// *This icon requires the feature* `AiFilledWindows` *to be enabled*.
#[component]
pub fn Windows(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 1024 1024"><path d="M523.8 191.4v288.9h382V128.1zm0 642.2l382 62.2v-352h-382zM120.1 480.2H443V201.9l-322.9 53.5zm0 290.4L443 823.2V543.8H120.1z" /></svg>
   }
}