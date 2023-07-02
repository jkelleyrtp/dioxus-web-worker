use std::ops::Deref;

use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{
    DedicatedWorkerGlobalScope, MessageEvent, Worker, WorkerGlobalScope, WorkerOptions, WorkerType,
};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let (worker, msgs) = use_webworker(cx);
    let message = use_state(cx, || "".to_string());

    render! {
        div {
            button {
                onclick: move |_| {
                    let msg = format!("Message from main: {}", message.get());
                    worker.post_message(&JsValue::from_str(&msg));
                },
                "Send a message to the worker"
            }
            input {
                value: "{message}",
                oninput: move |event| {
                    message.set(event.value.clone());
                }
            }
        }
        div {
            msgs.read().iter().map(|msg| render! {
                div {
                    "{msg}"
                }
            })
        }
    }
}

pub fn use_webworker(cx: &ScopeState) -> (&mut Worker, &UseRef<Vec<String>>) {
    let messages = use_ref(cx, || vec![]);

    let worker = cx.use_hook(|| {
        let worker = Worker::new_with_options("worker.js", &worker_options()).unwrap();

        let messages = messages.clone();
        let f: Closure<dyn Fn(MessageEvent) -> ()> = Closure::new(move |event: MessageEvent| {
            log::info!("Message from worker: {:?}", event.data());
            messages
                .write()
                .push(format!("Message from worker: {:?}", event.data()));
        });

        let val = f.into_js_value();
        let f = js_sys::Function::unchecked_from_js(val);
        worker.set_onmessage(Some(&f));

        worker
    });

    (worker, messages)
}

fn worker_options() -> WorkerOptions {
    let mut options = WorkerOptions::new();
    options.type_(WorkerType::Module);
    options
}

// todo: on the dioxus side of things, we can make this a macro or something that writes the JS snippet automatically to
// link it all together
#[wasm_bindgen]
pub fn start_webworker() {
    log::info!("Starting webworker");

    let self_ = js_sys::global();
    let js_value = self_.deref();
    let scope = DedicatedWorkerGlobalScope::unchecked_from_js_ref(js_value);
    // let scope = WorkerGlobalScope::unchecked_from_js_ref(js_value);

    let _scope = scope.clone();

    let f: Closure<dyn Fn(MessageEvent) -> ()> = Closure::new(move |event: MessageEvent| {
        log::info!("Message from maint thread: {:?}", event.data());
        let msg = format!("Message from main: {:?}", event.data());

        _scope.post_message(&JsValue::from_str(&msg));
    });

    let val = f.into_js_value();
    let f = js_sys::Function::unchecked_from_js(val);

    scope.set_onmessage(Some(&f))

    // let scope = WorkerGlobalScope.self();

    // loop and wait for messages from the main thread, then send a message back
}
