#[cfg(feature = "OcSmFileRemoved")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmFileRemoved")]
/// *This icon requires the feature* `OcSmFileRemoved` *to be enabled*.
#[component]
pub fn FileRemoved(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M2 1.75C2 .784 2.784 0 3.75 0h6.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v9.586A1.75 1.75 0 0 1 13.25 16h-9.5A1.75 1.75 0 0 1 2 14.25Zm1.75-.25a.25.25 0 0 0-.25.25v12.5c0 .138.112.25.25.25h9.5a.25.25 0 0 0 .25-.25V4.664a.25.25 0 0 0-.073-.177l-2.914-2.914a.25.25 0 0 0-.177-.073Zm4.5 6h2.242a.75.75 0 0 1 0 1.5h-2.24l-2.254.015a.75.75 0 0 1-.01-1.5Z" /></svg>
   }
}