//! The overlay window's content: the web app's `overlay/page.rs`, `panel.rs` and `model.rs`.
//!
//! Five widgets float over the game: the damage against the selected enemy, the items that add
//! the most damage against it, and the three parts of the stack. While the overlay is open
//! (the shortcut, Ctrl+K by default) widgets can be dragged by their title and resized from
//! their corner, and a settings card chooses which widgets show and which stay visible when the
//! overlay is closed. Ctrl+O selects the next enemy. Layout and choices are saved in
//! `%APPDATA%\tutorlolv2\overlay.bin`.
//!
//! The web scales widgets with a CSS transform; gpui has none, so widgets are resized instead
//! and their content scrolls.
use crate::{
    calculator::widgets::checkbox,
    components::{
        image::{Entry, image},
        stack::{StackGame, StackPanel, StackPart},
        tables::{Cell, Column, damage_columns, table},
        ui::{self, Tone},
    },
    livegame::{self, game::Game},
    storage, theme,
};
use bincode::{Decode, Encode};
use gpui::{
    AnyElement, AppContext, Context, Entity, FontWeight, IntoElement, MouseButton, MouseMoveEvent,
    MouseUpEvent, ParentElement, Pixels, Point, Render, SharedString, Styled, Task, Window, div,
    point, prelude::*, px, rgb, rgba,
};
use std::rc::Rc;
use strum::VariantArray;

#[derive(Clone, Copy, Debug, PartialEq, Eq, VariantArray)]
pub enum Widget {
    Damage,
    Recommendations,
    StackInsert,
    StackRemover,
    Stacks,
}

impl Widget {
    const fn index(self) -> usize {
        self as usize
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Damage => "Damage table",
            Self::Recommendations => "Recommendations",
            Self::StackInsert => "Add stacks",
            Self::StackRemover => "Remove stacks",
            Self::Stacks => "Stack table",
        }
    }
}

/// Saved widget choices and placement (`Panel` and `PanelState` on the web).
#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct Layout {
    pub enabled: [bool; 5],
    pub keep_visible: [bool; 5],
    /// `(left, top, width, height)` of each widget, in pixels.
    pub frames: [(f32, f32, f32, f32); 5],
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            enabled: [true; 5],
            keep_visible: [true, false, false, false, true],
            // Fits a 1280x720 screen; widgets are kept on screen when it is smaller.
            frames: [
                (24., 360., 620., 150.),
                (24., 530., 380., 170.),
                (860., 24., 400., 300.),
                (860., 340., 400., 200.),
                (420., 530., 420., 170.),
            ],
        }
    }
}

impl Layout {
    pub const fn visible(&self, widget: Widget, open: bool) -> bool {
        self.enabled[widget.index()] && (open || self.keep_visible[widget.index()])
    }
}

const LAYOUT_FILE: &str = "overlay.bin";
const LAYOUT_KIND: u8 = 2;
const MIN_SIZE: f32 = 120.;

#[derive(Clone, Copy)]
enum Drag {
    Move(Widget, Point<Pixels>),
    Resize(Widget, Point<Pixels>, (f32, f32)),
}

pub struct OverlayView {
    open: bool,
    game: Option<Rc<Game>>,
    message: Option<SharedString>,
    enemy: usize,
    stack: Entity<StackPanel>,
    layout: Layout,
    save_error: bool,
    drag: Option<Drag>,
    _poll: Task<()>,
}

