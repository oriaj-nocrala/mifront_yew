use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <div class="footer">
                // <div class="footer__logo">
                //     <img src="https://yew.rs/img/logo.png" alt="Yew logo" />
                // </div>
                // <div class="footer__text">
                //     <p>{ "Yew is an open-source Rust framework for creating client web apps with WebAssembly." }</p>
                // </div>
                <div class="footer__links">
                    <a href="http://ejemplo.com">{"Ejemplo"}</a>
                    <a href="http://ejemplo.com">{"Ejemplo"}</a>
                    <a href="http://ejemplo.com">{"Ejemplo"}</a>
                </div>
            </div>
        </footer>
    }
}