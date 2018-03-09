#[derive(Copy, Clone, Debug)]
pub enum Event {
    Init,

    Resize {
        width: u32,
        height: u32,
    },

    Unknown,
}
