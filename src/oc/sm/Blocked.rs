#[cfg(feature = "OcSmBlocked")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmBlocked")]
/// *This icon requires the feature* `OcSmBlocked` *to be enabled*.
#[component]
pub fn Blocked(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M4.467.22a.749.749 0 0 1 .53-.22h6.006c.199 0 .389.079.53.22l4.247 4.247c.141.14.22.331.22.53v6.006a.749.749 0 0 1-.22.53l-4.247 4.247a.749.749 0 0 1-.53.22H4.997a.749.749 0 0 1-.53-.22L.22 11.533a.749.749 0 0 1-.22-.53V4.997c0-.199.079-.389.22-.53Zm.84 1.28L1.5 5.308v5.384L5.308 14.5h5.384l3.808-3.808V5.308L10.692 1.5ZM4 7.75A.75.75 0 0 1 4.75 7h6.5a.75.75 0 0 1 0 1.5h-6.5A.75.75 0 0 1 4 7.75Z" /></svg>
   }
}