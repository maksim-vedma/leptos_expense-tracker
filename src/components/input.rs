use leptos::*;

#[derive(Debug, Clone, PartialEq)]
pub struct FormControl {
    label: String,
    value: String,
    errors: Vec<String>,
    // validators: Vec<Box<dyn Fn(&str) -> bool + 'static>>,
    valid: bool,
    touched: bool,
}

impl FormControl {
    pub fn new(label: String, value: String) -> Self {
        Self {
            label,
            value,
            ..Self::default()
        }
    }

    pub fn default() -> Self {
        Self {
            label: "".into(),
            value: "".into(),
            errors: vec![],
            valid: true,
            touched: false,
        }
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
        self.touched = true;
        self.validate();
    }

    fn validate(&mut self) {
        self.valid = true;
        self.errors.push("coucou".to_string());
    }
}

#[component]
pub fn Input(// control: Signal<FormControl>
) -> impl IntoView {
    // TODO change to a create_rw_signal() which returns "control" ? to pass only one prop
    // let control_rw = create_rw_signal::<FormControl>(FormControl::new(
    //     "Test label".into(),
    //     "default value".into(),
    // ));

    let (control, set_control) = create_signal::<FormControl>(FormControl::new(
        "Test label".into(),
        "default value".into(),
    ));

    create_effect(move |_| logging::log!("{:#?}", control.get()));

    view! {
        <div class="relative z-0">
            <input
                class="peer block w-full bg-transparent border-b-2 border-neutral-600
                focus:border-teal-400 text-sm outline-none py-2 invalid:border-red-600"
                type="text"
                id="coucou2"
                name="coucou2"
                prop:value=move || control.get().value
                on:input=move |ev| set_control.update(|c| c.set_value(event_target_value(&ev)))
                placeholder=" "
            />
            <label
                class="bottom-0 absolute text-sm text-gray-400 peer-focus:text-teal-400
                duration-200 transform peer-placeholder-shown:scale-100 top-3
                peer-focus:scale-75 -translate-y-6 peer-focus:-translate-y-6 origin-[0]
                scale-75 peer-placeholder-shown:translate-y-0 peer-p transition-all -z-10 truncate w-full
                peer-invalid:text-red-600"
                for="coucou2"
            >
                "Oui je suis beaucoup trop long pour mon propre bien"
            </label>
            <p>{move || control.get().value}</p>
            <p class="text-red-600 text-xs">{move || control.get().errors.join(", ")}</p>
        </div>
    }
}
