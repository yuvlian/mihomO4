pub mod prop {
    use crate::CharacterData;
    use crate::PropertyData;
    use std::collections::HashSet;

    pub fn combine_prop_fields<'a>(
        a: &'a [PropertyData],
        b: &'a [PropertyData],
    ) -> HashSet<&'a str> {
        a.iter()
            .map(|x| x.field.as_str())
            .chain(b.iter().map(|x| x.field.as_str()))
            .collect()
    }

    pub fn search_prop_field<'a>(
        prop: &'a [PropertyData],
        field: &str,
    ) -> Option<&'a PropertyData> {
        prop.iter().find(|x| x.field == field)
    }

    pub fn comb_prop_stats(a: &PropertyData, b: &PropertyData) -> String {
        if a.is_percent != b.is_percent {
            return format!("{:?} + {:?}", a, b);
        };

        let format_val = |x: f64| match a.is_percent {
            true => ((x * 1000f64).round() / 10f64) as i32,
            _ => x as i32,
        };

        let sum = (format_val(a.value) + format_val(b.value)) as f64;

        match a.is_percent {
            true => format!("{:.1}%", sum / 10f64),
            _ => sum.to_string(),
        }
    }

    pub fn comb_prop_list_stats(
        prop_one: &[PropertyData],
        prop_two: &[PropertyData],
        field: &str,
    ) -> String {
        let a = search_prop_field(prop_one, field);
        let b = search_prop_field(prop_two, field);

        match (a, b) {
            (Some(a), Some(b)) => comb_prop_stats(a, b),
            (None, Some(b)) => String::from(&b.display),
            (Some(a), None) => String::from(&a.display),
            (_, _) => String::from("N/A"),
        }
    }

    pub fn comb_prop_dmg_boost(ch: &CharacterData) -> Vec<Vec<String>> {
        let elemental: Vec<&PropertyData> = ch
            .properties
            .iter()
            .filter(|x| {
                x.r#type.ends_with("AddedRatio")
                    && x.field.ends_with("_dmg")
                    && x.field != "all_dmg"
            })
            .collect();

        let all_dmg = search_prop_field(&ch.properties, "all_dmg");

        if elemental.is_empty() {
            all_dmg
                .map(|all_dmg| {
                    vec![
                        all_dmg.field.to_string(),
                        all_dmg.icon.to_string(),
                        all_dmg.name.to_string(),
                        all_dmg.display.clone(),
                    ]
                })
                .into_iter()
                .collect()
        } else {
            elemental
                .into_iter()
                .map(|x| {
                    let display_value = if let Some(all_dmg) = &all_dmg {
                        comb_prop_stats(x, all_dmg)
                    } else {
                        x.display.clone()
                    };

                    vec![
                        x.field.to_string(),
                        x.icon.to_string(),
                        x.name.to_string(),
                        display_value,
                    ]
                })
                .collect()
        }
    }
}

pub mod attr {
    use crate::AttributeData;
    use crate::CharacterData;
    use std::collections::HashSet;

    pub fn combine_attr_fields<'a>(
        a: &'a [AttributeData],
        b: &'a [AttributeData],
    ) -> HashSet<&'a str> {
        a.iter()
            .map(|x| x.field.as_str())
            .chain(b.iter().map(|x| x.field.as_str()))
            .collect()
    }

    pub fn search_attr_field<'a>(
        attr: &'a [AttributeData],
        field: &str,
    ) -> Option<&'a AttributeData> {
        attr.iter().find(|x| x.field == field)
    }

    pub fn comb_attr_stats(a: &AttributeData, b: &AttributeData) -> String {
        if a.is_percent != b.is_percent {
            return format!("{:?} + {:?}", a, b);
        };

        let format_val = |x: f64| match a.is_percent {
            true => ((x * 1000f64).round() / 10f64) as i32,
            _ => x as i32,
        };

        let sum = (format_val(a.value) + format_val(b.value)) as f64;

        match a.is_percent {
            true => format!("{:.1}%", sum / 10f64),
            _ => sum.to_string(),
        }
    }

    pub fn comb_attr_list_stats(
        attr_one: &[AttributeData],
        attr_two: &[AttributeData],
        field: &str,
    ) -> String {
        let a = search_attr_field(attr_one, field);
        let b = search_attr_field(attr_two, field);

        match (a, b) {
            (Some(a), Some(b)) => comb_attr_stats(a, b),
            (None, Some(b)) => String::from(&b.display),
            (Some(a), None) => String::from(&a.display),
            (_, _) => String::from("N/A"),
        }
    }

    pub fn comb_attr_dmg_boost(ch: &CharacterData) -> Vec<Vec<String>> {
        let elemental: Vec<&AttributeData> = ch
            .attributes
            .iter()
            .filter(|x| {
                // x.r#type.ends_with("AddedRatio") &&
                x.field.ends_with("_dmg") && x.field != "all_dmg"
            })
            .collect();

        let all_dmg = search_attr_field(&ch.attributes, "all_dmg");

        if elemental.is_empty() {
            all_dmg
                .map(|all_dmg| {
                    vec![
                        all_dmg.field.to_string(),
                        all_dmg.icon.to_string(),
                        all_dmg.name.to_string(),
                        all_dmg.display.clone(),
                    ]
                })
                .into_iter()
                .collect()
        } else {
            elemental
                .into_iter()
                .map(|x| {
                    let display_value = if let Some(all_dmg) = &all_dmg {
                        comb_attr_stats(x, all_dmg)
                    } else {
                        x.display.clone()
                    };

                    vec![
                        x.field.to_string(),
                        x.icon.to_string(),
                        x.name.to_string(),
                        display_value,
                    ]
                })
                .collect()
        }
    }
}

pub fn sort_fields(list: Vec<String>) -> Vec<String> {
    let ref_fields = vec![
        "crit_rate",
        "crit_dmg",
        "break_dmg",
        "heal_rate",
        "sp_rate",
        "effect_hit",
        "effect_res",
    ];

    let mut sorted_list = list;
    sorted_list.sort_by_key(|x| {
        ref_fields
            .iter()
            .position(|&ref_field| ref_field == x)
            .unwrap_or(ref_fields.len())
    });

    sorted_list
}
