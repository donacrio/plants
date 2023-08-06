use nannou::{prelude::Key, wgpu, window, App, Draw, Frame};
use rand::random;
use std::path::PathBuf;

const TEXTURE_SIZE: [u32; 2] = [2160, 2160];
const RENDER_SIZE: [u32; 2] = [540, 540];

pub trait Artwork {
  fn new(model: BaseModel) -> Self;
  fn get_options() -> ArtworkOptions;
  fn get_base_model(&self) -> &BaseModel;
  fn get_base_model_mut(&mut self) -> &mut BaseModel;
  fn current_frame_name(&self) -> String;
  fn key_pressed(&mut self, app: &App, key: Key);
}

pub struct ArtworkOptions {
  pub texture_size: [u32; 2],
  pub render_size: [u32; 2],
  pub background_path: Option<PathBuf>,
}

impl Default for ArtworkOptions {
  fn default() -> Self {
    Self {
      texture_size: TEXTURE_SIZE,
      render_size: RENDER_SIZE,
      background_path: None,
    }
  }
}

pub struct BaseModel {
  pub window_id: window::Id,
  pub draw: Draw,
  pub texture: wgpu::Texture,
  pub renderer: nannou::draw::Renderer,
  pub texture_capturer: wgpu::TextureCapturer,
  pub texture_reshaper: wgpu::TextureReshaper,
  pub background_texture: Option<wgpu::Texture>,
  pub seed: u64,
  pub recording: bool,
}

fn make_base_model<T: 'static + Artwork>(app: &App, options: ArtworkOptions) -> BaseModel {
  let [win_w, win_h] = options.render_size;
  let window_id = app
    .new_window()
    .size(win_w, win_h)
    .view::<T>(view)
    .key_pressed::<T>(key_pressed)
    .build()
    .unwrap();
  let window = app.window(window_id).unwrap();
  let draw = Draw::new();

  // Retrieve the wgpu device.
  let device = window.device();
  // Create our custom texture.
  let sample_count = window.msaa_samples();

  let texture = wgpu::TextureBuilder::new()
    .size(options.texture_size)
    // Our texture will be used as the RENDER_ATTACHMENT for our `Draw` render pass.
    // It will also be SAMPLED by the `TextureCapturer` and `TextureResizer`.
    .usage(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING)
    // Use nannou's default multisampling sample count.
    .sample_count(sample_count)
    // Use a spacious 16-bit linear sRGBA format suitable for high quality drawing.
    .format(wgpu::TextureFormat::Rgba16Float)
    // Build it!
    .build(device);
  let texture_view = texture.view().build();

  let descriptor = texture.descriptor();
  let renderer =
    nannou::draw::RendererBuilder::new().build_from_texture_descriptor(device, descriptor);

  // Create the texture capturer.
  let texture_capturer = wgpu::TextureCapturer::default();

  // Create the texture reshaper for GUI display
  let texture_reshaper = wgpu::TextureReshaper::new(
    device,
    &texture_view,
    sample_count,
    texture.sample_type(),
    sample_count,
    texture.format(),
  );

  let background_texture = options.background_path.map(|background_path| {
    wgpu::Texture::from_path(&window, images_path(app, background_path)).unwrap()
  });
  let seed = rand::random();
  // Make sure the directory where we will save images to exists.
  std::fs::create_dir_all(&capture_directory(app)).unwrap();
  BaseModel {
    window_id,
    draw,
    texture,
    renderer,
    texture_capturer,
    texture_reshaper,
    background_texture,
    seed,
    recording: false,
  }
}

pub fn make_base_nannou_app<T: 'static + Artwork>() -> nannou::app::Builder<T> {
  nannou::app(model).exit(exit)
}

fn model<T: 'static + Artwork>(app: &App) -> T {
  T::new(make_base_model::<T>(app, T::get_options()))
}

fn view<T: Artwork>(_app: &App, model: &T, frame: Frame) {
  model
    .get_base_model()
    .texture_reshaper
    .encode_render_pass(frame.texture_view(), &mut frame.command_encoder());
}

// Wait for capture to finish.
fn exit<T: Artwork>(app: &App, model: T) {
  let window = app.window(model.get_base_model().window_id).unwrap();
  let device = window.device();
  model
    .get_base_model()
    .texture_capturer
    .await_active_snapshots(device)
    .unwrap();
}

fn key_pressed<T: Artwork>(app: &App, model: &mut T, key: Key) {
  let base_model = model.get_base_model_mut();
  match key {
    Key::R => base_model.recording = !base_model.recording,
    Key::T => {
      let seed = random();
      base_model.seed = seed;
    }
    _ => {}
  }
  model.key_pressed(app, key);
}

fn capture_directory(app: &App) -> std::path::PathBuf {
  app
    .project_path()
    .expect("could not locate project_path")
    .join("out")
    .join(app.exe_name().unwrap())
}

pub fn captured_frame_path(app: &App, name: &str) -> std::path::PathBuf {
  capture_directory(app).join(name).with_extension("png")
}

fn images_path(app: &App, path: PathBuf) -> PathBuf {
  app.assets_path().unwrap().join("images").join(path)
}
