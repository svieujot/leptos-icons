#[cfg(feature = "RiSystemFillRefresh")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillRefresh")]
/// *This icon requires the feature* `RiSystemFillRefresh` *to be enabled*.
#[component]
pub fn Refresh(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 22C6.477 22 2 17.523 2 12S6.477 2 12 2s10 4.477 10 10-4.477 10-10 10zm4.82-4.924A7 7 0 0 0 9.032 5.658l.975 1.755A5 5 0 0 1 17 12h-3l2.82 5.076zm-1.852 1.266l-.975-1.755A5 5 0 0 1 7 12h3L7.18 6.924a7 7 0 0 0 7.788 11.418z" /></g></svg>
   }
}