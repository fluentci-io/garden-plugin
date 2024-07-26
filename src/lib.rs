use extism_pdk::*;
use fluentci_pdk::dag;

use crate::helpers::setup_garden;

pub mod helpers;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    let stdout = setup_garden(&version)?;
    Ok(stdout)
}

#[plugin_fn]
pub fn build(args: String) -> FnResult<String> {
    setup_garden("")?;
    let stdout = dag()
        .pipeline("build")?
        .with_exec(vec!["garden", "build", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn cleanup(args: String) -> FnResult<String> {
    setup_garden("")?;
    let stdout = dag()
        .pipeline("cleanup")?
        .with_exec(vec!["garden", "cleanup", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn deploy(args: String) -> FnResult<String> {
    setup_garden("")?;
    let stdout = dag()
        .pipeline("deploy")?
        .with_exec(vec!["garden", "deploy", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn get(args: String) -> FnResult<String> {
    setup_garden("")?;
    let stdout = dag()
        .pipeline("get")?
        .with_exec(vec!["garden", "get", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn publish(args: String) -> FnResult<String> {
    setup_garden("")?;
    let stdout = dag()
        .pipeline("publish")?
        .with_exec(vec!["garden", "publish", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn run(args: String) -> FnResult<String> {
    setup_garden("")?;
    let stdout = dag()
        .pipeline("run")?
        .with_exec(vec!["garden", "run", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn test(args: String) -> FnResult<String> {
    setup_garden("")?;
    let stdout = dag()
        .pipeline("test")?
        .with_exec(vec!["garden", "test", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn validate(args: String) -> FnResult<String> {
    setup_garden("")?;
    let stdout = dag()
        .pipeline("validate")?
        .with_exec(vec!["garden", "validate", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn workflow(args: String) -> FnResult<String> {
    setup_garden("")?;
    let stdout = dag()
        .pipeline("workflow")?
        .with_exec(vec!["garden", "workflow", &args])?
        .stdout()?;

    Ok(stdout)
}
