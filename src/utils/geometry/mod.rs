use euclid::{Box3D, Point2D, Point3D, Rotation3D, Size2D, Transform3D, Vector2D, Vector3D};

pub struct ScreenSpace;
pub type ScreenVector = Vector2D<f64, ScreenSpace>;
pub type ScreenPoint = Point2D<f64, ScreenSpace>;
pub type ScreenSize = Size2D<f64, ScreenSpace>;
pub struct WorldSpace;
pub type WorldVector = Vector3D<f64, WorldSpace>;
pub type WorldPoint = Point3D<f64, WorldSpace>;
pub type WorldRotation = Rotation3D<f64, WorldSpace, WorldSpace>;
pub type WorldBox = Box3D<f64, WorldSpace>;
pub type WorldTransform = Transform3D<f64, WorldSpace, WorldSpace>;

pub type ProjectionMatrix = Transform3D<f64, WorldSpace, ScreenSpace>;