impl OverlayView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let poll = livegame::poll(cx, |this: &mut Self, game, message, cx| {
            if let Some(game) = &game {
                this.enemy = this.enemy.min(game.enemies.len().saturating_sub(1));
                this.stack.update(cx, |stack, cx| {
                    stack.set_game(Some(game.clone() as Rc<dyn StackGame>), cx)
                });
            }

            this.game = game;
            this.message = message;
            cx.notify();
        });

        Self {
            open: false,
            game: None,
            message: None,
            enemy: 0,
            stack: cx.new(|_| StackPanel::new(false)),
            layout: storage::read_record(LAYOUT_FILE, LAYOUT_KIND).unwrap_or_default(),
            save_error: false,
            drag: None,
            _poll: poll,
        }
    }

    pub fn set_open(&mut self, open: bool, cx: &mut Context<Self>) {
        self.open = open;
        self.drag = None;
        cx.notify();
    }

    /// Ctrl+O: the next enemy.
    pub fn next_enemy(&mut self, cx: &mut Context<Self>) {
        if let Some(game) = &self.game
            && !game.enemies.is_empty()
        {
            self.enemy = (self.enemy + 1) % game.enemies.len();
            cx.notify();
        }
    }

    fn save(&mut self, cx: &mut Context<Self>) {
        self.save_error = storage::write_record(LAYOUT_FILE, LAYOUT_KIND, &self.layout).is_err();
        cx.notify();
    }

    fn drag_to(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        let Some(drag) = self.drag else { return };

        match drag {
            Drag::Move(widget, offset) => {
                let frame = &mut self.layout.frames[widget.index()];
                frame.0 = f32::from(position.x - offset.x).max(0.);
                frame.1 = f32::from(position.y - offset.y).max(0.);
            }
            Drag::Resize(widget, start, (width, height)) => {
                let frame = &mut self.layout.frames[widget.index()];
                frame.2 = (width + f32::from(position.x - start.x)).max(MIN_SIZE);
                frame.3 = (height + f32::from(position.y - start.y)).max(MIN_SIZE);
            }
        }

        cx.notify();
    }

    fn content(&self, widget: Widget, cx: &mut Context<Self>) -> Option<AnyElement> {
        let game = self.game.clone()?;
        let enemy = self.enemy.min(game.enemies.len().checked_sub(1)?);

        Some(match widget {
            Widget::Damage => {
                let champion = game.enemies[enemy].champion_id;
                let mut columns = damage_columns(
                    &[game.enemy_damages(enemy)],
                    game.current_player.champion_id,
                    game.current_player.adaptive_type,
                    &game.items_meta,
                    &game.runes_meta,
                );

                columns.insert(
                    0,
                    Column {
                        header: Cell::empty(),
                        cells: vec![Cell::new(image(champion.image_source()).size(px(32.)))],
                    },
                );

                table("overlay-damage", columns).into_any_element()
            }
            Widget::Recommendations => {
                let base = game.total_damage(enemy);
                let rows = game
                    .item_scores(enemy)
                    .into_iter()
                    .take(8)
                    .map(|(damage, item)| {
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(image(item.image_source()).size_6())
                            .child(div().flex_1().truncate().text_sm().child(item.name()))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(theme::SKY_300))
                                    .child(format!("+{}", damage - base)),
                            )
                    });

                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .p_2()
                    .children(rows)
                    .into_any_element()
            }
            Widget::StackInsert | Widget::StackRemover | Widget::Stacks => {
                let part = match widget {
                    Widget::StackInsert => StackPart::Insert,
                    Widget::StackRemover => StackPart::Remover,
                    _ => StackPart::Totals,
                };

                self.stack.update(cx, |stack, cx| stack.part(part, cx))?
            }
        })
    }

    fn widget(
        &self,
        widget: Widget,
        viewport: gpui::Size<Pixels>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let (left, top, width, height) = self.layout.frames[widget.index()];
        // Saved on a larger screen: keep at least the title on this one.
        let left = left.min(f32::from(viewport.width) - 80.).max(0.);
        let top = top.min(f32::from(viewport.height) - 40.).max(0.);
        let open = self.open;

        let title = div()
            .id(("overlay-title", widget.index()))
            .flex()
            .items_center()
            .justify_between()
            .px_2()
            .py_1()
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(rgb(theme::STD_300))
            .bg(rgb(theme::STD_800))
            .cursor_move()
            .child(widget.label())
            .child(
                match self.game.as_ref().and_then(|g| g.enemies.get(self.enemy)) {
                    Some(enemy)
                        if widget == Widget::Damage || widget == Widget::Recommendations =>
                    {
                        format!("vs {} · Ctrl+O", enemy.champion_id.name())
                    }
                    _ => String::new(),
                },
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, event: &gpui::MouseDownEvent, _, cx| {
                    let (left, top, _, _) = this.layout.frames[widget.index()];
                    let offset = event.position - point(px(left), px(top));

                    this.drag = Some(Drag::Move(widget, offset));
                    cx.stop_propagation();
                }),
            );

        let handle = div()
            .id(("overlay-resize", widget.index()))
            .absolute()
            .right_0()
            .bottom_0()
            .size_3()
            .bg(rgb(theme::SKY_300))
            .cursor_nwse_resize()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, event: &gpui::MouseDownEvent, _, cx| {
                    let (_, _, width, height) = this.layout.frames[widget.index()];

                    this.drag = Some(Drag::Resize(widget, event.position, (width, height)));
                    cx.stop_propagation();
                }),
            );

        let content = self.content(widget, cx).unwrap_or_else(|| {
            div()
                .p_2()
                .text_sm()
                .text_color(rgb(theme::STD_400))
                .child(
                    self.message
                        .clone()
                        .unwrap_or("Waiting for a game...".into()),
                )
                .into_any_element()
        });

        div()
            .absolute()
            .left(px(left))
            .top(px(top))
            .w(px(width))
            .h(px(height))
            .flex()
            .flex_col()
            .rounded_md()
            .overflow_hidden()
            .border_1()
            .border_color(rgb(if open { theme::SKY_300 } else { theme::STD_800 }))
            .bg(rgba(0x18181be6))
            .when(open, |frame| frame.child(title))
            .child(
                div()
                    .id(("overlay-content", widget.index()))
                    .flex_1()
                    .min_h_0()
                    .overflow_scroll()
                    .child(content),
            )
            .when(open, |frame| frame.child(handle))
            .into_any_element()
    }

    /// The settings card shown while the overlay is open.
    fn settings(&self, cx: &mut Context<Self>) -> AnyElement {
        let rows = Widget::VARIANTS.iter().map(|&widget| {
            let index = widget.index();

            div()
                .flex()
                .items_center()
                .gap_4()
                .child(div().w(px(150.)).text_sm().child(widget.label()))
                .child(checkbox(
                    ("overlay-show", index),
                    "Show",
                    self.layout.enabled[index],
                    cx.listener(move |this, value: &bool, _, cx| {
                        this.layout.enabled[index] = *value;
                        this.save(cx);
                    }),
                ))
                .child(checkbox(
                    ("overlay-keep", index),
                    "Keep visible when closed",
                    self.layout.keep_visible[index],
                    cx.listener(move |this, value: &bool, _, cx| {
                        this.layout.keep_visible[index] = *value;
                        this.save(cx);
                    }),
                ))
        });

        div()
            .absolute()
            .top(px(24.))
            .left(px(24.))
            .w(px(560.))
            .occlude()
            .child(
                ui::card()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(ui::heading("Overlay widgets"))
                            .child(ui::button(
                                "overlay-done",
                                "Done",
                                Tone::Primary,
                                false,
                                false,
                                |_, _, cx| super::set_open(false, cx),
                            )),
                    )
                    .child(ui::muted(
                        "Drag a widget by its title to move it and from its corner to resize \
                         it. Ctrl+O shows the next enemy. Escape closes the overlay.",
                    ))
                    .children(rows)
                    .child(
                        ui::actions()
                            .child(ui::button(
                                "overlay-show-all",
                                "Show all",
                                Tone::Secondary,
                                false,
                                false,
                                cx.listener(|this, _, _, cx| {
                                    this.layout.enabled = [true; 5];
                                    this.save(cx);
                                }),
                            ))
                            .child(ui::button(
                                "overlay-hide-all",
                                "Hide all",
                                Tone::Secondary,
                                false,
                                false,
                                cx.listener(|this, _, _, cx| {
                                    this.layout.enabled = [false; 5];
                                    this.save(cx);
                                }),
                            ))
                            .child(ui::button(
                                "overlay-reset",
                                "Reset layout",
                                Tone::Secondary,
                                false,
                                false,
                                cx.listener(|this, _, _, cx| {
                                    this.layout.frames = Layout::default().frames;
                                    this.save(cx);
                                }),
                            )),
                    )
                    .when(self.save_error, |card| {
                        card.child(ui::status("Widget preferences could not be saved.", true))
                    }),
            )
            .into_any_element()
    }
}

