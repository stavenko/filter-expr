pub trait Context {
    fn evaluate(&self, key: &str, expression: &str) -> anyhow::Result<bool>;
}
