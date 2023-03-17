#[cfg(feature = "RiCommunicationFillChatHeart")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiCommunicationFillChatHeart")]
/// *This icon requires the feature* `RiCommunicationFillChatHeart` *to be enabled*.
#[component]
pub fn ChatHeart(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M6.455 19L2 22.5V4a1 1 0 0 1 1-1h18a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1H6.455zm5.563-4.3l3.359-3.359a2.25 2.25 0 0 0-3.182-3.182l-.177.177-.177-.177a2.25 2.25 0 0 0-3.182 3.182l3.359 3.359z" /></g></svg>
   }
}