use image::{DynamicImage, GenericImage};

use crate::trainer::Trainer;

/// Combines several trainer sprites into one by stitching them horizontally.
pub fn combine(trainers: &[Trainer]) -> DynamicImage {
    let mut width: u32 = 0;
    let mut height: u32 = 0;

    for trainer in trainers {
        width += trainer.sprite.width() + 1;
        if trainer.sprite.height() > height {
            height = trainer.sprite.height();
        }
    }

    let mut combined = DynamicImage::new_rgba8(width - 1, height);
    let mut shift = 0;

    for trainer in trainers {
        combined
            .copy_from(&trainer.sprite, shift, height - trainer.sprite.height())
            .unwrap();
        shift += trainer.sprite.width() + 1;
    }

    combined
}