#[cfg(feature = "VsDeviceCameraVideo")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDeviceCameraVideo")]
/// *This icon requires the feature* `VsDeviceCameraVideo` *to be enabled*.
#[component]
pub fn DeviceCameraVideo(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M14.25 4.74L11 6.62V4.5l-.5-.5h-9l-.5.5v7l.5.5h9l.5-.5v-2l3.25 1.87.75-.47V5.18l-.75-.44zM10 11H2V5h8v6zm4-1l-3-1.7v-.52L14 6v4z" /></svg>
   }
}