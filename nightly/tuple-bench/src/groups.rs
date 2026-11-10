//! Benchmark-only concrete tuple dispatch. No tuple type erasure.
use new::{
    AbilityId, Ctx, TypeMetadata,
    model::{Modifiers, RangeDamage},
};
pub type Formula = fn(&Ctx) -> f32;
pub type Batch = fn(&Ctx, Modifiers, &mut RangeDamage, &mut [i32]);
pub const trait ClosureGroup {
    fn len(&self) -> usize;
    fn eval_at(&self, index: usize, ctx: &Ctx) -> f32;
}

macro_rules! tuple_groups {
    ($(($len:literal; $($ty:ident:$field:tt),*)),* $(,)?) => {$ (
        const impl<$($ty,)*> ClosureGroup for ($($ty,)*)
        where $($ty: [const] Fn(&Ctx) -> f32,)*
        {
            fn len(&self) -> usize { $len }
            fn eval_at(&self, index: usize, ctx: &Ctx) -> f32 {
                // The unit tuple has no callable fields.
                let _ = ctx;
                match index {
                    $($field => (self.$field)(ctx),)*
                    _ => panic!("closure group index out of bounds"),
                }
            }
        }
    )*};
}

tuple_groups! {
    (0; ),
    (1; F0:0),
    (2; F0:0, F1:1),
    (3; F0:0, F1:1, F2:2),
    (4; F0:0, F1:1, F2:2, F3:3),
    (5; F0:0, F1:1, F2:2, F3:3, F4:4),
    (6; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5),
    (7; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6),
    (8; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7),
    (9; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7, F8:8),
    (10; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7, F8:8, F9:9),
    (11; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7, F8:8, F9:9, F10:10),
    (12; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7, F8:8, F9:9, F10:10, F11:11),
    (13; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7, F8:8, F9:9, F10:10, F11:11, F12:12),
    (14; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7, F8:8, F9:9, F10:10, F11:11, F12:12, F13:13),
    (15; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7, F8:8, F9:9, F10:10, F11:11, F12:12, F13:13, F14:14),
    (16; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7, F8:8, F9:9, F10:10, F11:11, F12:12, F13:13, F14:14, F15:15),
    (17; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7, F8:8, F9:9, F10:10, F11:11, F12:12, F13:13, F14:14, F15:15, F16:16),
    (18; F0:0, F1:1, F2:2, F3:3, F4:4, F5:5, F6:6, F7:7, F8:8, F9:9, F10:10, F11:11, F12:12, F13:13, F14:14, F15:15, F16:16, F17:17),
}

pub struct PointerGroup<'a>(pub &'a [Formula]);
impl ClosureGroup for PointerGroup<'_> {
    fn len(&self) -> usize {
        self.0.len()
    }
    fn eval_at(&self, index: usize, ctx: &Ctx) -> f32 {
        (self.0[index])(ctx)
    }
}
pub struct EnumGroup<'a>(pub &'a [new::Closure]);
const impl ClosureGroup for EnumGroup<'_> {
    fn len(&self) -> usize {
        self.0.len()
    }
    fn eval_at(&self, index: usize, ctx: &Ctx) -> f32 {
        (self.0[index])(ctx)
    }
}

pub const fn raw<C: [const] ClosureGroup>(ctx: &Ctx, group: &C, out: &mut [f32]) {
    debug_assert!(out.len() == group.len());
    let mut i = 0;
    while i < group.len() {
        out[i] = group.eval_at(i, ctx);
        i += 1;
    }
}
pub const fn abilities<C: [const] ClosureGroup>(
    ctx: &Ctx,
    onhit: &mut RangeDamage,
    modifiers: Modifiers,
    metadata: &[TypeMetadata<AbilityId>],
    group: &C,
    out: &mut [i32],
) {
    debug_assert!(out.len() == metadata.len());
    debug_assert!(metadata.len() == group.len());
    let mut i = 0;
    while i < metadata.len() {
        let meta = metadata[i];
        let modifier = new::helpers::ability_id_mod(meta.kind, meta.damage_type, modifiers);
        let damage = (modifier * group.eval_at(i, ctx)) as i32;
        onhit.inc_attr(meta.attributes, damage);
        out[i] = damage;
        i += 1;
    }
}
pub const fn pairs<C: [const] ClosureGroup, M>(
    ctx: &Ctx,
    onhit: &mut RangeDamage,
    modifiers: Modifiers,
    metadata: &[TypeMetadata<M>],
    group: &C,
    out: &mut [i32],
) {
    debug_assert!(group.len() == metadata.len() * 2);
    debug_assert!(out.len() == group.len());
    let mut i = 0;
    while i < metadata.len() {
        let modifier = modifiers.damages.modifier(metadata[i].damage_type);
        let mut j = 0;
        while j < 2 {
            let index = i * 2 + j;
            let damage = (modifier * group.eval_at(index, ctx)) as i32;
            onhit.inc_attr(metadata[i].attributes, damage);
            out[index] = damage;
            j += 1;
        }
        i += 1;
    }
}
pub fn pointer_pairs<T>(
    ctx: &Ctx,
    onhit: &mut RangeDamage,
    modifiers: Modifiers,
    metadata: &[TypeMetadata<T>],
    closures: &[Formula],
    result: &mut [i32],
) {
    let out_len = closures.len();

    debug_assert_eq!(out_len, metadata.len() << 1);

    let mut meta_index = 0usize;
    let mut out_index = 0usize;

    while meta_index < metadata.len() {
        let TypeMetadata {
            damage_type,
            attributes,
            ..
        } = *unsafe { metadata.get_unchecked(meta_index) };

        let modifier = modifiers.damages.modifier(damage_type);
        let mut j = 0;

        while j < 2 {
            let closure = unsafe { closures.get_unchecked((meta_index << 1) + j) };
            let damage = (modifier * closure(ctx)) as i32;

            onhit.inc_attr(attributes, damage);
            result[out_index] = damage;
            out_index += 1;
            j += 1;
        }

        meta_index += 1;
    }

    debug_assert_eq!(out_index, out_len);
}

pub fn pointer_abilities(
    ctx: &Ctx,
    onhit: &mut RangeDamage,
    modifiers: Modifiers,
    metadata: &[TypeMetadata<AbilityId>],
    closures: &[Formula],
    out: &mut [i32],
) {
    let len = metadata.len();

    debug_assert_eq!(len, closures.len());
    debug_assert_eq!(len, out.len());

    let mut i = 0;

    while i < len {
        let TypeMetadata {
            kind,
            damage_type,
            attributes,
        } = metadata[i];

        let closure = unsafe { closures.get_unchecked(i) };
        let modifier = new::helpers::ability_id_mod(kind, damage_type, modifiers);
        let damage = (modifier * closure(ctx)) as i32;

        onhit.inc_attr(attributes, damage);
        out[i] = damage;

        i += 1;
    }
}

include!(concat!(env!("OUT_DIR"), "/tuple_groups.rs"));
