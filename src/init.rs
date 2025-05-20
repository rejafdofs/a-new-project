use crate::error::りさると;

pub(crate) fn init(path: &str) -> bool {
    match (|| -> りさると<()> {
        crate::ghost_state::STATUS.lock()?.init(path);
        todo!();
    })() {
        Ok(_) => true,
        Err(_) => false,
    }
}
