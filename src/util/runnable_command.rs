use crate::types::AppInfo;

#[cfg(feature = "macros")]
pub use caretta_framework_macros::RunnableCommand;

pub trait RunnableCommand {
    fn run(self, app_info: AppInfo);
}
