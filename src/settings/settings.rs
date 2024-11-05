use crate::i18n::{t, use_i18n, Locale};
use leptos::prelude::*;
use leptos_i18n::t_string;
use thaw::*;

#[component]
pub fn Settings(open: RwSignal<bool>) -> impl IntoView {
    let i18n = use_i18n();
    let langs = vec![("English", "en"), ("简体中文", "zh")];
    let lang_value = RwSignal::new(langs[0].1.to_string());

    view! {
        <Dialog open=open>
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>{t!(i18n, settings)}</DialogTitle>
                    <DialogContent>
                        <form>
                            <FieldContextProvider>
                                <Field label=t_string!(i18n, setting.language).to_string()>
                                    <Select value=lang_value>
                                    {
                                        langs.into_iter()
                                            .map(|lang| view!{<option>{lang.0}</option>})
                                            .collect_view()
                                    }
                                    </Select>
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