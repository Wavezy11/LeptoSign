use leptos::*;
use leptos_router::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Subscriber {
    id: Option<i32>,
    email: Option<String>,
    surname: Option<String>,
    lastname: Option<String>,
    address: Option<String>,
    city: Option<String>,
    postal_code: Option<String>,
    phone_number: Option<String>,
}

// ================= DELETE =================
fn delete_subscriber(id: i32, set_subscribers: WriteSignal<Vec<Subscriber>>) {
    spawn_local(async move {
        let url = format!("http://127.0.0.1:3000/delete?id={}", id);
        let _ = Request::delete(&url).send().await;

        if let Ok(resp) = Request::get("http://127.0.0.1:3000/all").send().await {
            if let Ok(data) = resp.json::<Vec<Subscriber>>().await {
                set_subscribers.set(data);
            }
        }
    });
}

// ================= CREATE =================
fn create_subscriber(subscriber: Subscriber, navigate: impl Fn(&str, leptos_router::NavigateOptions) + 'static) {
    spawn_local(async move {
        let _ = Request::post("http://127.0.0.1:3000/subscribe")
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&subscriber).unwrap())
            .unwrap()
            .send()
            .await;

        navigate("/", leptos_router::NavigateOptions::default());
    });
}

// ================= SUBSCRIBER LIST =================
#[component]
fn SubscriberList() -> impl IntoView {
    let (subscribers, set_subscribers) = create_signal(Vec::<Subscriber>::new());

    // Fetch all subscribers on mount
    spawn_local({
        let set_subscribers = set_subscribers.clone();
        async move {
            if let Ok(resp) = Request::get("http://127.0.0.1:3000/all").send().await {
                if let Ok(data) = resp.json::<Vec<Subscriber>>().await {
                    set_subscribers.set(data);
                }
            }
        }
    });
let navigate = use_navigate();
let navigate_for_button = navigate.clone(); // clone voor de Add knop
let navigate_for_rows = navigate.clone();   // clone voor de table rows

view! {
    <div class="container">
        <h2 class="text-2xl font-bold mb-4 animate-fade-in">"All Subscribers"</h2>
        <button 
            class="mb-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors duration-300"
            on:click=move |_| {
                navigate_for_button("/create", leptos_router::NavigateOptions::default());
            }
        >"Add New Subscriber"</button>

        <table class="min-w-full border-collapse">
            <thead>
                <tr class="bg-gray-200">
                    <th>"Email"</th>
                    <th>"First name"</th>
                    <th>"Last name"</th>
                    <th>"Address"</th>
                    <th>"City"</th>
                    <th>"Postal"</th>
                    <th>"Phone"</th>
                    <th>"Edit"</th>
                    <th>"Delete"</th>
                </tr>
            </thead>
            <tbody>
                {move || {
                    subscribers.get().iter().map(|sub| {
                        let sub = sub.clone();
                        let navigate_edit = navigate_for_rows.clone();
                        let set_subscribers = set_subscribers.clone();
                        view! {
                            <tr class="transition-transform duration-300 hover:scale-105 hover:bg-gray-100">
                                <td>{sub.email.clone().unwrap_or_default()}</td>
                                <td>{sub.surname.clone().unwrap_or_default()}</td>
                                <td>{sub.lastname.clone().unwrap_or_default()}</td>
                                <td>{sub.address.clone().unwrap_or_default()}</td>
                                <td>{sub.city.clone().unwrap_or_default()}</td>
                                <td>{sub.postal_code.clone().unwrap_or_default()}</td>
                                <td>{sub.phone_number.clone().unwrap_or_default()}</td>
                                <td>
                                    <button 
                                        class="px-2 py-1 bg-yellow-400 rounded hover:bg-yellow-500 transition-colors duration-300"
                                        on:click=move |_| {
                                            if let Some(id) = sub.id {
                                                navigate_edit(&format!("/edit/{}", id), leptos_router::NavigateOptions::default());
                                            }
                                        }
                                    >"Edit"</button>
                                </td>
                                <td>
                                    <button 
                                        class="px-2 py-1 bg-red-500 text-white rounded hover:bg-red-600 transition-colors duration-300"
                                        on:click=move |_| {
                                            if let Some(id) = sub.id {
                                                delete_subscriber(id, set_subscribers.clone());
                                            }
                                        }
                                    >"Delete"</button>
                                </td>
                            </tr>
                        }
                    }).collect::<Vec<_>>()
                }}
            </tbody>
        </table>
    </div>
}
}   

