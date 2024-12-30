mod filter_expr;
mod logical;

pub use filter_expr::FilterExpr;
pub use logical::context::Context;
pub use logical::filter::Filter;

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::{filter_expr::FilterExpr, Context};

    #[test]
    fn parse_logical_expression_and_evaluate() {
        let sss = "a >= 300 | b == yes";
        let filter = crate::logical::filter::Filter::from_str(sss).unwrap();

        struct Ctx;
        #[derive(PartialEq, PartialOrd)]
        struct MyBool(bool);

        impl FromStr for MyBool {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.to_lowercase().as_str() {
                    "yes" => Ok(MyBool(true)),
                    "true" => Ok(MyBool(true)),
                    "ok" => Ok(MyBool(true)),
                    _ => Ok(MyBool(false)),
                }
            }
        }
        impl Context for Ctx {
            fn evaluate(&self, key: &str, expression: &str) -> anyhow::Result<bool> {
                match key {
                    "a" => {
                        let hardcoded_a = 300;
                        let f = FilterExpr::<usize>::from_str(expression).unwrap();
                        Ok(f.apply(hardcoded_a))
                    }
                    "b" => {
                        let hardcoded_b = MyBool(true);
                        let f = FilterExpr::<MyBool>::from_str(expression).unwrap();
                        Ok(f.apply(hardcoded_b))
                    }
                    x => {
                        panic!("Unexpected: `{x}`");
                    }
                }
            }
        }
        assert!(filter.evaluate(&Ctx).unwrap());
    }
}
