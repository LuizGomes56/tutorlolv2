use crate::{html::Html, parallel_task};
use tutorlolv2_gen::{CastId, ChampionId, Position};

pub fn champions_html() {
    parallel_task(|champion_id: ChampionId| {
        let mut html = Html::new(champion_id);

        html.push_str(&{
            let recommendation_cells: [[String; 2]; Position::VARIANTS as usize] =
                core::array::from_fn(|i| {
                    let position = Position::ARRAY[i];

                    fn cell<'a, T: CastId + 'a>(array: impl IntoIterator<Item = &'a T>) -> String {
                        let cells = array
                            .into_iter()
                            .map(|&value| {
                                let src = Html::src(value);
                                let entity_id = value.entity();
                                let name = value.name();
                                let folder = match entity_id {
                                    x if x.is_item() => "items",
                                    x if x.is_rune() => "runes",
                                    x if x.is_champion() => "champions",
                                    _ => unreachable!(),
                                };
                                format!(
                                    r#"
                                    <a 
                                        href="../../{folder}/{value:?}/index.html" 
                                        class="flex items-center gap-2"
                                    >
                                        <img src={src:?} alt={value:?} title={name:?}>
                                        <span>{name}</span>
                                    </a>
                                    "#
                                )
                            })
                            .collect::<String>();

                        format!(
                            r#"
                            <td class="content-baseline">
                                <div class="flex flex-col gap-2 py-2">
                                    {cells}
                                </div>
                            </td>
                            "#
                        )
                    }

                    [
                        cell(champion_id.recommended_items(position)),
                        cell(champion_id.recommended_runes(position)),
                    ]
                });

            let headers = Position::ARRAY
                .iter()
                .map(|&position| {
                    let src = Html::img_src(&format!("other/{position:?}.svg"));

                    format!(
                        r#"
                        <th>
                            <div class="flex items-center gap-2">
                                <img src={src:?} alt="{position:?}">
                                <span>{position:?}</span>
                            </div>
                        </th>
                        "#
                    )
                })
                .collect::<String>();

            let mut result = format!(
                "<table>
                    <thead>{headers}</thead>
                    <tbody>
                        <tr>"
            );

            recommendation_cells
                .iter()
                .for_each(|[items, _]| result.push_str(items));

            result.push_str("</tr><tr>");

            recommendation_cells
                .iter()
                .for_each(|[_, runes]| result.push_str(runes));

            result.push_str("</tr></tbody></table>");
            result
        });

        html.section("Source Code Representation")
            .code(champion_id.formula())
            .section(&format!(
                "This champion has {x} abilit{postfix}",
                x = champion_id.number_of_abilities(),
                postfix = if champion_id.number_of_abilities() == 1 {
                    "y"
                } else {
                    "ies"
                }
            ));

        champion_id
            .abilities()
            .into_iter()
            .enumerate()
            .for_each(|(i, meta)| {
                let lit = format!("{kind:?}", kind = meta.kind);
                html.section(&lit)
                    .code(champion_id.get_ability_formula(i))
                    .describe()
                    .idents(champion_id.get_ability_idents(i))
                    .code(champion_id.get_ability_closure(i));
            });

        html.section("Generator definition")
            .code(champion_id.generator())
            .json(champion_id);
        html
    });
}
