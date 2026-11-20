//! One-line text fields shared by every page: their contents, which has the keyboard, a
//! suggestion shown selected, and a refused value's message.
use super::line_edit::{self, Key, LineEdit};
use super::*;
use horizon_theme::glass::radius;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(super) enum Field {
    /// The instance dialog's (`instances_page.rs`).
    InstanceName,
    InstanceGroup,
    /// Its search in Mojang's versions (`new_instance.rs`).
    InstanceVersion,
    /// Home's Mods tab's filter (`instance_mods.rs`).
    InstanceModsFilter,
    /// Home's Multiplayer section's (`multiplayer_page.rs`).
    ServerName,
    ServerAddress,
    /// Home's Console's search (`console_page.rs`).
    ConsoleSearch,
    /// The Launch panel's (`launch_page.rs`).
    PreLaunch,
    Wrapper,
    LaunchEnv,
    PostExit,
    /// Home's memory row (`launch_page.rs`).
    MemoryLimit,
    /// Settings' accent as hex (`themes_page.rs`).
    AccentHex,
    /// A Java instance's own JVM arguments and window size (the instance sheet's Launch).
    JavaArgs,
    WindowWidth,
    WindowHeight,
}

#[derive(Default)]
pub(super) struct Fields {
    texts: HashMap<Field, LineEdit>,
    focus: HashMap<Field, FocusHandle>,
    focused: Option<Field>,
    /// The field to put the keyboard in on the next frame.
    pub(super) focus_soon: Option<Field>,
    /// A suggested value shown selected: the first key typed replaces it.
    selected: Option<Field>,
    /// A field whose value was refused, and why: shown under it until it's edited.
    invalid: Option<(Field, &'static str)>,
}

impl Fields {
    pub(super) fn text(&mut self, f: Field) -> &mut LineEdit {
        self.texts.entry(f).or_default()
    }

    pub(super) fn value(&self, f: Field) -> &str {
        self.texts.get(&f).map_or("", LineEdit::text)
    }

    /// Mark a field invalid without an explanatory message, or clear its previous error.
    pub(super) fn set_field_invalid(&mut self, field: Field, invalid: bool) {
        if invalid {
            self.invalid = Some((field, ""));
        } else if self.invalid.is_some_and(|(current, _)| current == field) {
            self.invalid = None;
        }
    }

    /// Whether this field has the keyboard, as of this frame.
    pub(super) fn has_focus(&self, f: Field) -> bool {
        self.focused == Some(f)
    }

    /// Which field has the keyboard, read every frame.
    pub(super) fn note_focus(&mut self, window: &mut Window, cx: &mut App) {
        if let Some(h) = self.focus_soon.take().and_then(|f| self.focus.get(&f)) {
            window.focus(h, cx);
        }
        self.focused = self
            .focus
            .iter()
            .find(|(_, h)| h.is_focused(window))
            .map(|(f, _)| *f);
        if self.focused.is_some() && self.focused != self.selected {
            self.selected = None;
        }
    }
}

impl Launcher {
    fn focus_handle(&mut self, f: Field, cx: &mut Context<Self>) -> FocusHandle {
        self.fields
            .focus
            .entry(f)
            .or_insert_with(|| cx.focus_handle().tab_index(0).tab_stop(true))
            .clone()
    }

