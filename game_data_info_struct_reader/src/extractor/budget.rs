use clausewitz_parser::{ClausewitzValue, Val};
use game_data_info_struct::{
    budget::Budget, budget::BudgetComponent, index_mut::IndexMut, resource::ResourceClass,
    resource::ALL_RESOURCES,
};

use crate::Extractor;

pub(crate) struct BudgetExtractor<'a> {
    budget: &'a Val<'a>,
}

impl<'a> Extractor for BudgetExtractor<'a> {
    type Yield = Budget;
    fn extract(&self) -> Self::Yield {
        let current_month_budget = self.budget.get_at_path("current_month").unwrap();

        let get_budget_val = |key: &str, budget_period: &Val| -> BudgetComponent {
            Self::get_budget_component_map(budget_period.get_at_path(key).unwrap())
        };

        Budget {
            income: get_budget_val("income", current_month_budget),
            expense: get_budget_val("expenses", current_month_budget),
        }
    }
}

impl<'a> BudgetExtractor<'a> {
    pub fn create(budget: &'a Val<'a>) -> BudgetExtractor<'a> {
        BudgetExtractor { budget }
    }
    fn get_budget_component_map(component: &Val<'_>) -> BudgetComponent {
        if let Val::Dict(sources) = component {
            sources.into_iter().fold(
                BudgetComponent::default(),
                |mut map, (contributor, contributions)| {
                    let contribitions_per_class = Self::get_contributions_per_class(contributions);

                    for (key, amount) in contribitions_per_class.into_iter() {
                        map.index_mut(&key)
                            .insert(String::from(*contributor), amount);
                    }
                    map
                },
            )
        } else {
            panic!()
        }
    }

    fn get_contributions_per_class(contributions: &Val<'_>) -> Vec<(ResourceClass, f64)> {
        ALL_RESOURCES
            .iter()
            .filter_map(|class| {
                if let Ok(val) = contributions.get_at_path(format!("{}", class).as_str()) {
                    match val {
                        Val::Decimal(d) => Some((class.clone(), *d)),
                        Val::Integer(i) => Some((class.clone(), *i as f64)),
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}
