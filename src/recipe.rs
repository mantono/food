use itertools::Itertools;
use regex::Regex;
use std::fmt;
use std::ops::Add;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct Ingredient {
    item: String,
    amount: Quantity,
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

    fn extract_ingredient(parts: &Vec<&str>) -> String {
        let pattern = Regex::new(r"^\s*-\s*").unwrap();
        let item: String = parts.first().unwrap().to_string();
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

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum Quantity {
    Pieces(u32),
    Weight(Weight),
    Volume(Volume),
    Custom(u32, String),
}

trait StdUnit {
    fn in_std_unit() -> u32;
}

impl std::ops::Add for Quantity {
    type Output = Quantity;

    fn add(self, other: Quantity) -> Quantity {
        let this = self.clone();
        match (self, other) {
            (Quantity::Pieces(n0), Quantity::Pieces(n1)) => Quantity::Pieces(n0 + n1),
            (Quantity::Custom(n0, type0), Quantity::Custom(n1, type1)) if type0 == type1 => Quantity::Custom(n0 + n1, type0),
            (Quantity::Volume(n0), Quantity::Volume(n1)) => Quantity::Volume(n0 + n1),
            (Quantity::Weight(n0), Quantity::Weight(n1)) => Quantity::Weight(n0 + n1),
            _ => this,
        }
    }
}

impl Quantity {
    pub fn parse(input: &str) -> Result<Quantity, String> {
        if input.is_empty() {
            return Ok(Quantity::Pieces(1));
        }

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let num_input: &str = parts.first().unwrap();
        let number: u32 = match num_input.parse() {
            Ok(0) => return Err(String::from("Invalid amount: 0")),
            Ok(n) => n,
            Err(_) => {
                let err_msg: String = format!("Invalid quantifier/integer: {}", num_input);
                return Err(err_msg);
            }
        };
        let quantifier: &str = &parts[1..].join(" ").to_lowercase();
        let parsed_quantity: Quantity = match quantifier {
            "" => Quantity::Pieces(number),
            "l" | "liter" | "liters" => Quantity::Volume(Volume::Liter(number)),
            "dl" | "deciliter" | "deciliters" => Quantity::Volume(Volume::Deciliter(number)),
            "cl" | "centiliter" | "centiliters" => Quantity::Volume(Volume::Centiliter(number)),
            "ml" | "milliliter" | "milliliters" => Quantity::Volume(Volume::Milliliter(number)),
            "tbsp" | "tb" | "msk" | "matsked" | "tablespoon" | "tablespoons" => Quantity::Volume(Volume::Tablespoon(number)),
            "tspn" | "tsp" | "ts" | "tsk" | "tesked" | "teaspoon" | "teaspoons" => Quantity::Volume(Volume::Teaspoon(number)),
            "krm" | "kryddmått" => Quantity::Volume(Volume::Spices(number)),
            "p" | "pt" | "pint" | "pints" => Quantity::Volume(Volume::Pints(number)),
            "cup" | "cups" => Quantity::Volume(Volume::Cups(number)),
            "fl oz" | "fluid ounce" | "fluid ounces" => Quantity::Volume(Volume::Ounces(number)),

            "g" | "gram" | "grams" => Quantity::Weight(Weight::Gram(number)),
            "kg" | "kilogram" | "kilograms" => Quantity::Weight(Weight::Kilogram(number)),
            "oz" | "ounce" | "ounces" => Quantity::Weight(Weight::Ounces(number)),
            "lbs" | "pound" | "pounds" => Quantity::Weight(Weight::Pounds(number)),
            _ => Quantity::Custom(number, quantifier.to_string()),
        };
        Ok(parsed_quantity)
    }
}

trait Quantifiable {
    fn amount(&self) -> u32;
    fn unit(&self) -> &str;
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (number, unit): (u32, &str) = match self {
            Quantity::Weight(w) => (w.amount(), w.unit()),
            Quantity::Volume(v) => (v.amount(), v.unit()),
            Quantity::Pieces(n) => (*n, ""),
            Quantity::Custom(n, t) => (*n, t),
        };
        write!(f, "{} {}", number, unit)
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum Weight {
    Kilogram(u32),
    Gram(u32),
    Pounds(u32),
    Ounces(u32),
}

impl Weight {
    pub fn as_grams(&self) -> u32 {
        match self {
            Weight::Kilogram(w) => 1000 * w,
            Weight::Gram(w) => *w,
            Weight::Pounds(w) => ((*w as f32) * 453.59237) as u32,
            Weight::Ounces(w) => ((*w as f32) * 28.349523125) as u32,
        }
    }
}

impl std::ops::Add for Weight {
    type Output = Weight;

    fn add(self, other: Weight) -> Weight {
        let sum: u32 = self.as_grams() + other.as_grams();
        Weight::Gram(sum)
    }
}

impl Quantifiable for Weight {
    fn amount(&self) -> u32 {
        *match self {
            Weight::Gram(g) => g,
            Weight::Pounds(lbs) => lbs,
            Weight::Ounces(oz) => oz,
            Weight::Kilogram(kg) => kg,
        }
    }

    fn unit(&self) -> &str {
        match self {
            Weight::Gram(_) => "g",
            Weight::Kilogram(_) => "kg",
            Weight::Ounces(_) => "oz",
            Weight::Pounds(_) => "lbs",
        }
    }
}

impl Quantifiable for Volume {
    fn amount(&self) -> u32 {
        *match self {
            Volume::Ounces(fl_oz) => fl_oz,
            Volume::Cups(cups) => cups,
            Volume::Pints(pints) => pints,
            Volume::Spices(spices) => spices,
            Volume::Teaspoon(teaspoon) => teaspoon,
            Volume::Tablespoon(tablespoon) => tablespoon,
            Volume::Milliliter(ml) => ml,
            Volume::Centiliter(cl) => cl,
            Volume::Deciliter(dl) => dl,
            Volume::Liter(l) => l,
        }
    }

    fn unit(&self) -> &str {
        match self {
            Volume::Ounces(_) => "fl oz",
            Volume::Cups(_) => "cups",
            Volume::Pints(_) => "pints",
            Volume::Spices(_) => "spices",
            Volume::Teaspoon(_) => "teaspoon",
            Volume::Tablespoon(_) => "tablespoon",
            Volume::Milliliter(_) => "ml",
            Volume::Centiliter(_) => "cl",
            Volume::Deciliter(_) => "dl",
            Volume::Liter(_) => "l",
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum Volume {
    Liter(u32),
    Deciliter(u32),
    Centiliter(u32),
    Milliliter(u32),
    Tablespoon(u32),
    Teaspoon(u32),
    Spices(u32),
    Pints(u32),
    Cups(u32),
    Ounces(u32),
}

impl Volume {
    pub fn as_milliliters(&self) -> u32 {
        match self {
            Volume::Liter(v) => v * 1000,
            Volume::Deciliter(v) => v * 100,
            Volume::Centiliter(v) => v * 10,
            Volume::Milliliter(v) | Volume::Spices(v) => *v,
            Volume::Tablespoon(v) => v * 15,
            Volume::Teaspoon(v) => v * 5,
            Volume::Pints(v) => v * 473,
            Volume::Ounces(v) => ((*v as f32) * 29.6) as u32,
            Volume::Cups(v) => 237 * v,
        }
        .clone()
    }
}

impl std::ops::Add for Volume {
    type Output = Volume;

    fn add(self, other: Volume) -> Volume {
        let sum: u32 = self.as_milliliters() + other.as_milliliters();
        Volume::Milliliter(sum)
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
    use crate::recipe::{divide_unit, Weight};
    use crate::recipe::{merge, Volume};
    use crate::recipe::{Ingredient, Quantity};
    use log::Level::Warn;

    #[test]
    fn test_parse_quantity_volume_liter() {
        assert_eq!(Quantity::Volume(Volume::Liter(1)), Quantity::parse("1 liter").unwrap());
        assert_eq!(Quantity::Volume(Volume::Liter(2)), Quantity::parse("2 l").unwrap());
        assert_eq!(Quantity::Volume(Volume::Liter(3)), Quantity::parse("3 liters").unwrap());
    }

    #[test]
    fn test_parse_quantity_volume_deciliter() {
        assert_eq!(
            Quantity::Volume(Volume::Deciliter(1)),
            Quantity::parse("1 deciliter").unwrap()
        );
        assert_eq!(Quantity::Volume(Volume::Deciliter(2)), Quantity::parse("2 dl").unwrap());
        assert_eq!(
            Quantity::Volume(Volume::Deciliter(3)),
            Quantity::parse("3 deciliters").unwrap()
        );
    }

    #[test]
    fn test_parse_quantity_volume_centiliter() {
        assert_eq!(
            Quantity::Volume(Volume::Centiliter(1)),
            Quantity::parse("1 centiliter").unwrap()
        );
        assert_eq!(Quantity::Volume(Volume::Centiliter(2)), Quantity::parse("2 cl").unwrap());
        assert_eq!(
            Quantity::Volume(Volume::Centiliter(3)),
            Quantity::parse("3 centiliters").unwrap()
        );
    }

    #[test]
    fn test_parse_quantity_volume_milliliter() {
        assert_eq!(
            Quantity::Volume(Volume::Milliliter(1)),
            Quantity::parse("1 milliliter").unwrap()
        );
        assert_eq!(Quantity::Volume(Volume::Milliliter(2)), Quantity::parse("2 ml").unwrap());
        assert_eq!(
            Quantity::Volume(Volume::Milliliter(3)),
            Quantity::parse("3 milliliters").unwrap()
        );
    }

    #[test]
    fn test_parse_quantity_volume_tablespoon() {
        assert_eq!(
            Quantity::Volume(Volume::Tablespoon(1)),
            Quantity::parse("1 tablespoon").unwrap()
        );
        assert_eq!(Quantity::Volume(Volume::Tablespoon(2)), Quantity::parse("2 tbsp").unwrap());
        assert_eq!(Quantity::Volume(Volume::Tablespoon(2)), Quantity::parse("2 tb").unwrap());
        assert_eq!(Quantity::Volume(Volume::Tablespoon(2)), Quantity::parse("2 msk").unwrap());
        assert_eq!(
            Quantity::Volume(Volume::Tablespoon(3)),
            Quantity::parse("3 tablespoons").unwrap()
        );
    }

    #[test]
    fn test_parse_quantity_volume_teaspoon() {
        assert_eq!(Quantity::Volume(Volume::Teaspoon(1)), Quantity::parse("1 teaspoon").unwrap());
        assert_eq!(Quantity::Volume(Volume::Teaspoon(2)), Quantity::parse("2 tsp").unwrap());
        assert_eq!(Quantity::Volume(Volume::Teaspoon(2)), Quantity::parse("2 tspn").unwrap());
        assert_eq!(Quantity::Volume(Volume::Teaspoon(2)), Quantity::parse("2 ts").unwrap());
        assert_eq!(Quantity::Volume(Volume::Teaspoon(2)), Quantity::parse("2 tsk").unwrap());
        assert_eq!(Quantity::Volume(Volume::Teaspoon(2)), Quantity::parse("2 tesked").unwrap());
        assert_eq!(Quantity::Volume(Volume::Teaspoon(3)), Quantity::parse("3 teaspoons").unwrap());
    }

    #[test]
    fn test_parse_quantity_volume_spices() {
        assert_eq!(Quantity::Volume(Volume::Spices(1)), Quantity::parse("1 krm").unwrap());
        assert_eq!(Quantity::Volume(Volume::Spices(2)), Quantity::parse("2 kryddmått").unwrap());
    }

    #[test]
    fn test_parse_quantity_volume_pints() {
        assert_eq!(Quantity::Volume(Volume::Pints(1)), Quantity::parse("1 pint").unwrap());
        assert_eq!(Quantity::Volume(Volume::Pints(2)), Quantity::parse("2 p").unwrap());
        assert_eq!(Quantity::Volume(Volume::Pints(2)), Quantity::parse("2 pt").unwrap());
        assert_eq!(Quantity::Volume(Volume::Pints(3)), Quantity::parse("3 pints").unwrap());
    }

    #[test]
    fn test_parse_quantity_volume_cups() {
        assert_eq!(Quantity::Volume(Volume::Cups(1)), Quantity::parse("1 cup").unwrap());
        assert_eq!(Quantity::Volume(Volume::Cups(2)), Quantity::parse("2 cups").unwrap());
    }

    #[test]
    fn test_parse_quantity_volume_ounces() {
        assert_eq!(Quantity::Volume(Volume::Ounces(1)), Quantity::parse("1 fluid ounce").unwrap());
        assert_eq!(
            Quantity::Volume(Volume::Ounces(2)),
            Quantity::parse("2 fluid ounces").unwrap()
        );
        assert_eq!(Quantity::Volume(Volume::Ounces(3)), Quantity::parse("3 fl oz").unwrap());
    }

    #[test]
    fn test_parse_quantity_weights_kilogram() {
        assert_eq!(Quantity::Weight(Weight::Kilogram(1)), Quantity::parse("1 kilogram").unwrap());
        assert_eq!(Quantity::Weight(Weight::Kilogram(2)), Quantity::parse("2 kg").unwrap());
        assert_eq!(Quantity::Weight(Weight::Kilogram(3)), Quantity::parse("3 kilograms").unwrap());
    }

    #[test]
    fn test_parse_quantity_weights_gram() {
        assert_eq!(Quantity::Weight(Weight::Gram(1)), Quantity::parse("1 gram").unwrap());
        assert_eq!(Quantity::Weight(Weight::Gram(2)), Quantity::parse("2 g").unwrap());
        assert_eq!(Quantity::Weight(Weight::Gram(3)), Quantity::parse("3 grams").unwrap());
    }

    #[test]
    fn test_parse_quantity_weights_pounds() {
        assert_eq!(Quantity::Weight(Weight::Pounds(1)), Quantity::parse("1 pound").unwrap());
        assert_eq!(Quantity::Weight(Weight::Pounds(2)), Quantity::parse("2 lbs").unwrap());
        assert_eq!(Quantity::Weight(Weight::Pounds(3)), Quantity::parse("3 pounds").unwrap());
    }

    #[test]
    fn test_parse_quantity_weights_ounces() {
        assert_eq!(Quantity::Weight(Weight::Ounces(1)), Quantity::parse("1 ounce").unwrap());
        assert_eq!(Quantity::Weight(Weight::Ounces(2)), Quantity::parse("2 ounces").unwrap());
        assert_eq!(Quantity::Weight(Weight::Ounces(3)), Quantity::parse("3 oz").unwrap());
    }

    #[test]
    fn test_parse_quantity_pieces() {
        assert_eq!(Quantity::Pieces(1), Quantity::parse("").unwrap());
        assert_eq!(Quantity::Pieces(2), Quantity::parse("2").unwrap());
        assert!(Quantity::parse("0").is_err());
        assert!(Quantity::parse("-1").is_err());
    }

    #[test]
    fn test_parse_quantity_custom_unit() {
        assert_eq!(
            Quantity::Custom(2, String::from("packages")),
            Quantity::parse("2 packages").unwrap()
        );
    }

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
