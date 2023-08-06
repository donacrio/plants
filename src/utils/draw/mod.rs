use super::geometry::{ScreenPoint, ScreenVector};
use nannou::{
  draw::{
    primitive::{Path, PathStroke},
    Drawing,
  },
  geom,
};
use rand::{rngs::StdRng, Rng};
use std::{f64::consts::PI, ops::Add};

pub trait StrokeDrawer<'a> {
  fn stroke_from_points(self, points: &[ScreenPoint]) -> Drawing<'a, Path>;
}

impl<'a> StrokeDrawer<'a> for Drawing<'a, PathStroke> {
  fn stroke_from_points(self, points: &[ScreenPoint]) -> Drawing<'a, Path> {
    self.points(
      points
        .iter()
        .map(|p| geom::Point2::new(p.x as f32, p.y as f32))
        .collect::<Vec<geom::Point2>>(),
    )
  }
}

pub trait BrushDrawer<'a> {
  fn brush_from_points(
    self,
    points: &[ScreenPoint],
    radius: f64,
    rng: &mut StdRng,
  ) -> Drawing<'a, Path>;
}

impl<'a> BrushDrawer<'a> for Drawing<'a, PathStroke> {
  fn brush_from_points(
    self,
    points: &[ScreenPoint],
    radius: f64,
    rng: &mut StdRng,
  ) -> Drawing<'a, Path> {
    self.points((0..radius as usize).flat_map(|_| {
      points
        .iter()
        .map(|p| {
          let r = radius * rng.gen::<f64>().sqrt();
          let theta = 2.0 * PI * rng.gen::<f64>().sqrt();
          p.add(ScreenVector::new(theta.cos(), theta.sin()) * r)
        })
        .map(|p| geom::Point2::new(p.x as f32, p.y as f32))
        .collect::<Vec<geom::Point2>>()
    }))
  }
}
