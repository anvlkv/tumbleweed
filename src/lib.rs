pub enum TBWMode {
    BW,
    CAM,
    IMG,
}

pub enum TBWMessage {
    Start(f32, f32),
    Resize(f32, f32),
    Mode(TBWMode),
    Capture,
    Cursor(f32, f32),
    Brush(f32),
}

impl From<(f32, f32, f32)> for TBWMessage {
    fn from((op, val1, val2): (f32, f32, f32)) -> Self {
        match op {
            0.1 => Self::Start(val1, val2),
            0.2 => Self::Resize(val1, val2),
            0.3 => Self::Mode(match val1 {
                0.1 => TBWMode::BW,
                0.2 => TBWMode::CAM,
                0.3 => TBWMode::IMG,
                _ => panic!(),
            }),
            0.4 => Self::Capture,
            0.5 => Self::Cursor(val1, val2),
            0.6 => Self::Brush(val1),
            _ => panic!(),
        }
    }
}

pub struct Tumbleweed {
    world: pix::Raster<pix::rgb::Rgba8p>,
    points: Vec<pointy::Pt<f32>>,
}

impl Tumbleweed {
    pub fn new() -> Self {
        Self {
            world: pix::Raster::<pix::rgb::Rgba8p>::with_clear(0, 0),
            points: vec![],
        }
    }

    pub fn handle_message(&mut self, msg: TBWMessage) {
      use pix::{Raster, rgb::Rgba8p};
      use pointy::Pt;
        match msg {
            TBWMessage::Start(width, height) => {
              self.world = pix::Raster::<pix::rgb::Rgba8p>::with_clear(width as u32, height as u32);
              self.points = vec![
                Pt::new(width/2.0, height/2.0)
              ];
            }
            TBWMessage::Resize(width, height) => {}
            TBWMessage::Mode(mode) => {}
            TBWMessage::Capture => {}
            TBWMessage::Cursor(x, y) => {
              self.points.push(Pt::new(x, y));
            }
            TBWMessage::Brush(w) => {}
        }
        self.imprint();
    }

    pub fn get_bytes(&self) -> Vec<u8> {
      Vec::from(self.world.as_u8_slice())
    }

    fn imprint(&mut self) {
      use footile::{Plotter, PathOp};
      use pointy::Pt;
      use pix::rgb::{Rgba8p};

      let midpoint = Pt::new(self.world.width() as f32/2.0, self.world.height() as f32/2.0);
      let mut plot = Plotter::new(self.world.clone());
      let mut pts = self.points.iter().peekable();
      let mut ops: Vec<PathOp> = vec![
        PathOp::PenWidth(2.0),
        PathOp::Move(midpoint)
      ];
      while let Some(point) = pts.next() {
        let next_pt = pts.peek();
        if let Some(next) = next_pt {
          ops.push(PathOp::Quad(*point, **next))
        }
        else {
          ops.push(PathOp::Close())
        }
      }
      self.world = plot.stroke(ops, Rgba8p::new(0, 0, 0, 255)).clone()
    }
}
