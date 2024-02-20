use crate::ginseng::Ginseng;
use synthesis::game::Game;
use synthesis::prelude::{NNPolicy, Policy};
use tch::nn::VarStore;
use tch::Tensor;

pub struct GinsengNet;

impl Policy<Ginseng, { Ginseng::MAX_NUM_ACTIONS }> for GinsengNet {
    fn eval(&mut self, _game: &Ginseng) -> ([f32; Ginseng::MAX_NUM_ACTIONS], [f32; 3]) {
        todo!()
    }
}

impl NNPolicy<Ginseng, { Ginseng::MAX_NUM_ACTIONS }> for GinsengNet {
    fn new(_vs: &VarStore) -> Self {
        todo!()
    }

    fn forward(&self, _xs: &Tensor) -> (Tensor, Tensor) {
        todo!()
    }
}
