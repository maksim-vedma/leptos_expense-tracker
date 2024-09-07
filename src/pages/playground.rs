use crate::components::dialog::Dialog;
use crate::components::*;
use leptos::*;

#[component]
pub fn Playground() -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal::<bool>(false);

    view! {
        <div>
            <p>"Je suis la HOME"</p>
            <input::Input></input::Input>
            <p>"Je suis la HOMEEND"</p>
            <simple_button::SimpleButton on:click=move |_| {
                set_show_modal.set(true);
            }>"Ouvrir la Modal"</simple_button::SimpleButton>
            <simple_button::SimpleButton>"Simple bouton juste pour voir"</simple_button::SimpleButton>

            <Details summary=String::from("Test détails")>
                <p>"Je suis le contenu du details"</p>
                <p>
                    Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.
                </p>
            </Details>
        <Dialog show=show_modal set_show=set_show_modal>cocuocuocu</Dialog>
        </div>
    }
}
