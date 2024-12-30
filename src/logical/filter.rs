use anyhow::anyhow;
use bet::BeTree;
use std::str::FromStr;

use super::{bool_operator::BoolOperator, context::Context};

#[derive(Clone, Debug)]
pub struct Filter {
    expr: BeTree<BoolOperator, String>,
}

impl FromStr for Filter {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut expr: BeTree<BoolOperator, String> = BeTree::new();
        for c in input.chars() {
            match c {
                '&' => expr.push_operator(BoolOperator::And),
                '|' => expr.push_operator(BoolOperator::Or),
                '!' => expr.push_operator(BoolOperator::Not),
                ' ' => {}
                '(' => expr.open_par(),
                ')' => expr.close_par(),
                _ => expr.mutate_or_create_atom(String::new).push(c),
            }
        }

        // then we parse each leaf
        let expr = expr.try_map_atoms(|raw| raw.parse())?;

        Ok(Self { expr })
    }
}

impl Filter {
    pub fn evaluate(&self, ctx: &impl Context) -> anyhow::Result<bool> {
        let result = self.expr.eval_faillible(
            |atom| {
                let pos = atom
                    .find(|c| c == '=' || c == '>' || c == '<' || c == '^')
                    .ok_or(anyhow!("No comparison in atom"))?;
                let (var_name, expr) = atom.split_at(pos);
                let var_name = var_name.trim();
                let expr = expr.trim();
                ctx.evaluate(var_name, expr)
            },
            |op, a, b| match (op, b) {
                (BoolOperator::And, Some(b)) => Ok(a && b),
                (BoolOperator::Or, Some(b)) => Ok(a || b),
                (BoolOperator::Not, None) => Ok(!a),
                _ => Err(anyhow!(
                    "expecting a || b, a && b or !b ({op:?} {a:?} {b:?}). Also, this error could be when you don`t use brackets for your expressions"
                )),
            },
            |op, a| {
                matches!(
                    (op, a),
                    (BoolOperator::And, false) | (BoolOperator::Or, true)
                )
            },
        )?;

        Ok(result.is_some_and(|v| v))
    }
}
