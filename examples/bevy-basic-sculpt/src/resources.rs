use bevy::prelude::*;
use freestyle_sculpt::deformation::DeformationField;

#[derive(Resource, Copy, Clone, Default, Deref, DerefMut)]
pub struct CurrentDeformation(usize);

#[derive(Deref, DerefMut)]
pub struct AvailableDeformations(Vec<Box<dyn DeformationField>>);

impl AvailableDeformations {
    pub fn new(deformations: Vec<Box<dyn DeformationField>>) -> Self {
        Self(deformations)
    }
}
