use gloo_events::EventListener;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Event, EventSource, MessageEvent};
use yew::{html, Component, Html};
type AppData = Vec<u32>;
use yew_router::prelude::*;

const APPDATA_URL: &str = "http://localhost:8000/";

type Shared = String;

pub struct App {
    es: EventSource,
    data: Option<Shared>,
    _listener: EventListener,
}

pub enum Msg {
    EsReady(Result<Shared, serde_json::Error>),
    Ignore,
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let es = EventSource::new("http://127.0.0.1:8000/campaigns")
            .map_err(|js_val: JsValue| {
                let err: js_sys::Error = js_val.dyn_into().expect("JS ERROR");
                err
            })
            .expect("ES ERROR");

        let cb = ctx
            .link()
            .callback(|bufstr: String| Msg::EsReady(Ok(bufstr)));

        let listener = EventListener::new(&es, "message", move |event: &Event| {
            let event = event.dyn_ref::<MessageEvent>().expect("EVENT_ERROR");
            let text = event.data().as_string().unwrap();
            cb.emit(text);
        });

        Self {
            es,
            data: None,
            _listener: listener,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div>
                { self.view_ready_state() }
                { self.view_shared() }

            </div>
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::EsReady(response) => {
                match response {
                    Ok(data_result) => {
                        self.data = Some(data_result);
                    }
                    Err(e) => {
                        log::error!("{}", e);
                    }
                };
            }

            Msg::Ignore => {
                return false;
            }
        }
        true
    }
}

impl App {
    fn view_ready_state(&self) -> Html {
        html! {
            <p>{ format!("Connection State: {:?}", self.es.ready_state()) }</p>
        }
    }

    fn view_shared(&self) -> Html {
        if let Some(ref value) = self.data {
            html! {
                <p>{ format!("{:?}", value) }</p>
            }
        } else {
            html! {
                <p>{ "Data hasn't fetched yet." }</p>
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use wasm_bindgen_test::*;
    use yew::{html, FunctionComponent, FunctionProvider, Properties};
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    pub fn obtain_result() -> String {
        gloo_utils::document()
            .get_element_by_id("result")
            .expect("No result found. Most likely, the application crashed and burned")
            .inner_html()
    }

    pub fn obtain_result_by_id(id: &str) -> String {
        gloo_utils::document()
            .get_element_by_id(id)
            .expect("No result found. Most likely, the application crashed and burned")
            .inner_html()
    }

    #[wasm_bindgen_test]
    async fn it_works() {
        struct PropsPassedFunction {}
        #[derive(Properties, Clone, PartialEq)]
        struct PropsPassedFunctionProps {
            value: String,
        }
        impl FunctionProvider for PropsPassedFunction {
            type TProps = PropsPassedFunctionProps;

            fn run(props: &Self::TProps) -> yew::Html {
                assert_eq!(&props.value, "props");
                html! {
                    <div id="result">
                        {"done"}
                    </div>
                }
            }
        }
        type PropsComponent = FunctionComponent<PropsPassedFunction>;

        yew::start_app_with_props_in_element::<PropsComponent>(
            gloo_utils::document().get_element_by_id("output").unwrap(),
            PropsPassedFunctionProps {
                value: "props".to_string(),
            },
        );

        let result = obtain_result();

        assert_eq!(&result, "done");
    }
}
