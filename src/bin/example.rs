#[cfg(feature = "example")]
extern crate glutin;
extern crate dungeonbuilder;
#[cfg(feature = "example")]
extern crate libc;
#[cfg(feature = "example")]
extern crate gl;
#[macro_use]
#[cfg(feature = "example")]
extern crate glium;
#[cfg(feature = "example")]
extern crate cgmath;

#[cfg(feature = "example")]
use dungeonbuilder::dungeonbuilder::DungeonBuilder;
#[cfg(feature = "example")]
use dungeonbuilder::point::Point;
#[cfg(feature = "example")]
use dungeonbuilder::dimensionoptions::DimensionOptions;
#[cfg(feature = "example")]
use dungeonbuilder::hallwayoptions::HallwayOptions;

#[cfg(feature = "example")]
use cgmath::FixedArray;

#[cfg(not(feature = "example"))]
#[allow(dead_code)]
fn main() {
    println!("This example requires the `example` feature to be enabled");
}

#[cfg(feature = "example")]
fn main() {
    use glium::{DisplayBuild, Surface};
    let dungeon = DungeonBuilder::new()
        .in_area(Point::new(0f32,0f32), Point::new(3000f32, 3000f32))
        .with_dimension_options(DimensionOptions::new_with_max(100f32, 100f32, 1000f32, 3000f32))
        .with_hallway_options(HallwayOptions::new(15.0f32, 500f32, 5f32, 20f32))
        .build();

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        color: [f32; 3],
    }

    let mut lines : Vec<Vertex> = Vec::new();
    for room in dungeon.rooms().iter() {
        lines.push(Vertex { position: [room.lower_left().x(), room.lower_left().y()], color: [1.0f32, 0.0f32, 0.0f32]});
        lines.push(Vertex { position: [room.lower_left().x(), room.upper_right().y()], color: [1.0f32, 0.0f32, 0.0f32]});
        lines.push(Vertex { position: [room.lower_left().x(), room.upper_right().y()], color: [1.0f32, 0.0f32, 0.0f32]});
        lines.push(Vertex { position: [room.upper_right().x(), room.upper_right().y()], color: [1.0f32, 0.0f32, 0.0f32]});
        lines.push(Vertex { position: [room.upper_right().x(), room.upper_right().y()], color: [1.0f32, 0.0f32, 0.0f32]});
        lines.push(Vertex { position: [room.upper_right().x(), room.lower_left().y()], color: [1.0f32, 0.0f32, 0.0f32]});
        lines.push(Vertex { position: [room.upper_right().x(), room.lower_left().y()], color: [1.0f32, 0.0f32, 0.0f32]});
        lines.push(Vertex { position: [room.lower_left().x(), room.lower_left().y()], color: [1.0f32, 0.0f32, 0.0f32]});
    }

    for hallway in dungeon.hallways().iter() {
        lines.push(Vertex { position: [hallway.lower_left().x(), hallway.lower_left().y()], color: [0.0f32, 1.0f32, 0.0f32]});
        lines.push(Vertex { position: [hallway.lower_left().x(), hallway.upper_right().y()], color: [0.0f32, 1.0f32, 0.0f32]});
        lines.push(Vertex { position: [hallway.lower_left().x(), hallway.upper_right().y()], color: [0.0f32, 1.0f32, 0.0f32]});
        lines.push(Vertex { position: [hallway.upper_right().x(), hallway.upper_right().y()], color: [0.0f32, 1.0f32, 0.0f32]});
        lines.push(Vertex { position: [hallway.upper_right().x(), hallway.upper_right().y()], color: [0.0f32, 1.0f32, 0.0f32]});
        lines.push(Vertex { position: [hallway.upper_right().x(), hallway.lower_left().y()], color: [0.0f32, 1.0f32, 0.0f32]});
        lines.push(Vertex { position: [hallway.upper_right().x(), hallway.lower_left().y()], color: [0.0f32, 1.0f32, 0.0f32]});
        lines.push(Vertex { position: [hallway.lower_left().x(), hallway.lower_left().y()], color: [0.0f32, 1.0f32, 0.0f32]});
    }


    implement_vertex!(Vertex, position, color);

    let vertex_buffer = glium::VertexBuffer::new(&display, lines);
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

    let vertex_shader_src = r#"
        #version 140
        uniform mat4 matrix;
        in vec2 position;
        in vec3 color;
        out vec3 vColor;
        void main() {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
            vColor = color;
        }"#;

        let fragment_shader_src = r#"
        #version 140
        in vec3 vColor;
        out vec4 color;
        void main() {
            color = vec4(vColor, 1.0); 
        }"#;
        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
        let ortho_matrix: cgmath::Matrix4<f32> = cgmath::ortho(-100.0, 3100.0, -100.0, 3100.0, -1.0, 1.0);
        let fixed_ortho_matrix = ortho_matrix.as_fixed();
        let uniforms = uniform! {
            matrix: *fixed_ortho_matrix,
        };

        loop {
            let mut target = display.draw();
            target.clear_color(0.1, 0.1, 0.1, 1.0);
            target.draw(&vertex_buffer, &indices, &program, &uniforms,
                        &Default::default()).unwrap();
            target.finish();

            if display.is_closed() {
                break;
            }
        }
}
