use std::str::FromStr;

use filter_expr::{Context, FilterExpr};
use serde::Serialize;

#[derive(Serialize)]
pub struct DataItem {
    pub(crate) name: String,
    pub(crate) age: usize,
    pub(crate) weight: f32,
    pub(crate) has_dogs: bool,
}

impl Context for DataItem {
    fn evaluate(&self, key: &str, expression: &str) -> anyhow::Result<bool> {
        match key {
            "name" => {
                let f = FilterExpr::<String>::from_str(expression)?;

                Ok(f.apply(self.name.clone()))
            }
            "age" => {
                let f = FilterExpr::<usize>::from_str(expression)?;

                Ok(f.apply(self.age))
            }
            "weight" => {
                let f = FilterExpr::<f32>::from_str(expression)?;

                Ok(f.apply(self.weight))
            }
            "has-dogs" => {
                let f = FilterExpr::<bool>::from_str(expression)?;

                Ok(f.apply(self.has_dogs))
            }
            _ => Err(anyhow::anyhow!("Cannot filter by {key}")),
        }
    }
}
