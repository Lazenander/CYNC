use crate::pipeline::driver::driver::CompilerDriver;

pub mod merge_module;
mod module_skeleton;

impl CompilerDriver {
    pub fn resolve_names(&mut self) -> bool {
        self.merge_modules() && self.module_skeleton()
    }
}
