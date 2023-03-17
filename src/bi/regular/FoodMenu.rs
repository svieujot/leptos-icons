#[cfg(feature = "BiRegularFoodMenu")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularFoodMenu")]
/// *This icon requires the feature* `BiRegularFoodMenu` *to be enabled*.
#[component]
pub fn FoodMenu(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3 2h2v20H3zm7 4h7v2h-7zm0 4h7v2h-7z" /><path d="M19 2H6v20h13c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zm0 18H8V4h11v16z" /></svg>
   }
}