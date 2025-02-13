#[derive(Eq, PartialEq, Default, Copy, Clone, Debug)]
pub enum BlockType
{
	#[default]
	Air,
	Grass,
	Dirt,
}

impl BlockType {
    pub fn is_solid(&self) -> bool {
        match self {
            BlockType::Air => false,
            BlockType::Grass => true,
            BlockType::Dirt => true,
        }
    }
    pub fn is_air(&self) -> bool {
        !self.is_solid()
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Voxel
{
	pub block_type: BlockType,
    pub texture_position: [i8; 2]
}