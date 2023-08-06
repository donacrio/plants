pub mod leaf;

pub struct LSystem<T, F, P>
where
  F: FnMut(T, &P) -> Vec<T>,
{
  axiom: Vec<T>,
  rules: F,
  constants: P,
}

impl<T, F, P> LSystem<T, F, P>
where
  F: FnMut(T, &P) -> Vec<T>,
{
  pub fn new(axiom: Vec<T>, rules: F, constants: P) -> LSystem<T, F, P> {
    LSystem {
      axiom,
      rules,
      constants,
    }
  }
}

impl<T, F, P> Iterator for LSystem<T, F, P>
where
  T: Clone,
  F: FnMut(T, &P) -> Vec<T>,
{
  type Item = Vec<T>;

  fn next(&mut self) -> Option<Vec<T>> {
    let result = self.axiom.clone();
    let mut new_axiom = Vec::new();
    for element in self.axiom.drain(..) {
      new_axiom.extend((self.rules)(element, &self.constants).into_iter());
    }
    self.axiom = new_axiom;
    Some(result)
  }
}
