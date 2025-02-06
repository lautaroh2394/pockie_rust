use crate::{clickable::Clickable, drawable::Drawable};

pub trait Screenable: Drawable + Clickable {}