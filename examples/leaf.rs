use nannou::{prelude::Key, App};
use plants::utils::app::{make_static_artwork, Artwork, ArtworkOptions, BaseModel, StaticArtwork};
use plants::utils::draw::BrushDrawer;
use plants::utils::geometry::{ProjectionMatrix, WorldBox, WorldPoint, WorldTransform};
use plants::{
  systems::{
    leaf::{leaf_rule, LEAF_AXIOM},
    LSystem,
  },
  turtle,
};
use rand::{distributions::Standard, prelude::Distribution, rngs::StdRng, SeedableRng};
use std::{f64::consts::FRAC_PI_3, usize::MAX};

struct Model {
  base_model: BaseModel,
  steps: usize,
  turtle_params: turtle::polygon::Params,
}

impl Artwork for Model {
  fn new(base_model: BaseModel) -> Self {
    Self {
      base_model,
      steps: 20,
      turtle_params: turtle::polygon::Params::new(FRAC_PI_3),
    }
  }
  fn get_options() -> ArtworkOptions {
    ArtworkOptions {
      ..ArtworkOptions::default()
    }
  }
  fn get_base_model(&self) -> &BaseModel {
    &self.base_model
  }
  fn get_base_model_mut(&mut self) -> &mut BaseModel {
    &mut self.base_model
  }
  fn current_frame_name(&self) -> String {
    format!("frame_{}", self.base_model.seed)
  }
  fn key_pressed(&mut self, _app: &App, key: Key) {
    match key {
      Key::Equals => self.steps += 1,
      Key::Minus => self.steps = (self.steps - 1).clamp(0, MAX),
      _ => {}
    }
  }
}

impl StaticArtwork for Model {
  fn draw(&mut self) {
    let mut rng: StdRng = StdRng::seed_from_u64(self.base_model.seed);
    let draw = &self.base_model.draw;
    draw.background().color(nannou::color::WHITE);

    let [w_w, _] = self.base_model.texture.size();
    let size = w_w as f64;

    let bbox = WorldBox::new(
      WorldPoint::new(-(size / 2.0), -(size / 2.0), 0.0),
      WorldPoint::new(size / 2.0, size / 2.0, 0.0),
    );

    let polygons_3d = grow_l_system(self.steps, &mut rng, &self.turtle_params, bbox);
    let projection = ProjectionMatrix::perspective(1.0);

    let polygons_2d = polygons_3d
      .iter()
      .map(|polygon| {
        polygon
          .iter()
          .filter_map(|point| projection.transform_point3d(*point))
          .map(|point| point.xy())
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();

    polygons_2d.iter().for_each(|polygon| {
      draw
        .polyline()
        .stroke_weight(2.0)
        .brush_from_points(polygon, 7.0, &mut rng)
        .color(nannou::color::BLACK);
    });
  }
}

fn grow_l_system(
  steps: usize,
  rng: &mut StdRng,
  turtle_params: &turtle::polygon::Params,
  bbox_out: WorldBox,
) -> Vec<Vec<WorldPoint>> {
  let mut l_system = LSystem::new(LEAF_AXIOM.to_vec(), leaf_rule, Standard.sample(rng));
  let commands = l_system.nth(steps).unwrap();
  let polygons = turtle::polygon::to_geom(commands, turtle_params);
  let bbox_in = WorldBox::from_points(polygons.iter().flatten());

  let transform = WorldTransform::translation(
    -(bbox_in.min.x + bbox_in.max.x) / 2.0,
    -(bbox_in.min.y + bbox_in.max.y) / 2.0,
    -(bbox_in.min.z + bbox_in.max.z) / 2.0,
  )
  .then_scale(
    bbox_out.width() / bbox_in.width(),
    bbox_out.height() / bbox_in.height(),
    bbox_out.depth() / bbox_in.depth(),
  )
  .then_scale(0.95, 0.95, 0.95);

  polygons
    .into_iter()
    .map(|polygon| {
      polygon
        .into_iter()
        .filter_map(|point| transform.transform_point3d(point))
        .collect()
    })
    .collect()
}
fn main() {
  make_static_artwork::<Model>().run()
}
