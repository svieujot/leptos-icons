#[cfg(feature = "RiDevelopmentLineCursor")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDevelopmentLineCursor")]
/// *This icon requires the feature* `RiDevelopmentLineCursor` *to be enabled*.
#[component]
pub fn Cursor(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path fill-rule="nonzero" d="M15.388 13.498l2.552 7.014-4.698 1.71-2.553-7.014-3.899 2.445L8.41 1.633l11.537 11.232-4.558.633zm-.011 5.818l-2.715-7.46 2.96-.41-5.64-5.49-.79 7.83 2.53-1.587 2.715 7.46.94-.343z" /></g></svg>
   }
}