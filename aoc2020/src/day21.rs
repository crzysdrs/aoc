use crate::Day;
use regex::Regex;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

pub struct Food {
    allergens: Vec<String>,
    ingredients: Vec<String>,
}

fn find_allergens(v: &[Food]) -> Vec<(String, String)> {
    let mut maybe: HashMap<String, HashSet<String>> = HashMap::new();

    v.iter().for_each(|f| {
        let ingred = f.ingredients.iter().cloned().collect::<HashSet<String>>();
        f.allergens.iter().for_each(|a| {
            maybe
                .entry(a.clone())
                .and_modify(|v| {
                    *v = v.intersection(&ingred).cloned().collect();
                })
                .or_insert_with(|| ingred.clone());
        });
    });

    while maybe.iter().any(|(_k, v)| v.len() > 1) {
        let found = maybe
            .iter()
            .filter(|(_k, v)| v.len() == 1)
            .map(|(_k, v)| v.iter().next().unwrap())
            .cloned()
            .collect::<HashSet<String>>();

        maybe
            .iter_mut()
            .filter(|(_k, v)| v.len() > 1)
            .for_each(|(_k, v)| {
                *v = v.difference(&found).cloned().collect();
            });
    }

    maybe
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().next().unwrap()))
        .collect()
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 21;
    type Input = Food;
    type Sol1 = usize;
    type Sol2 = String;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        let split = Regex::new(r"([^)]+) \(contains ([^)]+)\)").unwrap();

        let input = r
            .lines()
            .flatten()
            .map(|l| {
                let cap = split.captures(&l).unwrap();
                let ingredients = cap
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split(' ')
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                let allergens = cap
                    .get(2)
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();

                Food {
                    ingredients,
                    allergens,
                }
            })
            .collect();
        Ok(input)
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        let allergens = find_allergens(v)
            .into_iter()
            .map(|(_k, v)| v)
            .collect::<HashSet<_>>();

        v.iter()
            .flat_map(|f| f.ingredients.iter())
            .filter(|i| !allergens.contains(*i))
            .count()
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        let mut allergens = find_allergens(v);
        allergens.sort_by_key(|(k, _v)| k.clone());
        let allergens = allergens.into_iter().map(|(_k, v)| v).collect::<Vec<_>>();
        allergens.join(",")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let s = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&v), 5);

        assert_eq!(Solution::p2(&v), "mxmxvkd,sqjhc,fvjkl".to_string());
        //unimplemented!()
    }
}
