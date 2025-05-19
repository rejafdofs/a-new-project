use crate::error::りさると;

pub(crate) fn init(str: &str) -> bool {
    match (|| -> りさると<()> {
        crate::ghost_state::GHOSTSTATE.lock()?.init();
        todo!();
    })() {
        Ok(_) => true,
        Err(_) => false,
    }
}
