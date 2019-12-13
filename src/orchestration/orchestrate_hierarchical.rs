use orchestration::orchestrate;
use rayon::prelude::*;

impl<'a> orchestrate::Conductor<'a> {
    pub fn orchestrate_hierarchical(&self) {
        unimplemented!()
    }

    pub fn compute_plan_hierarchical(
        &self,
        num_machines: u32,
        num_cards_per_machine: u32,
        final_level: bool,
    ) {
        let compute_times = &self.m.perf.compute_times;
        let activation_sizes = &self.m.perf.activation_sizes;
        let output_activation_sizes = &self.m.perf.output_activation_sizes;
        let parameter_sizes = &self.m.perf.parameter_sizes;

        unimplemented!()
    }
}
