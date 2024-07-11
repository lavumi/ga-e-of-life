pub use dispatcher::UnifiedDispatcher;
pub use reset_input::ResetInputDelta;
pub use update_camera::UpdateCamera;
pub use update_cells::UpdateCells;

mod dispatcher;
mod reset_input;

mod update_camera;
mod update_cells;

pub fn build() -> Box<dyn UnifiedDispatcher + 'static> {
    dispatcher::new()
}
