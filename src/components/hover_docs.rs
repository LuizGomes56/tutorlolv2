//! Documentation tooltips: the web app's `hover.rs`, `hover.js` and `#hoverdocs` element.
//!
//! gpui's built-in `hoverable_tooltip` waits 500 ms before showing and before hiding, and places
//! the tooltip at the cursor position from when that delay ended. Neither can be configured, so
//! this module replaces it with `hover.js`'s behavior:
//!
//! - Show immediately when an element is hovered while Shift is held, or when Shift is pressed
//!   while hovering one.
//! - Hide as soon as neither the element nor the tooltip is hovered, on Escape, when the window
//!   loses focus, or when the pointer leaves the window.
//! - Place the tooltip at the element's bottom-left corner. Align it to the element's right edge
//!   or put it above the element when it would overflow the window, keeping an 8px margin.
//!
//! Two elements cooperate through the [`HoverDocsState`] global:
//!
//! - [`HoverDocsExt::hover_docs`] adds an [`Anchor`] to an element. The anchor covers its parent,
//!   tracks whether the parent is hovered and, while it is the active one, records its bounds.
//! - [`layer`], rendered once by the shell, draws the active tooltip above everything else.
//!   It is drawn after every other deferred element (such as the selector dropdown, whose options
//!   have docs too), so the bounds recorded by anchors in this frame are already known.
use crate::{
    components::code::{Highlighted, code_text},
    theme,
};
use gpui::{
    App, AppContext, AvailableSpace, Bounds, Context, Element, ElementId, Entity, Global,
    GlobalElementId, Hitbox, HitboxBehavior, InspectorElementId, IntoElement, LayoutId, Length,
    ModifiersChangedEvent, MouseExitEvent, MouseMoveEvent, ParentElement, Pixels, Position, Render,
    Stateful, Style, Styled, Window, deferred, div, point, prelude::*, px, rems, rgb,
};
use std::rc::Rc;
use tutorlolv2::CastId;
use tutorlolv2_codec::Spans;

/// Distance kept from the window edges, as in `hover.js`.
const WINDOW_MARGIN: Pixels = px(8.);
/// Above the selector dropdown (priority 1).
const LAYER_PRIORITY: usize = 2;

/// Builds the tooltip's blocks, shown top to bottom. Called when the tooltip opens.
type BuildDocs = Rc<dyn Fn() -> Vec<Highlighted>>;
/// The anchor's [`GlobalElementId`], which is unique within the window.
type AnchorId = Rc<[ElementId]>;

#[derive(Default)]
struct HoverDocsState {
    /// The anchor under the pointer, kept so pressing Shift can show its docs.
    hovered: Option<(AnchorId, BuildDocs)>,
    active: Option<Active>,
}

impl Global for HoverDocsState {}

struct Active {
    anchor: AnchorId,
    view: Entity<HoverDocs>,
    tooltip_hovered: bool,
    /// The anchor's bounds in the current frame. `None` when it was not drawn (its element
    /// was removed, like a dropdown option after closing), which hides the tooltip.
    anchor_bounds: Option<Bounds<Pixels>>,
}

fn state(cx: &mut App) -> &mut HoverDocsState {
    cx.default_global::<HoverDocsState>()
}

fn show(anchor: AnchorId, build: &BuildDocs, window: &mut Window, cx: &mut App) {
    if state(cx)
        .active
        .as_ref()
        .is_some_and(|active| active.anchor == anchor)
    {
        return;
    }

    let docs = build();
    let view = cx.new(|_| HoverDocs(docs));

    state(cx).active = Some(Active {
        anchor,
        view,
        tooltip_hovered: false,
        anchor_bounds: None,
    });

    window.refresh();
}

/// Hides the tooltip, if any.
pub fn hide(window: &mut Window, cx: &mut App) {
    if state(cx).active.take().is_some() {
        window.refresh();
    }
}

/// Hides the tooltip once neither its element nor the tooltip itself is hovered. Runs after the
/// current event so that leaving the element and entering the tooltip (two listeners of the same
/// mouse move) are both seen first.
fn hide_when_unhovered(window: &mut Window, cx: &mut App) {
    window.defer(cx, |window, cx| {
        let state = state(cx);
        let Some(active) = &state.active else {
            return;
        };

        let anchor_hovered = state
            .hovered
            .as_ref()
            .is_some_and(|(anchor, _)| *anchor == active.anchor);

        if !anchor_hovered && !active.tooltip_hovered {
            hide(window, cx);
        }
    });
}

/// Shows the hovered element's docs when Shift is pressed. Called by the shell, which is on the
/// keyboard dispatch path.
pub fn modifiers_changed(event: &ModifiersChangedEvent, window: &mut Window, cx: &mut App) {
    if !event.modifiers.shift {
        return;
    }

    if let Some((anchor, build)) = state(cx).hovered.clone() {
        show(anchor, &build, window, cx);
    }
}

pub trait HoverDocsExt: ParentElement + Sized {
    /// Shows the global definition of a champion, item or rune when this element is hovered
    /// with Shift held (`Docs::ChampionGlobal`, `ItemGlobal`, `RuneGlobal` on the web).
    fn hover_docs<T: CastId>(self, id: T) -> Self {
        self.docs(move || vec![Highlighted::from(id.render_global_markup::<Spans>())])
    }

    /// Shows the blocks returned by `build` when this element is hovered with Shift held.
    fn docs(self, build: impl Fn() -> Vec<Highlighted> + 'static) -> Self {
        self.child(Anchor {
            build: Rc::new(build),
        })
    }
}

/// The element needs an id: the anchor is identified by its path of element ids.
impl<E: ParentElement> HoverDocsExt for Stateful<E> {}

