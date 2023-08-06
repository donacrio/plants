use std::fmt::Display;

use rand::{distributions::Standard, prelude::Distribution};

use crate::turtle::polygon::{TurtlePolygon, TurtlePolygonInterpretation};

#[derive(Clone, Debug)]
pub enum Leaf {
    Vertex,
    Grow(f64, f64, Option<f64>),
    MainApex(f64, bool),
    SideApex(f64),
    AnglePos,
    AngleNeg,
    Save,
    Load,
    Pile,
    Depile,
}

#[derive(Clone)]
pub struct LeafParameters {
    main_apex_length: f64,
    main_apex_growth_rate: f64,
    side_apex_length: f64,
    side_apex_growth_rate: f64,
    notch_length: f64,
    notch_growth_rate: f64,
    potential_decrement: f64,
}

impl LeafParameters {
    pub fn new(
        main_apex_length: f64,
        main_apex_growth_rate: f64,
        side_apex_length: f64,
        side_apex_growth_rate: f64,
        notch_length: f64,
        notch_growth_rate: f64,
        potential_decrement: f64,
    ) -> Self {
        Self {
            main_apex_length,
            main_apex_growth_rate,
            side_apex_length,
            side_apex_growth_rate,
            notch_length,
            notch_growth_rate,
            potential_decrement,
        }
    }
}

impl Distribution<LeafParameters> for Standard {
    fn sample<R>(&self, rng: &mut R) -> LeafParameters
    where
        R: rand::Rng + ?Sized,
    {
        let main_apex_growth_rate = rng.gen_range(1.0..1.2);
        let side_apex_growth_rate = main_apex_growth_rate * (1.0 + 0.1 * rng.gen::<f64>());
        LeafParameters {
            main_apex_length: 5.0,
            main_apex_growth_rate,
            side_apex_length: rng.gen_range(1.0..3.0),
            side_apex_growth_rate,
            notch_length: rng.gen_range(0.5..1.5),
            notch_growth_rate: rng.gen_range(1.0..1.05),
            potential_decrement: rng.gen_range(0.25..1.0),
        }
    }
}

pub const LEAF_AXIOM: &[Leaf; 12] = &[
    Leaf::Save,
    Leaf::Pile,
    Leaf::MainApex(0.0, true),
    Leaf::Vertex,
    Leaf::Depile,
    Leaf::Load,
    Leaf::Save,
    Leaf::Pile,
    Leaf::MainApex(0.0, false),
    Leaf::Vertex,
    Leaf::Depile,
    Leaf::Load,
];

pub fn leaf_rule(input: Leaf, parameters: &LeafParameters) -> Vec<Leaf> {
    match input {
        Leaf::Vertex => vec![Leaf::Vertex],
        Leaf::Grow(length, growth_rate, time) => match time {
            Some(time) if time > 1.0 => vec![Leaf::Grow(
                length * growth_rate,
                growth_rate,
                Some(time - parameters.potential_decrement),
            )],
            None => vec![Leaf::Grow(length * growth_rate, growth_rate, None)],
            _ => vec![Leaf::Grow(length, growth_rate, time)],
        },
        Leaf::MainApex(time, direction) => match direction {
            true => vec![
                Leaf::Vertex,
                Leaf::Grow(
                    parameters.main_apex_length,
                    parameters.main_apex_growth_rate,
                    None,
                ),
                Leaf::Vertex,
                Leaf::Save,
                Leaf::AnglePos,
                Leaf::SideApex(time),
                Leaf::Grow(
                    parameters.notch_length,
                    parameters.notch_growth_rate,
                    Some(time),
                ),
                Leaf::Vertex,
                Leaf::Depile,
                Leaf::Load,
                Leaf::Save,
                Leaf::AnglePos,
                Leaf::SideApex(time),
                Leaf::Pile,
                Leaf::Vertex,
                Leaf::Load,
                Leaf::MainApex(time + 1.0, direction),
            ],
            false => vec![
                Leaf::Vertex,
                Leaf::Grow(
                    parameters.main_apex_length,
                    parameters.main_apex_growth_rate,
                    None,
                ),
                Leaf::Vertex,
                Leaf::Save,
                Leaf::AngleNeg,
                Leaf::SideApex(time),
                Leaf::Grow(
                    parameters.notch_length,
                    parameters.notch_growth_rate,
                    Some(time),
                ),
                Leaf::Vertex,
                Leaf::Depile,
                Leaf::Load,
                Leaf::Save,
                Leaf::AngleNeg,
                Leaf::SideApex(time),
                Leaf::Pile,
                Leaf::Vertex,
                Leaf::Load,
                Leaf::MainApex(time + 1.0, direction),
            ],
        },
        Leaf::SideApex(time) => {
            if time > 1.0 {
                vec![
                    Leaf::Grow(
                        parameters.side_apex_length,
                        parameters.side_apex_growth_rate,
                        None,
                    ),
                    Leaf::SideApex(time - parameters.potential_decrement),
                ]
            } else {
                vec![Leaf::SideApex(time)]
            }
        }
        Leaf::AnglePos => vec![Leaf::AnglePos],
        Leaf::AngleNeg => vec![Leaf::AngleNeg],
        Leaf::Save => vec![Leaf::Save],
        Leaf::Load => vec![Leaf::Load],
        Leaf::Pile => vec![Leaf::Pile],
        Leaf::Depile => vec![Leaf::Depile],
    }
}

impl Display for Leaf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Leaf::Vertex => write!(f, "."),
            Leaf::Grow(length, growth_rate, time) => match time {
                Some(time) => write!(f, "G({}, {}, {})", length, growth_rate, time),
                None => write!(f, "G({}, {})", length, growth_rate),
            },
            Leaf::MainApex(time, direction) => write!(f, "A({}, {})", time, direction),
            Leaf::SideApex(time) => write!(f, "B({})", time),
            Leaf::AnglePos => write!(f, "+"),
            Leaf::AngleNeg => write!(f, "-"),
            Leaf::Save => write!(f, "["),
            Leaf::Load => write!(f, "]"),
            Leaf::Pile => write!(f, "{{"),
            Leaf::Depile => write!(f, "}}"),
        }
    }
}

impl TurtlePolygonInterpretation for Leaf {
    fn to_turtle(&self) -> TurtlePolygon {
        match *self {
            Leaf::Vertex => TurtlePolygon::Vertex,
            Leaf::Grow(length, _, _) => TurtlePolygon::Forward(length),
            Leaf::MainApex(_, _) => TurtlePolygon::None,
            Leaf::SideApex(_) => TurtlePolygon::None,
            Leaf::AnglePos => TurtlePolygon::Left,
            Leaf::AngleNeg => TurtlePolygon::Right,
            Leaf::Save => TurtlePolygon::Push,
            Leaf::Load => TurtlePolygon::Pop,
            Leaf::Pile => TurtlePolygon::NewPolygon,
            Leaf::Depile => TurtlePolygon::ClosePolygon,
        }
    }
}