    /// A one-line text field: typing, paste, Backspace, Enter to confirm, Escape, Tab to the next.
    pub(super) fn field(
        &mut self,
        f: Field,
        placeholder: &'static str,
        width: f32,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let t = self.theme;
        let handle = self.focus_handle(f, cx);
        let focused = self.fields.focused == Some(f);
        let selected = focused && self.fields.selected == Some(f);
        let invalid = self.fields.invalid.is_some_and(|(g, _)| g == f);
        self.fields.text(f).show(((width - 24.) / 7.5) as usize);
        // Home's Add server line is in the glass look already; the other fields follow with
        // their pages.
        // The dark appearance draws every field in the glass look.
        let glass = t.glass;
        let base = if glass {
            // The mods filter and the log search sit by buttons: the mockup's small `FIELD`, as
            // tall as they are.
            crate::glass_ui::field(focused, invalid)
                .when(
                    matches!(f, Field::InstanceModsFilter | Field::ConsoleSearch),
                    |d| d.h(px(36.)).px(px(12.)).rounded(px(radius::CONTROL)),
                )
                // The instance dialog's are the large `FIELD_LG`: 48 px, 15 px text, white at 6 %,
                // 10 % while typed in.
                .when(
                    !invalid
                        && matches!(
                            f,
                            Field::InstanceName | Field::InstanceGroup | Field::InstanceVersion
                        ),
                    |d| {
                        d.h(px(48.))
                            .text_size(px(15.))
                            .bg(crate::glass_ui::white(if focused { 0.1 } else { 0.06 }))
                    },
                )
                // The accent's hex: `h-9 w-[96px] rounded-[10px] px-3 text-[13px] font-medium`.
                .when(f == Field::AccentHex, |d| {
                    d.h(px(36.))
                        .px(px(12.))
                        .rounded(px(radius::CONTROL))
                        .text_size(px(13.))
                        .font_weight(FontWeight::MEDIUM)
                })
                // The mockup's focus ring (`:focus-visible`, 2 px of the accent's soft shade) on a
                // field of its own, drawn inside its edge: GPUI has no outline.
                .when(focused && !invalid, |d| {
                    let small = matches!(
                        f,
                        Field::InstanceModsFilter | Field::ConsoleSearch | Field::AccentHex
                    );
                    d.child(crate::glass_ui::focus_ring(if small {
                        radius::CONTROL
                    } else {
                        radius::ROW
                    }))
                })
        } else {
            div()
                .flex()
                .items_center()
                .h(px(theme::FIELD))
                .px(px(12.))
                .overflow_hidden()
                .rounded(px(theme::RADIUS))
                .border_1()
                .border_color(if invalid {
                    t.danger
                } else if focused {
                    t.accent
                } else {
                    t.rule
                })
                .bg(t.surface)
                .text_size(px(theme::UI.px))
        };
        let field = base
            .id(SharedString::from(format!("field-{f:?}")))
            .track_focus(&handle)
            .w(px(width))
            .cursor_text()
            .child(line_edit::content(
                self.fields.text(f),
                &t,
                focused,
                selected,
                false,
                placeholder,
            ))
            .on_click(cx.listener(move |this, ev: &ClickEvent, window, cx| {
                // A click puts the caret in: the suggestion is no longer selected.
                if matches!(ev, ClickEvent::Mouse(_)) {
                    this.fields.selected = None;
                }
                if let Some(h) = this.fields.focus.get(&f) {
                    window.focus(h, cx);
                }
                cx.notify();
            }))
            .on_key_down(cx.listener(move |this, ev: &KeyDownEvent, window, cx| {
                if ev.keystroke.key != "enter" && this.fields.invalid.is_some_and(|(g, _)| g == f) {
                    this.fields.invalid = None;
                }
                // A suggestion is selected: what's typed or pasted replaces it.
                let selected = this.fields.selected.take() == Some(f);
                if selected {
                    this.fields.text(f).select_all();
                }
                match line_edit::handle(this.fields.text(f), ev, cx) {
                    Key::Enter if matches!(f, Field::InstanceName | Field::InstanceGroup) => {
                        this.save_instance(cx)
                    }
                    Key::Enter => this.field_enter(f),
                    Key::Escape => {
                        this.instance_escape();
                        window.blur(cx);
                    }
                    // Tab and the rest go on to the window: Tab moves the focus.
                    Key::Pass => return,
                    _ => {}
                }
                cx.stop_propagation();
                cx.notify();
            }));
        line_edit::mouse(field, cx, move |this| this.fields.text(f))
    }

    fn field_enter(&mut self, f: Field) {
        match f {
            Field::InstanceName
            | Field::InstanceGroup
            | Field::InstanceVersion
            | Field::InstanceModsFilter
            | Field::ConsoleSearch
            | Field::JavaArgs
            | Field::WindowWidth
            | Field::WindowHeight
            | Field::AccentHex => {}
            Field::PreLaunch | Field::Wrapper | Field::LaunchEnv | Field::PostExit => {
                self.save_launch()
            }
            Field::MemoryLimit => self.memory_enter(),
            Field::ServerName | Field::ServerAddress => self.add_server(),
        }
    }
}
