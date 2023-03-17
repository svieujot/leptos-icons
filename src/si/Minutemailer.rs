#[cfg(feature = "SiMinutemailer")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiMinutemailer")]
/// *This icon requires the feature* `SiMinutemailer` *to be enabled*.
#[component]
pub fn Minutemailer(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M17.187 19.181L24 4.755 0 12.386l9.196 1.963.043 4.896 2.759-2.617-2.147-2.076 7.336 4.63z" /></svg>
   }
}