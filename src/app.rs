// Importaciones necesarias
//use leptos::leptos_dom::ev::{SubmitEvent};
use leptos::*;
use serde::{Deserialize, Serialize};
//use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

// Declaración de una función externa para invocar comandos desde JavaScript/TypeScript
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// Definición de una estructura para los argumentos de saludo
#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

// Definición del componente de la aplicación
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <html>
            <head>
                <title>"ArmCord"</title>
                <meta http-equiv="refresh" content="0; url=https://discord.com/app" />
            </head>
            <body>
                <h1>"Hello World!"</h1>
                <h1>"Redirecting you to Discord..."</h1>
                <h1>"If you are stuck on this page check your internet connection!"</h1>

                <script src="./renderer.js"></script>
            </body>
        </html>
    }
}