use crate::character::AttributeLike;
use crate::{Client, Error};
use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
struct FetchError {
    url: String,
    status: reqwest::StatusCode,
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to fetch URL: {}. Status: {}",
            self.url, self.status
        )
    }
}

impl Error for FetchError {}

pub async fn fetch_json(url: &str, client: &Client) -> Result<String, Box<dyn Error>> {
    let response = client.get(url).send().await?;

    if response.status().is_success() {
        response.text().await.map_err(Into::into)
    } else {
        Err(Box::new(FetchError {
            url: url.to_string(),
            status: response.status(),
        }))
    }
}

pub fn combine_attr_fields<T: AttributeLike>(a: &[T], b: &[T]) -> HashSet<String> {
    a.iter()
        .map(|x| x.field().clone())
        .chain(b.iter().map(|x| x.field().clone()))
        .collect()
}

pub fn search<'a, T: AttributeLike>(lst: &'a [T], field: &str) -> Option<&'a T> {
    lst.iter().find(|x| x.field() == field)
}

pub fn comb_attr_stats<T: AttributeLike>(a: &T, b: &T) -> String {
    if a.is_percent() != b.is_percent() {
        return format!("{} + {}", a.display(), b.display());
    }

    let is_percent = a.is_percent();
    let format_value = |x: f64| {
        if is_percent {
            ((x * 1000.0).round() / 10.0) as i32
        } else {
            x as i32
        }
    };
    let sum = format_value(a.value()) + format_value(b.value());

    if is_percent {
        format!("{:.1}%", sum as f64 / 10.0)
    } else {
        sum.to_string()
    }
}

pub fn comb_stats<T: AttributeLike>(lst1: &[T], lst2: &[T], field: &str) -> String {
    let a = search(lst1, field);
    let b = search(lst2, field);

    match (a, b) {
        (None, None) => "N/A".to_string(),
        (None, Some(b)) => b.display().clone(),
        (Some(a), None) => a.display().clone(),
        (Some(a), Some(b)) => comb_attr_stats(a, b),
    }
}

pub fn get_atk_boosts<T: AttributeLike>(ch_properties: &[T]) -> Vec<Vec<String>> {
    let elemental: Vec<&T> = ch_properties
        .iter()
        .filter(|x| x.field().ends_with("_dmg") && x.field() != "all_dmg")
        .collect();

    let all_dmg = search(ch_properties, "all_dmg");

    if elemental.is_empty() {
        return all_dmg
            .map(|x| {
                vec![
                    x.field().clone(),
                    x.icon().clone(),
                    x.name().clone(),
                    x.display().clone(),
                ]
            })
            .into_iter()
            .collect();
    }

    elemental
        .into_iter()
        .map(|x| {
            vec![
                x.field().clone(),
                x.icon().clone(),
                x.name().clone(),
                if let Some(all_dmg) = all_dmg {
                    comb_attr_stats(x, all_dmg)
                } else {
                    x.display().clone()
                },
            ]
        })
        .collect()
}

pub fn sort_fields(lst: &[String]) -> Vec<String> {
    let ref_fields = vec![
        "crit_rate",
        "crit_dmg",
        "break_dmg",
        "heal_rate",
        "sp_rate",
        "effect_hit",
        "effect_res",
    ];

    let mut sorted_list = lst.to_vec();
    sorted_list.sort_by_key(|x| {
        ref_fields
            .iter()
            .position(|&ref_field| ref_field == x)
            .unwrap_or(ref_fields.len())
    });

    sorted_list
}
