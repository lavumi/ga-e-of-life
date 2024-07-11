use crate::resources::InputHandler;
use specs::{System, Write};

//todo 이 시스템이 과연 필요할까 고민좀
//현재 handle input이 변경때만 불러지고 변경이 없으면 안불러져서 강제로 지워줘야함
pub struct ResetInputDelta;

impl<'a> System<'a> for ResetInputDelta {
    type SystemData = Write<'a, InputHandler>;

    fn run(&mut self, mut input: Self::SystemData) {
        input.reset_delta();
    }
}
