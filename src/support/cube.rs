use crate::support::rendering_info::*;

pub const VERTICES: [Vertex; 9] = [
    Vertex {
        position: (0.0, 0.0, 0.0),
    }, // dummy vector because in the original model indices
    Vertex {
        position: (0.0, 0.0, 0.0),
    },
    Vertex {
        position: (1.0, 0.0, 0.0),
    },
    Vertex {
        position: (1.0, 1.0, 0.0),
    },
    Vertex {
        position: (1.0, 1.0, 1.0),
    },
    Vertex {
        position: (0.0, 0.0, 1.0),
    },
    Vertex {
        position: (0.0, 1.0, 0.0),
    },
    Vertex {
        position: (0.0, 1.0, 1.0),
    },
    Vertex {
        position: (1.0, 0.0, 1.0),
    },
];

pub const NORMALS: [Normal; 9] = [
    Normal {
        normal: (0.0, 0.0, 0.0),
    }, // dummy vector because in the original model indices
    // start at 1
    Normal {
        normal: (0.0, 0.0, 0.0),
    },
    Normal {
        normal: (1.0, 0.0, 0.0),
    },
    Normal {
        normal: (1.0, 1.0, 0.0),
    },
    Normal {
        normal: (1.0, 1.0, 1.0),
    },
    Normal {
        normal: (0.0, 0.0, 1.0),
    },
    Normal {
        normal: (0.0, 1.0, 0.0),
    },
    Normal {
        normal: (0.0, 1.0, 1.0),
    },
    Normal {
        normal: (1.0, 0.0, 1.0),
    },
];

pub const INDICES: [u16; 36] = [
    0, 2, 1, 0, 3, 2, 1, 2, 6, 6, 5, 1, 4, 5, 6, 6, 7, 4, 2, 3, 6, 6, 3, 7, 0, 7, 3, 0, 4, 7, 0, 1,
    5, 0, 5, 4,
];
