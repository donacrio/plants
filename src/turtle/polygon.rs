use crate::utils::geometry::{WorldPoint, WorldRotation, WorldVector};
use euclid::Angle;
use std::{collections::VecDeque, f64::consts::FRAC_PI_4, fmt::Debug};

pub trait TurtlePolygonInterpretation {
    fn to_turtle(&self) -> TurtlePolygon;
}
pub enum TurtlePolygon {
    Vertex,
    Forward(f64),
    Left,
    Right,
    Push,
    Pop,
    NewPolygon,
    ClosePolygon,
    None,
}

pub struct Params {
    pub angle: f64,
}

impl Params {
    pub fn new(angle: f64) -> Self {
        Self { angle }
    }
}

pub fn to_geom<T: TurtlePolygonInterpretation + Debug>(
    commands: Vec<T>,
    params: &Params,
) -> Vec<Vec<WorldPoint>> {
    let mut polygons = vec![];

    let mut position = WorldVector::zero();
    let mut rotation = WorldRotation::around_z(Angle::radians(FRAC_PI_4));
    let mut states = VecDeque::new();
    let mut points = vec![];
    let mut saved_points = VecDeque::new();
    for command in commands.iter() {
        match command.to_turtle() {
            TurtlePolygon::Vertex => points.push(position.to_point()),
            TurtlePolygon::Forward(length) => {
                position += rotation.transform_vector3d(WorldVector::one()) * length
            }
            TurtlePolygon::Left => {
                rotation = rotation.then(&WorldRotation::around_z(Angle::radians(params.angle)));
            }
            TurtlePolygon::Right => {
                rotation = rotation.then(&WorldRotation::around_z(Angle::radians(-params.angle)));
            }
            TurtlePolygon::Push => states.push_back((position, rotation)),
            TurtlePolygon::Pop => (position, rotation) = states.pop_back().unwrap(),
            TurtlePolygon::NewPolygon => {
                saved_points.push_back(points);
                points = vec![];
            }
            TurtlePolygon::ClosePolygon => {
                polygons.push(points);
                points = saved_points.pop_back().unwrap();
            }
            TurtlePolygon::None => {}
        }
    }
    polygons
}
