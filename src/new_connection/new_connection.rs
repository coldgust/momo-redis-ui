use leptos::attr::required;
use crate::i18n::{t, use_i18n};
use leptos::prelude::*;
use leptos_i18n::__private::macros_reexport::t_string;
use thaw::*;

#[component]
pub fn NewConnection(open: RwSignal<bool>) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <Dialog open=open>
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>{t!(i18n, new_connection)}</DialogTitle>
                    <DialogContent>
                        <form>
                            <FieldContextProvider>
                                <Field label=t_string!(i18n, host) required=true>
                                    <Input rules=vec![InputRule::required(true.into())]/>
                                </Field>
                                <Field label=t_string!(i18n, port) required=true>
                                    <SpinButton<u32> step_page=1 max=65535/>
                                </Field>
                            </FieldContextProvider>
                        </form>
                    </DialogContent>
                    <DialogActions>
                        <Button appearance=ButtonAppearance::Primary>{t!(i18n, confirm)}</Button>
                        <Button on:click=move |_| open.set(false)>{t!(i18n, cancel)}</Button>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    }
}