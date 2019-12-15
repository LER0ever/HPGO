pub mod analysis;
pub mod orchestrate;
pub mod orchestrate_hierarchical;

pub trait Conductor {
    fn orchestrate(&self);
    fn compute_plan(&mut self);
    fn analyse_plan(&self);
}