/// Whether `inner` belongs to an element inside the parent of `outer`. An anchor's path ends
/// with its own id, after the path of the element it covers.
fn contains(outer: &[ElementId], inner: &[ElementId]) -> bool {
    let host = &outer[..outer.len() - 1];

    inner.len() > outer.len() && inner.starts_with(host)
}

/// Covers its parent, without taking part in its layout.
struct Anchor {
    build: BuildDocs,
}

impl IntoElement for Anchor {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for Anchor {
    type RequestLayoutState = ();
    type PrepaintState = (AnchorId, Hitbox);

    fn id(&self) -> Option<ElementId> {
        Some("hover-docs-anchor".into())
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        let mut style = Style {
            position: Position::Absolute,
            ..Style::default()
        };

        style.inset.top = Length::Definite(px(0.).into());
        style.inset.right = Length::Definite(px(0.).into());
        style.inset.bottom = Length::Definite(px(0.).into());
        style.inset.left = Length::Definite(px(0.).into());

        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let anchor: AnchorId = id.expect("the anchor has an id").iter().cloned().collect();

        if let Some(active) = &mut state(cx).active
            && active.anchor == anchor
        {
            active.anchor_bounds = Some(bounds);
        }

        // `Normal` does not change how the parent and its children handle the mouse.
        (anchor, window.insert_hitbox(bounds, HitboxBehavior::Normal))
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        (anchor, hitbox): &mut Self::PrepaintState,
        window: &mut Window,
        _: &mut App,
    ) {
        let anchor = anchor.clone();
        let hitbox = hitbox.clone();
        let build = self.build.clone();

        window.on_mouse_event(move |event: &MouseMoveEvent, phase, window, cx| {
            if !phase.bubble() {
                return;
            }

            // Occluded by the dropdown or the tooltip itself, or clipped by a scroll container.
            let hovered = hitbox.is_hovered(window);
            let state = state(cx);
            let was_hovered = state
                .hovered
                .as_ref()
                .is_some_and(|(hovered, _)| *hovered == anchor);

            if hovered && !was_hovered {
                // Nested elements with docs: the innermost one wins, as with `closest()` in
                // `hover.js`. Listeners run innermost first, so an outer anchor must not take
                // over from an anchor inside its element.
                if state
                    .hovered
                    .as_ref()
                    .is_some_and(|(hovered, _)| contains(&anchor, hovered))
                {
                    return;
                }

                state.hovered = Some((anchor.clone(), build.clone()));
            } else if !hovered && was_hovered {
                state.hovered = None;
            } else {
                return;
            }

            if hovered && event.modifiers.shift {
                show(anchor.clone(), &build, window, cx);
            }

            hide_when_unhovered(window, cx);
        });
    }
}

/// Draws the active tooltip. The shell renders this once, as its last child.
pub fn layer() -> impl IntoElement {
    deferred(Layer { tooltip: None }).with_priority(LAYER_PRIORITY)
}

struct Layer {
    tooltip: Option<gpui::AnyElement>,
}

impl IntoElement for Layer {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for Layer {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        let style = Style {
            position: Position::Absolute,
            ..Style::default()
        };

        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(active) = &mut state(cx).active else {
            return;
        };

        // Reset for the next frame, where the anchor records its bounds again.
        let Some(anchor) = active.anchor_bounds.take() else {
            state(cx).active = None;
            return;
        };

        let mut tooltip = active.view.clone().into_any_element();
        let size = tooltip.layout_as_root(AvailableSpace::min_size(), window, cx);
        let viewport = window.viewport_size();

        // `positionTooltip` in `hover.js`.
        let mut origin = point(anchor.left(), anchor.bottom());

        if origin.x + size.width > viewport.width - WINDOW_MARGIN {
            origin.x = (anchor.right() - size.width).max(WINDOW_MARGIN);
        }

        if origin.y + size.height > viewport.height - WINDOW_MARGIN {
            origin.y = (anchor.top() - size.height).max(WINDOW_MARGIN);
        }

        tooltip.prepaint_at(origin, window, cx);
        self.tooltip = Some(tooltip);
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(mut tooltip) = self.tooltip.take() else {
            return;
        };

        tooltip.paint(window, cx);

        // `hover.js` hides on `pointerout`; moving out of the window sends no mouse move.
        window.on_mouse_event(|_: &MouseExitEvent, _, window, cx| hide(window, cx));
    }
}

struct HoverDocs(Vec<Highlighted>);

impl HoverDocs {
    fn hovered(&mut self, hovered: &bool, window: &mut Window, cx: &mut Context<Self>) {
        let view = cx.entity_id();

        if let Some(active) = &mut state(cx).active
            && active.view.entity_id() == view
        {
            active.tooltip_hovered = *hovered;
            hide_when_unhovered(window, cx);
        }
    }
}

impl Render for HoverDocs {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // `sm:max-w-md sm:max-h-96 p-2 leading-6 text-base border border-std-800 bg-std-900`
        let tooltip = div()
            .id("hover-docs")
            .max_w(rems(28.))
            .max_h(rems(24.))
            .overflow_y_scroll()
            .p_2()
            .text_base()
            .border_1()
            .border_color(rgb(theme::STD_800))
            .bg(rgb(theme::STD_900))
            // Elements below the tooltip are not hovered or clicked through it.
            .occlude()
            .on_hover(cx.listener(Self::hovered));

        // `#hoverdocs_code`: `flex flex-col gap-2`.
        code_text(tooltip.flex().flex_col().gap_2())
            .children(self.0.iter().map(|block| block.styled_text()))
    }
}
