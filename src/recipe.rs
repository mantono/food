use crate::qty::{Quantity, Volume, Weight};
use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, MulAssign};
use std::path::PathBuf;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct Recipe {
    pub title: String,
    pub ingredients: Vec<Ingredient>,
    pub servings: Option<u8>,
}

impl Recipe {
    pub fn new(title: &str, ingredients: Vec<Ingredient>, servings: u8) -> Recipe {
        Recipe {
            title: title.to_string(),
            ingredients,
            servings: Some(servings),
        }
    }

    pub fn size(&self) -> usize {
        self.ingredients.len()
    }

    pub fn apply_serving_size(&mut self, size: u8) {
        let current: u8 = match self.servings {
            Some(n) if n == size => return,
            None => return,
            Some(n) => n,
        };

        let ratio: f32 = (size as f32) / (current as f32);
        self.ingredients
            .iter_mut()
            .for_each(|i: &mut Ingredient| i.mul_assign(ratio))
    }

    pub fn from_file(path: PathBuf) -> Option<Recipe> {
        let lines: Vec<String> = match std::fs::read_to_string(path) {
            Ok(content) => content.lines().map(str::to_owned).collect(),
            Err(_) => return None,
        };

        let title: String = lines
            .first()
            .clone()
            .expect("Expected a first line")
            .to_string();

        let mut servings: Option<u8> = None;

        let ingredients: Vec<Ingredient> = lines
            .iter()
            .inspect(|line| {
                if crate::SERVINGS_PATTERN.is_match(&line) {
                    let parts = line.split(':').collect::<Vec<&str>>();
                    let last = parts.last().unwrap();
                    let number: u8 = last.trim().parse().unwrap();
                    servings = Some(number)
                }
            })
            .filter(|line| crate::ITEM_PATTERN.is_match(line))
            .map(|line| Ingredient::parse(&line))
            .filter_map(Result::ok)
            .collect();

        Some(Recipe {
            title,
            ingredients,
            servings,
        })
    }
}

impl Ord for Recipe {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size().cmp(&other.size())
    }
}

impl PartialOrd for Recipe {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let servings = match self.servings {
            Some(n) => n.to_string(),
            None => "".to_string(),
        };
        write!(f, "{}, {}", self.title, servings)
    }
}

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
        let item: String = (*parts.first().unwrap()).to_string();
        crate::ITEM_PATTERN.replace_all(&item, "").to_lowercase()
    }
}

impl std::ops::Add for Ingredient {
    type Output = Ingredient;

    fn add(self, other: Ingredient) -> Ingredient {
        if self.item != other.item {
            panic!("Cannot add items of different type")
        }
        let quantity = self.amount + other.amount;
        Ingredient::new(self.item, quantity)
    }
}

impl std::ops::MulAssign<f32> for Ingredient {
    fn mul_assign(&mut self, rhs: f32) {
        self.amount *= rhs
    }
}

pub fn join_ingredients(mut recipes: Vec<Recipe>) -> Vec<Ingredient> {
    let ingredients: Vec<Ingredient> = recipes
        .iter_mut()
        .map(|r| r.ingredients.clone())
        .flatten()
        .collect();

    merge(ingredients)
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
    use crate::recipe::{divide_unit, merge};
    use crate::recipe::{Ingredient, Recipe};

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

    #[test]
    fn test_change_servings_size() {
        let ingredients: Vec<Ingredient> = vec![
            Ingredient::parse(" - milk, 5 dl").unwrap(),
            Ingredient::parse(" - eggs, 5").unwrap(),
        ];
        let mut recipe = Recipe::new("Pancakes", ingredients, 4u8);
        recipe.apply_serving_size(8u8);

        let milk: u32 = match &recipe.ingredients.first().unwrap().amount {
            Quantity::Volume(v) => v.as_milliliters(),
            _ => 0u32,
        };

        assert_eq!(1_000u32, milk);

        let eggs: u32 = match &recipe.ingredients.last().unwrap().amount {
            Quantity::Pieces(p) => *p,
            _ => 0u32,
        };

        assert_eq!(10u32, eggs);
    }
}
