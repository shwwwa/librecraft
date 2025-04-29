
/// Specifies which position in the block this face occupies
pub enum FaceDirection(u8) {
    Bottom = 0,
    Top = 1,
    North = 2,
    South = 3,
    West = 4,
    East = 5,
}

pub struct Face {
    pub direction: FaceDirection,
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub normals: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 4]>,
    pub uvs: Vec<[f32; 2]>,
    pub texture: String,
}

pub struct BlockShape {
    pub faces: Vec<Face>,
}

pub fn full_cube(block: &BlockData) -> Self {

}
