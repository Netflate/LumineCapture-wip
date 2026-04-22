

#[derive(Debug, Clone)]

pub enum SourceType {
    Monitor = 1,
    Window = 2,
    Virtual = 4,
}
pub struct CapturedFrame {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub struct StreamInfo {
    pub node_id: u32,
    pub size: Option<(i32, i32)>,
    pub position: Option<(i32, i32)>,
    pub source_type: SourceType,
}



pub struct CaptureResult {
    pub frame: CapturedFrame,
    pub streams: Vec<StreamInfo>,
}