// ================= EDIT SUBSCRIBER =================
#[component]
fn EditSubscriber() -> impl IntoView {
    let params = use_params_map();
    let id = create_memo(move |_| {
        params.with(|p| {
            p.get("id")
                .and_then(|v| v.parse::<i32>().ok())
                .unwrap_or_default()
        })
    });

    let (email, set_email) = create_signal(String::new());
    let (surname, set_surname) = create_signal(String::new());
    let (lastname, set_lastname) = create_signal(String::new());
    let (address, set_address) = create_signal(String::new());
    let (city, set_city) = create_signal(String::new());
    let (postal_code, set_postal_code) = create_signal(String::new());
    let (phone_number, set_phone_number) = create_signal(String::new());

    let navigate = use_navigate();

    create_effect(move |_| {
        let subscriber_id = id.get();
        spawn_local({
            let set_email = set_email.clone();
            let set_surname = set_surname.clone();
            let set_lastname = set_lastname.clone();
            let set_address = set_address.clone();
            let set_city = set_city.clone();
            let set_postal_code = set_postal_code.clone();
            let set_phone_number = set_phone_number.clone();

            async move {
                if let Ok(resp) = Request::get("http://127.0.0.1:3000/all").send().await {
                    if let Ok(data) = resp.json::<Vec<Subscriber>>().await {
                        if let Some(sub) = data.into_iter().find(|s| s.id == Some(subscriber_id)) {
                            set_email.set(sub.email.unwrap_or_default());
                            set_surname.set(sub.surname.unwrap_or_default());
                            set_lastname.set(sub.lastname.unwrap_or_default());
                            set_address.set(sub.address.unwrap_or_default());
                            set_city.set(sub.city.unwrap_or_default());
                            set_postal_code.set(sub.postal_code.unwrap_or_default());
                            set_phone_number.set(sub.phone_number.unwrap_or_default());
                        }
                    }
                }
            }
        });
    });

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        
        let subscriber_id = id.get();
        let sub = Subscriber {
            id: Some(subscriber_id),
            email: Some(email.get()),
            surname: Some(surname.get()),
            lastname: Some(lastname.get()),
            address: Some(address.get()),
            city: Some(city.get()),
            postal_code: Some(postal_code.get()),
            phone_number: Some(phone_number.get()),
        };

        let navigate = navigate.clone();
        spawn_local(async move {
            let _ = Request::put("http://127.0.0.1:3000/update")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&sub).unwrap())
                .unwrap()
                .send()
                .await;

            navigate("/", leptos_router::NavigateOptions::default());
        });
    };

    view! {
        <div class="container">
            <h2>"Edit Subscriber"</h2>
            <form on:submit=on_submit>
                <input 
                    type="email" 
                    placeholder="Email" 
                    prop:value=email 
                    on:input=move |ev| set_email.set(event_target_value(&ev)) 
                    required=true
                />
                <input 
                    type="text" 
                    placeholder="First name" 
                    prop:value=surname 
                    on:input=move |ev| set_surname.set(event_target_value(&ev)) 
                    required=true
                />
                <input 
                    type="text" 
                    placeholder="Last name" 
                    prop:value=lastname 
                    on:input=move |ev| set_lastname.set(event_target_value(&ev)) 
                    required=true
                />
                <input 
                    type="text" 
                    placeholder="Address" 
                    prop:value=address 
                    on:input=move |ev| set_address.set(event_target_value(&ev)) 
                    required=true
                />
                <input 
                    type="text" 
                    placeholder="City" 
                    prop:value=city 
                    on:input=move |ev| set_city.set(event_target_value(&ev)) 
                    required=true
                />
                        <input 
                    type="text" 
                    placeholder="Postal code"
                    pattern=r"^[1-9][0-9]{3}\s?[A-Z]{2}$"
                    prop:value=postal_code
                    on:input=move |ev| set_postal_code.set(event_target_value(&ev))
                    required=true
                />

                <input 
                    type="tel" 
                    placeholder="Phone number"
                    pattern=r"^\+?[0-9]{8,15}$"
                    prop:value=phone_number
                    on:input=move |ev| set_phone_number.set(event_target_value(&ev))
                    required=true
                />


                <button type="submit">"Update"</button>
            </form>
        </div>
    }
}

// ================= CREATE SUBSCRIBER =================
#[component]
fn CreateSubscriber() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (surname, set_surname) = create_signal(String::new());
    let (lastname, set_lastname) = create_signal(String::new());
    let (address, set_address) = create_signal(String::new());
    let (city, set_city) = create_signal(String::new());
    let (postal_code, set_postal_code) = create_signal(String::new());
    let (phone_number, set_phone_number) = create_signal(String::new());

    let navigate = use_navigate();

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        
        let sub = Subscriber {
            id: None, // No ID for new subscribers
            email: Some(email.get()),
            surname: Some(surname.get()),
            lastname: Some(lastname.get()),
            address: Some(address.get()),
            city: Some(city.get()),
            postal_code: Some(postal_code.get()),
            phone_number: Some(phone_number.get()),
        };

        create_subscriber(sub, navigate.clone());
    };

    view! {
        <div class="container">
            <h2>"Add New Subscriber"</h2>
            <form on:submit=on_submit>
                <input 
                    type="email" 
                    placeholder="Email" 
                    prop:value=email 
                    on:input=move |ev| set_email.set(event_target_value(&ev)) 
                    required=true
                />
                <input 
                    type="text" 
                    placeholder="First name" 
                    prop:value=surname 
                    on:input=move |ev| set_surname.set(event_target_value(&ev)) 
                    required=true
                />
                <input 
                    type="text" 
                    placeholder="Last name" 
                    prop:value=lastname 
                    on:input=move |ev| set_lastname.set(event_target_value(&ev)) 
                    required=true
                />
                <input 
                    type="text" 
                    placeholder="Address" 
                    prop:value=address 
                    on:input=move |ev| set_address.set(event_target_value(&ev)) 
                    required=true
                />
                <input 
                    type="text" 
                    placeholder="City" 
                    prop:value=city 
                    on:input=move |ev| set_city.set(event_target_value(&ev)) 
                    required=true
                />
                <input 
                    type="text" 
                    placeholder="Postal code" 
                    prop:value=postal_code 
                    on:input=move |ev| set_postal_code.set(event_target_value(&ev)) 
                    required=true
                />
                <input 
                    type="text" 
                    placeholder="Phone number" 
                    prop:value=phone_number 
                    on:input=move |ev| set_phone_number.set(event_target_value(&ev)) 
                    required=true
                />
                <button type="submit">"Create Subscriber"</button>
            </form>
        </div>
    }
}

// ================= APP =================
#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=SubscriberList/>
                <Route path="/create" view=CreateSubscriber/>
                <Route path="/edit/:id" view=EditSubscriber/>
            </Routes>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}