impl Render for OverlayView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let open = self.open;
        let viewport = window.viewport_size();
        let widgets = Widget::VARIANTS
            .iter()
            // Closed and without a game, the overlay shows nothing over the desktop.
            .filter(|_| open || self.game.is_some())
            .filter(|&&widget| self.layout.visible(widget, open))
            .map(|&widget| self.widget(widget, viewport, cx))
            .collect::<Vec<_>>();

        div()
            .id("overlay")
            .relative()
            .size_full()
            .text_color(rgb(theme::TEXT))
            // A light veil shows that the overlay takes the mouse.
            .when(open, |root| root.bg(rgba(0x00000040)))
            .on_mouse_move(
                cx.listener(|this, event: &MouseMoveEvent, _, cx| this.drag_to(event.position, cx)),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _: &MouseUpEvent, _, cx| {
                    if this.drag.take().is_some() {
                        this.save(cx);
                    }
                }),
            )
            .children(widgets)
            .when(open, |root| root.child(self.settings(cx)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recommendations_and_editors_disappear_when_closed_by_default() {
        let layout = Layout::default();
        assert!(layout.visible(Widget::Recommendations, true));
        assert!(!layout.visible(Widget::Recommendations, false));
        assert!(!layout.visible(Widget::StackInsert, false));
        assert!(!layout.visible(Widget::StackRemover, false));
        assert!(layout.visible(Widget::Damage, false));
        assert!(layout.visible(Widget::Stacks, false));
    }

    #[test]
    fn layout_survives_binary_roundtrip() {
        let mut layout = Layout {
            enabled: [true, false, true, false, true],
            keep_visible: [false, true, false, true, false],
            ..Default::default()
        };
        layout.frames[2] = (1., 2., 300., 400.);
        let config = bincode::config::standard();
        let bytes = bincode::encode_to_vec(&layout, config).unwrap();
        let (loaded, read): (Layout, _) = bincode::decode_from_slice(&bytes, config).unwrap();
        assert_eq!(loaded, layout);
        assert_eq!(read, bytes.len());
    }
}
