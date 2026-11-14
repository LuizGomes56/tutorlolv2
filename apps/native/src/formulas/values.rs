//! Item and rune documentation: the web app's `ValueFormulas` component.
use crate::{
    components::{
        code::{Code, Highlighted},
        h2,
        image::Entry,
        paragraph, section,
        selector::{Selected, Selector},
    },
    random::random_variant,
};
use codec::Spans;
use gpui::{AppContext, Context, Entity, IntoElement, ParentElement, Render, Subscription, Window};
use math::{EntityKind, ValueId};

pub struct ValueFormulas<T: Entry + ValueId> {
    title: &'static str,
    selector: Entity<Selector<T>>,
    global: Highlighted,
    function: Highlighted,
    generator: Option<Highlighted>,
    _selected: Subscription,
}

impl<T: Entry + ValueId> ValueFormulas<T> {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let value = random_variant::<T>();
        let selector = cx.new(|cx| Selector::new(value, cx));

        let selected = cx.subscribe(&selector, |this, _, Selected(value), cx| {
            this.show(*value);
            cx.notify();
        });

        let title = const {
            match T::KIND {
                EntityKind::Item => "Items",
                EntityKind::Rune => "Runes",
                EntityKind::Champion => panic!("champions are not values"),
            }
        };

        let mut this = Self {
            title,
            selector,
            global: Highlighted::plain(""),
            function: Highlighted::plain(""),
            generator: None,
            _selected: selected,
        };

        this.show(value);
        this
    }

    fn show(&mut self, value: T) {
        self.global = value.render_global_markup::<Spans>().into();
        self.function = value.render_fn_markup::<Spans>().into();
        self.generator = value
            .render_generator_markup::<Spans>()
            .map(|spans| Highlighted::new(&spans));
    }
}

impl<T: Entry + ValueId> Render for ValueFormulas<T> {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let generator = self.generator.clone().map(|generator| {
            [
                h2("Implementation").into_any_element(),
                Code(generator).into_any_element(),
            ]
        });

        section()
            .child(h2(self.title))
            .child(paragraph(
                "Documentation for the internal source code being used to evaluate its damage",
            ))
            .child(self.selector.clone())
            .child(h2("Source code definition"))
            .child(Code(self.global.clone()))
            .child(Code(self.function.clone()))
            .children(generator.into_iter().flatten())
    }
}
