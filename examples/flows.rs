use tracing::{info, span, Level};
use tracing_subscriber::prelude::*;

fn main() {
    let (chrome_layer, _guard) = tracing_chrome::ChromeLayerBuilder::new()
        .include_args(true)
        .file("trace-flows.json")
        .build();
    tracing_subscriber::registry().with(chrome_layer).init();

    let parent_span = span!(Level::INFO, "parent_task");
    let _entered = parent_span.enter();

    info!("Starting parent task");

    // Create child spans that follow from the parent
    let child1_span = span!(Level::INFO, "child_task_1");
    child1_span.follows_from(parent_span.clone());

    let child2_span = span!(Level::INFO, "child_task_2");
    child2_span.follows_from(parent_span.clone());

    // Simulate work in child spans
    {
        let _entered = child1_span.enter();
        info!("Working on child task 1");
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    {
        let _entered = child2_span.enter();
        info!("Working on child task 2");
        std::thread::sleep(std::time::Duration::from_millis(150));
    }

    info!("Parent task completed");
}
