pub mod orchestrate;
pub mod orchestrate_hierarchical;

pub trait OrchestrationResult {
    fn get_speedup(&self) -> Option<f64>;
    fn get_splits(&self) -> Option<Vec<u32>>;

    fn pretty_print(&self) -> Option<String>;
}

pub trait Conductor {
    // call corresponding new() functions
    fn orchestrate(&self);
    fn compute_plan(&mut self);
    fn analyse_plan(&self);
    fn return_plan(&self) -> Box<dyn OrchestrationResult>;
}