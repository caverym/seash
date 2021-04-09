#![allow(unused)] // NOTE(Able): This should be avoided in the first release to keep source clean

pub enum Action {
    Next(),
    Execute,
    Exit,
}
