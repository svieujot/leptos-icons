#[cfg(feature = "FaBrandsUnsplash")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaBrandsUnsplash")]
/// *This icon requires the feature* `FaBrandsUnsplash` *to be enabled*.
#[component]
pub fn Unsplash(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M448,230.17V480H0V230.17H141.13V355.09H306.87V230.17ZM306.87,32H141.13V156.91H306.87Z" /></svg>
   }
}