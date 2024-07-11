use specs::{Entities, Entity, Join, Read, System, Write, WriteStorage};

use crate::components::Cell;
use crate::resources::StageTick;

pub struct UpdateCells;

impl<'a> System<'a> for UpdateCells {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Cell>,
        Write<'a, StageTick>,
        Read<'a, Vec<Entity>>,
    );

    fn run(&mut self, (entities, mut cells, mut tick, entity_map): Self::SystemData) {
        if tick.current_spent < tick.stage_tick {
            return;
        }

        tick.current_spent = 0.0;

        //fix fxx hard code
        let width = 77;
        let height = 59;
        let total_cells = width * height;

        let mut next_states = vec![false; total_cells];

        for cell in (&cells).join() {
            let index = cell.index;

            let neighbors = [
                index.wrapping_sub((height + 1) as u32), // 상좌
                index.wrapping_sub(height as u32),       // 상
                index.wrapping_sub((height - 1) as u32), // 상우
                index.wrapping_sub(1),                   // 좌
                index.wrapping_add(1),                   // 우
                index.wrapping_add((height - 1) as u32), // 하좌
                index.wrapping_add(height as u32),       // 하
                index.wrapping_add((height + 1) as u32), // 하우
            ];

            let mut alive_count = 0;

            for &neighbor_index in &neighbors {
                if neighbor_index < total_cells as u32 {
                    if let Some(neighbor_entity) = entity_map.get(neighbor_index as usize).copied()
                    {
                        if let Some(neighbor) = cells.get(neighbor_entity) {
                            if neighbor.alive {
                                alive_count += 1;
                            }
                        }
                    }
                }
            }

            // 예시 규칙으로 셀의 next 상태를 저장합니다.
            next_states[index as usize] = if alive_count == 3 {
                true
            } else if !(2..=3).contains(&alive_count) {
                false
            } else {
                cell.alive // 유지
            };
        }

        // 두 번째 루프: next 상태를 기반으로 alive 상태를 업데이트합니다.
        for (_entity, cell) in (&entities, &mut cells).join() {
            cell.alive = next_states[cell.index as usize];
        }
    }
}
