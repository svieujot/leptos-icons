#[cfg(feature = "SiAdguard")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiAdguard")]
/// *This icon requires the feature* `SiAdguard` *to be enabled*.
#[component]
pub fn Adguard(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12 0C8.249 0 3.725.861 0 2.755 0 6.845-.051 17.037 12 24 24.051 17.037 24 6.845 24 2.755 20.275.861 15.751 0 12 0zm-.106 15.429L6.857 9.612c.331-.239 1.75-1.143 2.794.042l2.187 2.588c.009-.001 5.801-5.948 5.815-5.938.246-.22.694-.503 1.204-.101l-6.963 9.226z" /></svg>
   }
}