use avian3d::prelude::PhysicsLayer;


#[derive(PhysicsLayer, Default)]
pub enum GamePhysicsLayer {
    #[default]
    Default, 
    Ball,  
    Ground
}