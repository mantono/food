use regex::Regex;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Ingredient {
    item: String,
    amount: Quantity,
}

impl Ingredient {
    pub fn parse(input: &str) -> Result<Ingredient, String> {
        let parts: Vec<&str> = input.trim().split(',').collect();
        match parts.len() {
            0 => panic!("Invalid line '{}", input),
            1 => {
                let ingr = Ingredient {
                    item: parts.first().unwrap().to_string(),
                    amount: Quantity::Pieces(1),
                };
                Ok(ingr)
            }
            2 => {
                let ingr = Ingredient {
                    item: parts.first().unwrap().to_string(),
                    amount: Quantity::parse(parts[1])?,
                };
                Ok(ingr)
            }
            _ => panic!("Invalid line '{}", input),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Quantity {
    Pieces(u32),
    Weight(Weight),
    Volume(Volume),
    Custom(u32, String),
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
            Err(e) => {
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

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Weight {
    Kilogram(u32),
    Gram(u32),
    Pounds(u32),
    Ounces(u32),
}

impl Weight {
    fn as_grams(&self) -> u32 {
        match self {
            Weight::Kilogram(w) => 1000 * w,
            Weight::Gram(w) => *w,
            Weight::Pounds(w) => ((*w as f32) * 453.59237) as u32,
            Weight::Ounces(w) => ((*w as f32) * 28.349523125) as u32,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
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
    fn as_milliliters(&self) -> u32 {
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

#[cfg(test)]
mod tests {
    use crate::recipe::Volume;
    use crate::recipe::Weight;
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
}
