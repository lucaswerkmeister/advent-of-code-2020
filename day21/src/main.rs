use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Ingredient(String);

fn i(s: &str) -> Ingredient {
    Ingredient(s.to_owned())
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Allergen(String);

fn a(s: &str) -> Allergen {
    Allergen(s.to_owned())
}

fn parse_input(
    input: &str,
) -> (
    HashMap<Allergen, Vec<HashSet<Ingredient>>>,
    Vec<HashSet<Ingredient>>,
) {
    let mut ingredient_lists_with_allergen: HashMap<Allergen, Vec<HashSet<Ingredient>>> =
        HashMap::new();
    let mut ingredient_lists: Vec<HashSet<Ingredient>> = Vec::new();
    for line in input.lines() {
        let mut halves = line.split(" (contains ");
        let ingredients = halves
            .next()
            .unwrap()
            .split(' ')
            .map(i)
            .collect::<HashSet<_>>();
        ingredient_lists.push(ingredients.clone());
        let allergens = halves
            .next()
            .unwrap()
            .trim_end_matches(')')
            .split(", ")
            .map(a)
            .collect::<Vec<_>>();
        assert!(halves.next().is_none());
        for allergen in allergens {
            ingredient_lists_with_allergen
                .entry(allergen)
                .or_default()
                .push(ingredients.clone());
        }
    }
    (ingredient_lists_with_allergen, ingredient_lists)
}

fn determine_allergens(
    mut ingredient_lists_with_allergen: HashMap<Allergen, Vec<HashSet<Ingredient>>>,
) -> HashMap<Ingredient, Allergen> {
    let mut allergen_of_ingredient: HashMap<Ingredient, Allergen> = HashMap::new();
    let mut last_resolved_ingredient = None;
    while !ingredient_lists_with_allergen.is_empty() {
        'outer: for (allergen, ingredient_lists) in &ingredient_lists_with_allergen {
            let mut ingredient_intersection = HashSet::new();
            let mut ingredient_lists = ingredient_lists.iter();
            ingredient_lists
                .next()
                .unwrap()
                .iter()
                .for_each(|ingredient| {
                    ingredient_intersection.insert(ingredient.clone());
                });
            for ingredient_list in ingredient_lists {
                if ingredient_list.len() == 1 {
                    let ingredient = ingredient_list.iter().next().unwrap();
                    allergen_of_ingredient.insert(ingredient.clone(), allergen.clone());
                    last_resolved_ingredient = Some(ingredient.clone());
                    break 'outer;
                }
                ingredient_intersection = ingredient_intersection
                    .intersection(ingredient_list)
                    .cloned()
                    .collect();
            }
            let mut ingredient_intersection = ingredient_intersection.into_iter();
            let ingredient = ingredient_intersection
                .next()
                .expect("at least one possible ingredient for allergen");
            if ingredient_intersection.next().is_none() {
                allergen_of_ingredient.insert(ingredient.clone(), allergen.clone());
                last_resolved_ingredient = Some(ingredient.clone());
                break 'outer;
            }
        }
        if let Some(ingredient) = last_resolved_ingredient {
            last_resolved_ingredient = None;
            ingredient_lists_with_allergen.remove(&allergen_of_ingredient[&ingredient]);
            for (_, ingredient_lists) in ingredient_lists_with_allergen.iter_mut() {
                for ingredient_list in ingredient_lists {
                    ingredient_list.remove(&ingredient);
                }
            }
        } else {
            panic!("unable to determine any allergen");
        }
    }
    allergen_of_ingredient
}

fn part1(
    ingredient_lists: Vec<HashSet<Ingredient>>,
    allergens: HashMap<Ingredient, Allergen>,
) -> usize {
    ingredient_lists
        .iter()
        .flatten()
        .filter(|&ingredient| !allergens.contains_key(ingredient))
        .count()
}

