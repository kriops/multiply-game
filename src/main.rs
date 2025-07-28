use leptos::mount::mount_to_body;
use leptos::prelude::*;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ConfettiOptions {
    particle_count: u32,
    spread: u32,
    shapes: Vec<String>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "confetti")]
    fn confetti_js(options: &JsValue);
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (a, set_a) = signal(rng());
    let (b, set_b) = signal(rng());
    let (streak, set_streak) = signal(0);
    let (total_streak, set_total_streak) = signal(0);
    let (old_streak_target, set_old_streak_target) = signal(1);
    let (streak_target, set_streak_target) = signal(1);
    let (confetti_count, set_confetti_count) = signal(0);
    let (turn, set_turn) = signal(0);
    let (last_success, set_last_success) = signal(false);

    let run_confetti = || {
        let options = ConfettiOptions {
            particle_count: 360,
            spread: 360,
            shapes: vec!["star".to_string()],
        };
        let js_options = serde_wasm_bindgen::to_value(&options).unwrap();
        confetti_js(&js_options);
    };

    let handle_guess = move |value: i32| {
        if value == a.get() * b.get() {
            // SUCCESS
            *set_streak.write() += 1;
            *set_total_streak.write() += 1;
            *set_last_success.write() = true;
            if streak.get() == streak_target.get() {
                // STREAK TARGET HIT
                let current_streak_target = streak_target.get();
                let old_streak_target = old_streak_target.get();
                let new_streak_target = current_streak_target + old_streak_target;
                *set_old_streak_target.write() = current_streak_target;
                *set_streak_target.write() = new_streak_target;
                *set_confetti_count.write() += 1;

                *set_streak.write() = 0;

                run_confetti()
            }
        } else {
            // FAILURE
            *set_streak.write() = 0;
            *set_total_streak.write() = 0;
            *set_last_success.write() = false;
        }
        *set_turn.write() += 1;
        *set_a.write() = rng();
        *set_b.write() = rng();
    };

    view! {
        <div
            class=move || {
                match turn.get() {
                    0 => "has-background-light",
                    _ => {
                        match last_success.get() {
                            true => "has-background-success",
                            false => "has-background-danger",
                        }
                    }
                }
            }

            style="position: fixed;
            top: 0;
            left: 0;
            width: 100vw;
            height: 100vh;
            z-index: -1;"
        >
            <div class="section">
                <div class="container is-max-tablet">
                    <div class="fixed-grid has-10-cols">
                        <div class="grid box is-gap-0 has-background-white">
                            {ordered_numbers()
                                .into_iter()
                                .map(|inner_vec| {
                                    view! {
                                        {inner_vec
                                            .into_iter()
                                            .map(|value| {
                                                view! {
                                                    <div class="cell" style="aspect-ratio: 1 / 1">
                                                        <button
                                                            on:click=move |_| handle_guess(value)
                                                            class="button is-fullwidth"
                                                            style="height: 100%"
                                                        >
                                                            {value}
                                                        </button>
                                                    </div>
                                                }
                                            })
                                            .collect_view()}
                                    }
                                })
                                .collect_view()}
                        </div>
                    </div>
                    <progress
                        class="progress is-medium"
                        value=streak
                        max= move || streak_target.get() - 1
                        data-text="ASDF"
                    />
                    <div class="level">
                        <div class="level-item has-text-centered">
                            <div>
                                <p class="heading">Streak</p>
                                <p class="title">{total_streak}</p>
                            </div>
                        </div>
                        <div class="level-item has-text-centered">
                            <div>
                                <p class="heading">Regn ut</p>
                                <p class="title is-1">{a}{'×'}{b}</p>
                            </div>
                        </div>
                        <div class="level-item has-text-centered">
                            <div>
                                <p class="heading">Konfetti</p>
                                <p class="title">{confetti_count}</p>
                            </div>
                        </div>
                    </div>
                    <div class="has-text-centered my-12">
                    // <p class="is-size-7">Produsert av <a href="https://kristofferopsahl.com">Kristoffer Opsahl</a>.</p>
        </div>
                </div>

            </div>
        </div>
    }
}

fn multiplication_table() -> Vec<Vec<i32>> {
    let mut values = Vec::new();
    for i in 1..=10 {
        let mut inner_values = Vec::new();
        for j in 1..=10 {
            inner_values.push(i * j);
        }
        values.push(inner_values);
    }
    values.reverse();
    values
}

fn ordered_numbers() -> Vec<Vec<i32>> {
    let mut values = Vec::new();
    for i in 0..10 {
        let mut inner_values = Vec::new();
        for j in 1..=10 {
            inner_values.push(i * 10 + j);
        }
        values.push(inner_values);
    }
    values.reverse();
    values
}

fn rng() -> i32 {
    let mut n_buffer = [0u8; 1];
    let n = match getrandom::fill(&mut n_buffer) {
        Ok(_x) =>  n_buffer[0],
        Err(_) => 0,
    };
    (1 + n % 10) as i32
}