#[cfg(feature = "BiSolidBuildingHouse")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBuildingHouse")]
/// *This icon requires the feature* `BiSolidBuildingHouse` *to be enabled*.
#[component]
pub fn BuildingHouse(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18.991 2H9.01C7.899 2 7 2.899 7 4.01v5.637l-4.702 4.642A1 1 0 0 0 3 16v5a1 1 0 0 0 1 1h16a1 1 0 0 0 1-1V4.009C21 2.899 20.102 2 18.991 2zm-8.069 13.111V20H5v-5.568l2.987-2.949 2.935 3.003v.625zM13 9h-2V7h2v2zm4 8h-2v-2h2v2zm0-4h-2v-2h2v2zm0-4h-2V7h2v2z" /><path d="M7 15h2v2H7z" /></svg>
   }
}