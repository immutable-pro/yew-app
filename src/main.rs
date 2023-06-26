mod api;
mod components;

use crate::api::video::Video;
use crate::components::video_details::VideoDetails;
use crate::components::video_list::VideoList;
use gloo_net::http::Request;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let selected_video = use_state(|| None);
    let videos = use_state(Vec::new);
    {
        let videos = videos.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos: Vec<Video> =
                        Request::get("https://yew.rs/tutorial/data.json")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    videos.set(fetched_videos);
                });
                || ()
            },
            (),
        );
    }

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideoList videos={(*videos).clone()} on_click={on_video_select.clone()} />
            </div>
            { for details }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
