use crate::Vertex;

pub fn unique(i: usize, vertex: &Vertex, vertices: &[&Vertex]) -> bool {
    let position = vertices.iter().position(|comp| {
        comp.vertex == vertex.vertex
            && comp.normal == vertex.normal
            && comp.tangent == vertex.tangent
            && comp.uv == vertex.uv
            && comp.color == vertex.color
    });

    position.is_none() || position.unwrap() >= i
}

pub fn unique_position(i: usize, vertex: &Vertex, vertices: &[&Vertex]) -> bool {
    let position = vertices
        .iter()
        .position(|comp| comp.vertex == vertex.vertex);

    position.is_none() || position.unwrap() >= i
}
