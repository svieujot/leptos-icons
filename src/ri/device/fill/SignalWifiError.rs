#[cfg(feature = "RiDeviceFillSignalWifiError")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDeviceFillSignalWifiError")]
/// *This icon requires the feature* `RiDeviceFillSignalWifiError` *to be enabled*.
#[component]
pub fn SignalWifiError(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M12 3c4.284 0 8.22 1.497 11.31 3.996L22.498 8H18v5.571L12 21 .69 6.997C3.78 4.497 7.714 3 12 3zm10 16v2h-2v-2h2zm0-9v7h-2v-7h2z" /></g></svg>
   }
}