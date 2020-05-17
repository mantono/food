use regex::Regex;

pub struct Ingredient {
    item: String,
    amount: Quantity
}

pub enum Quantity {
    Pieces(u32),
    Weight(Weight),
    Volume(Volume),
    Custom(u32, String)
}

pub enum Weight {
    Kilogram(u32),
    Gram(u32),
    Pounds(u32),
    Ounces(u32)
}

impl Weight {
    fn as_grams(&self) -> u32 {
        match self {
            Weight::Kilogram(w) => 1000 * w,
            Weight::Gram(w) => w,
            Weight::Pounds(w) => ((w as f32) * 28.349523125) as u32,
            Weight::Ounces(w) => ((w as f32) * 453.59237) as u32
        }
    }
}

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
    Ounces(u32)
}

impl Volume {
    fn as_milliliters(&self) -> u32 {
        match self {
            Volume::Liter(v) => v * 1000,
            Volume::Deciliter(v) => v * 100,
            Volume::Centiliter(v) => v * 10,
            Volume::Milliliter(v) | Volume::Spices(v) => v,
            Volume::Tablespoon(v) => v * 15,
            Volume::Teaspoon(v) => v * 5,
            Volume::Pints(v) => v * 473,
            Volume::Ounces(v) => ((v as f32) * 29.6) as u32,
            Volume::Cups(v) => 237 * v
        }.clone()
    }
}

const SPLIT_ON: &str = r"\s+";

fn parse_quantity(&str: amount) -> Result<Quantity, &str> {
    let parts: [&str] = amount.trim().spit(SPLIT_ON);
    let number: u32 =
}

mod tests {
    use crate::recipe::{Quantity, parse_quantity};
    use crate::recipe::Quantity::Volume;

    #[cfg(test)]
    fn test_parse_quantity_volume() {
        assert_eq!(Volume::Liter(1), parse_quantity("1 liter").unwrap());
        assert_eq!(Volume::Liter(2), parse_quantity("2 l").unwrap());
        assert_eq!(Volume::Liter(3), parse_quantity("3 liters").unwrap());

        assert_eq!(Volume::Deciliter(1), parse_quantity("1 deciliter").unwrap());
        assert_eq!(Volume::Deciiliter(2), parse_quantity("2 dl").unwrap());
        assert_eq!(Volume::Deciiliter(3), parse_quantity("3 deciliters").unwrap());

        assert_eq!(Volume::Centiliter(1), parse_quantity("1 centiliter").unwrap());
        assert_eq!(Volume::Centiliter(2), parse_quantity("2 cl").unwrap());
        assert_eq!(Volume::Centiliter(3), parse_quantity("3 centiliters").unwrap());

        assert_eq!(Volume::Milliliter(1), parse_quantity("1 milliliter").unwrap());
        assert_eq!(Volume::Milliliter(2), parse_quantity("2 ml").unwrap());
        assert_eq!(Volume::Milliliter(3), parse_quantity("3 milliliters").unwrap());

        assert_eq!(Volume::Tablespoon(1), parse_quantity("1 tablespoon").unwrap());
        assert_eq!(Volume::Tablespoon(2), parse_quantity("2 tbsp").unwrap());
        assert_eq!(Volume::Tablespoon(2), parse_quantity("2 tb").unwrap());
        assert_eq!(Volume::Tablespoon(2), parse_quantity("2 msk").unwrap());
        assert_eq!(Volume::Tablespoon(3), parse_quantity("3 tablespoons").unwrap());

        assert_eq!(Volume::Teaspoon(1), parse_quantity("1 teaspoon").unwrap());
        assert_eq!(Volume::Teaspoon(2), parse_quantity("2 tsp").unwrap());
        assert_eq!(Volume::Teaspoon(2), parse_quantity("2 tspn").unwrap());
        assert_eq!(Volume::Teaspoon(2), parse_quantity("2 ts").unwrap());
        assert_eq!(Volume::Teaspoon(2), parse_quantity("2 tsk").unwrap());
        assert_eq!(Volume::Teaspoon(3), parse_quantity("3 teaspoons").unwrap());

        assert_eq!(Volume::Pint(1), parse_quantity("1 pint").unwrap());
        assert_eq!(Volume::Pint(2), parse_quantity("2 p").unwrap());
        assert_eq!(Volume::Pint(2), parse_quantity("2 pt").unwrap());
        assert_eq!(Volume::Pint(3), parse_quantity("3 pints").unwrap());

        assert_eq!(Volume::Ounces(1), parse_quantity("1 ounce"))

        assert_eq!(Volume::Cups(1), parse_quantity("1 cup").unwrap());
        assert_eq!(Volume::Cups(2), parse_quantity("2 cups").unwrap());
    }
}