fn part2(allergens: HashMap<Ingredient, Allergen>) -> String {
    let mut ingredients = allergens.keys().collect::<Vec<_>>();
    ingredients
        .sort_by(|&ingredient1, &ingredient2| allergens[ingredient1].cmp(&allergens[ingredient2]));
    ingredients
        .into_iter()
        .map(|ingredient| ingredient.0.clone())
        .collect::<Vec<_>>()
        .join(",")
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    let (ingredient_lists_with_allergen, ingredient_lists) = parse_input(&input);
    let allergens = determine_allergens(ingredient_lists_with_allergen);
    println!("{}", part1(ingredient_lists, allergens.clone()));
    println!("{}", part2(allergens));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    macro_rules! set {
        ( $( $x:expr ),* ) => {
            vec![ $( $x, )* ].into_iter().collect::<HashSet<_>>()
        };
    }

    #[test]
    fn test_parse_input() {
        let input = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";
        let mut ingredient_lists_with_allergen: HashMap<Allergen, Vec<HashSet<Ingredient>>> =
            HashMap::new();
        ingredient_lists_with_allergen.insert(
            a("dairy"),
            vec![
                set![i("mxmxvkd"), i("kfcds"), i("sqjhc"), i("nhms")],
                set![i("trh"), i("fvjkl"), i("sbzzf"), i("mxmxvkd")],
            ],
        );
        ingredient_lists_with_allergen.insert(
            a("fish"),
            vec![
                set![i("mxmxvkd"), i("kfcds"), i("sqjhc"), i("nhms")],
                set![i("sqjhc"), i("mxmxvkd"), i("sbzzf")],
            ],
        );
        ingredient_lists_with_allergen.insert(a("soy"), vec![set![i("sqjhc"), i("fvjkl")]]);
        let ingredient_lists = vec![
            set![i("mxmxvkd"), i("kfcds"), i("sqjhc"), i("nhms")],
            set![i("trh"), i("fvjkl"), i("sbzzf"), i("mxmxvkd")],
            set![i("sqjhc"), i("fvjkl")],
            set![i("sqjhc"), i("mxmxvkd"), i("sbzzf")],
        ];
        assert_eq!(
            (ingredient_lists_with_allergen, ingredient_lists),
            parse_input(input)
        );
    }

    #[test]
    fn test_determine_allergens() {
        let mut ingredient_lists_with_allergen: HashMap<Allergen, Vec<HashSet<Ingredient>>> =
            HashMap::new();
        ingredient_lists_with_allergen.insert(
            a("dairy"),
            vec![
                set![i("mxmxvkd"), i("kfcds"), i("sqjhc"), i("nhms")],
                set![i("trh"), i("fvjkl"), i("sbzzf"), i("mxmxvkd")],
            ],
        );
        ingredient_lists_with_allergen.insert(
            a("fish"),
            vec![
                set![i("mxmxvkd"), i("kfcds"), i("sqjhc"), i("nhms")],
                set![i("sqjhc"), i("mxmxvkd"), i("sbzzf")],
            ],
        );
        ingredient_lists_with_allergen.insert(a("soy"), vec![set![i("sqjhc"), i("fvjkl")]]);
        let mut expected: HashMap<Ingredient, Allergen> = HashMap::new();
        expected.insert(i("mxmxvkd"), a("dairy"));
        expected.insert(i("sqjhc"), a("fish"));
        expected.insert(i("fvjkl"), a("soy"));
        assert_eq!(
            expected,
            determine_allergens(ingredient_lists_with_allergen)
        );
    }

    #[test]
    fn test_part1() {
        let ingredient_lists = vec![
            set![i("mxmxvkd"), i("kfcds"), i("sqjhc"), i("nhms")],
            set![i("trh"), i("fvjkl"), i("sbzzf"), i("mxmxvkd")],
            set![i("sqjhc"), i("fvjkl")],
            set![i("sqjhc"), i("mxmxvkd"), i("sbzzf")],
        ];
        let mut allergens: HashMap<Ingredient, Allergen> = HashMap::new();
        allergens.insert(i("mxmxvkd"), a("dairy"));
        allergens.insert(i("sqjhc"), a("fish"));
        allergens.insert(i("fvjkl"), a("soy"));
        assert_eq!(5, part1(ingredient_lists, allergens));
    }

    #[test]
    fn test_part2() {
        let mut allergens: HashMap<Ingredient, Allergen> = HashMap::new();
        allergens.insert(i("mxmxvkd"), a("dairy"));
        allergens.insert(i("sqjhc"), a("fish"));
        allergens.insert(i("fvjkl"), a("soy"));
        assert_eq!("mxmxvkd,sqjhc,fvjkl", part2(allergens));
    }
}
