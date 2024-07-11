use crate::components::*;
use crate::configs;
use specs::{Builder, World, WorldExt};

pub fn cell_grid(world: &mut World) {
    let x_min = -configs::GRID_SIZE_HALF[0];
    let x_max = configs::GRID_SIZE_HALF[0] + 1;
    let y_min = -configs::GRID_SIZE_HALF[1];
    let y_max = configs::GRID_SIZE_HALF[1] + 1;

    let mut entity_map = vec![];
    let mut index = 0;
    for x in x_min..x_max {
        for y in y_min..y_max {
            let entity = world
                .create_entity()
                .with(Tile {
                    uv: [0.0, 1.0, 0.0, 1.0],
                    atlas: "agent".to_string(),
                })
                .with(Transform {
                    position: [x as f32, y as f32, 0.0],
                    // position: [0.,0.,0.],
                    size: [0.85, 0.85],
                    rotation: 0.,
                })
                .with(Cell {
                    index,
                    alive: false,
                    next: false,
                }) // Initially all cells are off
                .build();
            index += 1;
            entity_map.push(entity);
        }
    }

    world.insert(entity_map);
}

pub fn set_cells_alive_at_positions(world: &mut World, positions_to_set_alive: Vec<[f32; 2]>) {
    use specs::Join;

    let entities = world.entities();
    let positions = world.read_storage::<Transform>();
    let mut cells = world.write_storage::<Cell>();

    // 먼저 필요한 위치의 엔티티를 찾습니다.
    let entities_to_set_alive: Vec<_> = (&entities, &positions)
        .join()
        .filter_map(|(entity, pos)| {
            if positions_to_set_alive
                .iter()
                .any(|&p| p == [pos.position[0], pos.position[1]])
            {
                Some(entity)
            } else {
                None
            }
        })
        .collect();

    // 이제 해당 엔티티의 Cell 컴포넌트를 업데이트합니다.
    for entity in entities_to_set_alive {
        if let Some(cell) = cells.get_mut(entity) {
            cell.alive = true;
        }
    }
}
