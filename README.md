`filter-expr` is a library for creating command line utils with filtering
capabilities. 

It allows you to make filtering, using comparison and matching:

```sh
$ your-programm delete-items --where "type=person & age > 40"
$ your-programm list-items --where "(weight >= 25.6 & weight < 30) | density < 0.01"
$ your-programm copy-items --where "!(weight >= 25.6 & weight < 30) | density < 0.01"
```

This library is very basic, and relies on `bet` crate for logical expression
parsing. Don't forget to use parenthesis when there are more than 2 conditions
in your filter.

# How to use:

Create command line parsed structure, like: 

```rust 
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(long="where")]
    pub(crate) filter: Option<filter_expr::Filter>,
}

```

Then, `Filter` could be applied for some context,  which is literally some data
item, you want to check, if you want to process it or not.

```rust
use std::str::FromStr;

use filter_expr::{Context, FilterExpr};

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
```

 * _key_ - is variable item in filter expression, could have relation to data
 structure field.
 * _expression_ - is something like "> 5", "= 12", "<= 8.8". Operator with
 value.


Then it is possible to use it like:

Filter by name.
```bash
$ cargo run --example simple-filter-cli -- --where "\!(name=Jeff)" 
[
  {
    "name": "Mike",
    "age": 40,
    "weight": 86.0,
    "has_dogs": false
  },
  {
    "name": "Caren",
    "age": 35,
    "weight": 96.0,
    "has_dogs": true
  },
  {
    "name": "Chelsea",
    "age": 20,
    "weight": 55.0,
    "has_dogs": false
  },
  {
    "name": "Moa",
    "age": 13,
    "weight": 40.0,
    "has_dogs": false
  }
]
```

Filter by age
```sh
$ cargo run --example simple-filter-cli -- --where "age > 30"
[
  {
    "name": "Mike",
    "age": 40,
    "weight": 86.0,
    "has_dogs": false
  },
  {
    "name": "Caren",
    "age": 35,
    "weight": 96.0,
    "has_dogs": true
  }
]
```

Filter by age and weight
```sh
$ cargo run --example simple-filter-cli -- --where "age < 20 & weight <47.999"
[
  {
    "name": "Moa",
    "age": 13,
    "weight": 40.0,
    "has_dogs": false
  }
]
```

Filter something strange
```sh
$ cargo run --example simple-filter-cli -- --where "(has-dogs=false | age = 35) & weight > 40"
[
  {
    "name": "Mike",
    "age": 40,
    "weight": 86.0,
    "has_dogs": false
  },
  {
    "name": "Caren",
    "age": 35,
    "weight": 96.0,
    "has_dogs": true
  },
  {
    "name": "Chelsea",
    "age": 20,
    "weight": 55.0,
    "has_dogs": false
  }
]
```
