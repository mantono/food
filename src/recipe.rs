use crate::qty::{Quantity, Volume, Weight};
use itertools::Itertools;
use regex::Regex;
use std::fmt;
use std::ops::Add;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct Ingredient {
    pub item: String,
    pub amount: Quantity,
}

impl std::fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.item, self.amount)
    }
}

impl Ingredient {
    pub fn parse(input: &str) -> Result<Ingredient, String> {
        let parts: Vec<&str> = input.trim().split(',').collect();
        match parts.len() {
            1 => {
                let ingredient = Ingredient {
                    item: Ingredient::extract_ingredient(&parts),
                    amount: Quantity::Pieces(1),
                };
                Ok(ingredient)
            }
            2 => {
                let ingredient = Ingredient {
                    item: Ingredient::extract_ingredient(&parts),
                    amount: Quantity::parse(parts[1])?,
                };
                Ok(ingredient)
            }
            _ => Err(format!("Invalid line '{}'", input)),
        }
    }

    pub fn new(item: String, amount: Quantity) -> Ingredient {
        Ingredient { item, amount }
    }

    fn extract_ingredient(parts: &[&str]) -> String {
        let pattern = Regex::new(r"^\s*-\s*").unwrap();
        let item: String = (*parts.first().unwrap()).to_string();
        pattern.replace_all(&item, "").to_string()
    }
}

impl std::ops::Add for Ingredient {
    type Output = Ingredient;

    fn add(self, other: Ingredient) -> Ingredient {
        if self.item != other.item {
            panic!("Cannot add items of different type")
        }
        let quantity = self.amount + other.amount;
        Ingredient::new(self.item.clone(), quantity)
    }
}

pub fn merge(mut ingredients: Vec<Ingredient>) -> Vec<Ingredient> {
    ingredients.sort_by(|i0, i1| i0.item.cmp(&i1.item));
    ingredients
        .iter()
        .group_by(|i| i.item.clone())
        .into_iter()
        .map(|(_, v)| sum(v.collect::<Vec<&Ingredient>>()))
        .collect()
}

fn sum(ingredients: Vec<&Ingredient>) -> Ingredient {
    ingredients[1..]
        .iter()
        .fold(ingredients[0].clone(), |i0, i1| try_add(&i0, i1))
}

fn try_add(i0: &Ingredient, i1: &Ingredient) -> Ingredient {
    if i0.item != i1.item {
        (*i0).clone()
    } else {
        (*i0).clone().add((*i1).clone())
    }
}

pub fn divide_unit(i: &Ingredient) -> Ingredient {
    let q: Quantity = match &i.amount {
        Quantity::Weight(w) => {
            let grams: u32 = w.as_grams();
            let weight: Weight = if grams % 1_000 == 0 {
                Weight::Kilogram(grams / 1_000)
            } else {
                Weight::Gram(grams)
            };
            Quantity::Weight(weight)
        }
        Quantity::Volume(v) => {
            let milliliters: u32 = v.as_milliliters();
            let volume: Volume = if milliliters % 1_000 == 0 {
                Volume::Liter(milliliters / 1_000)
            } else if milliliters % 100 == 0 {
                Volume::Deciliter(milliliters / 100)
            } else if milliliters % 10 == 0 {
                Volume::Centiliter(milliliters / 10)
            } else {
                Volume::Milliliter(milliliters)
            };
            Quantity::Volume(volume)
        }
        _ => i.amount.clone(),
    };
    Ingredient {
        item: i.item.clone(),
        amount: q,
    }
}

#[cfg(test)]
mod tests {
    use crate::qty::{Quantity, Volume};
    use crate::recipe::Ingredient;
    use crate::recipe::{divide_unit, merge};

    #[test]
    fn test_parse_single_ingredient() {
        let ingr = Ingredient::parse("milk, 2 l").unwrap();
        assert_eq!("milk", ingr.item);
        assert_eq!(Quantity::Volume(Volume::Liter(2)), ingr.amount);
    }

    #[test]
    fn test_parse_single_ingredient_with_dashes_and_whitespace() {
        let ingr = Ingredient::parse(" - milk, 2 l").unwrap();
        assert_eq!("milk", ingr.item);
        assert_eq!(Quantity::Volume(Volume::Liter(2)), ingr.amount);
    }

    #[test]
    fn test_merge_same_ingredient_same_unit() {
        let items: Vec<Ingredient> = vec![
            Ingredient::parse(" - milk, 5 dl").unwrap(),
            Ingredient::parse(" - milk, 4 dl").unwrap(),
        ];

        let items: Vec<Ingredient> = merge(items);
        let milk: &Ingredient = items.first().unwrap();
        assert_eq!(Quantity::Volume(Volume::Milliliter(900)), milk.amount)
    }

    #[test]
    fn test_merge_same_ingredient_different() {
        let items: Vec<Ingredient> = vec![
            Ingredient::parse(" - milk, 5 dl").unwrap(),
            Ingredient::parse(" - milk, 1 l").unwrap(),
        ];

        let items: Vec<Ingredient> = merge(items);
        let milk: &Ingredient = items.first().unwrap();
        assert_eq!(Quantity::Volume(Volume::Milliliter(1500u32)), milk.amount)
    }

    #[test]
    fn test_change_unit_to_most_human_readable() {
        let items: Vec<Ingredient> = vec![
            Ingredient::parse(" - milk, 5 dl").unwrap(),
            Ingredient::parse(" - milk, 1 l").unwrap(),
        ];

        let items: Vec<Ingredient> = merge(items);
        let milk: &Ingredient = items.first().unwrap();
        let milk: Ingredient = divide_unit(milk);
        assert_eq!(Quantity::Volume(Volume::Deciliter(15u32)), milk.amount)
    }
}
