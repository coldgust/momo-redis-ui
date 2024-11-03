use crate::i18n::{t, use_i18n};
use leptos::prelude::*;
use thaw::*;

#[component]
pub fn Settings(open: RwSignal<bool>) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <Dialog open=open>
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>{t!(i18n, settings)}</DialogTitle>
                    <DialogContent>123</DialogContent>
                    <DialogActions>
                        <Button appearance=ButtonAppearance::Primary>{t!(i18n, confirm)}</Button>
                        <Button on:click=move |_| open.set(false)>{t!(i18n, cancel)}</Button>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    }
}