#![cfg(feature = "macros")]

use caretta_framework::{types::AppInfo, util::RunnableCommand};
use rmcp::model::Implementation;

struct RunnableCommandStruct1;

impl RunnableCommand for RunnableCommandStruct1 {
    fn run(self, _app_info: AppInfo) {
        print!("Run {}", stringify!(RunnableCommandStruct1::run()))
    }
}

#[derive(RunnableCommand)]
enum RunnableCommandEnum {
    Struct1(RunnableCommandStruct1),
}

#[derive(RunnableCommand)]
struct RunnableCommandStruct2 {
    #[runnable_command]
    runnable: RunnableCommandEnum,
}

#[tokio::test]
async fn test() {
    let runnable = RunnableCommandStruct2 {
        runnable: RunnableCommandEnum::Struct1(RunnableCommandStruct1),
    };
    runnable.run(AppInfo { name: "example", info: Implementation::from_build_env() });
}
