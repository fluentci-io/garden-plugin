use std::vec;

use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup_garden(version: &str) -> Result<String, Error> {
    let arch = dag().get_arch()?;
    let mut os = dag().get_os()?;

    let arch = match arch.as_str() {
        "x86_64" => "amd64".into(),
        "aarch64" => "arm64".into(),
        _ => arch,
    };

    if os == "linux" {
        let is_alpine = !dag()
            .pkgx()?
            .with_exec(vec!["cat /etc/os-release | grep -q 'alpine' || echo KO"])?
            .stdout()?
            .contains("KO");

        if is_alpine {
            os = "alpine".into();
        }
    }

    dag().set_envs(vec![
        ("ARCH".into(), arch),
        ("OS".into(), os),
        ("GARDEN_VERSION".into(), "0.13.35".into()),
    ])?;

    if !version.is_empty() {
        dag().set_envs(vec![("VERSION".into(), version.into())])?;
    }

    let home = dag().get_env("HOME")?;
    let path = dag().get_env("PATH")?;

    dag().set_envs(vec![(
        "PATH".into(),
        format!("{}/.local/bin:{}", home, path),
    )])?;

    let stdout = dag()
        .pkgx()?
        .with_exec(vec![
          "type garden > /dev/null || pkgx wget https://github.com/garden-io/garden/releases/download/${VERSION}/garden-${VERSION}-${OS}-${ARCH}.tar.gz",
        ])?
        .with_exec(vec![
            "type garden > /dev/null 2> /dev/null || pkgx tar xvf garden-${VERSION}-${OS}-${ARCH}.tar.gz",
        ])?
        .with_exec(vec!["type garden > /dev/null 2> /dev/null || cp ${OS}-${ARCH}/garden $HOME/.local/bin"])?
        .with_exec(vec!["[ -d ${OS}-${ARCH} ] && rm -rf ${OS}-${ARCH} || true"])?
        .with_exec(vec!["[ -f garden-${VERSION}-${OS}-${ARCH}.tar.gz ] && rm -rf garden-${VERSION}-${OS}-${ARCH}.tar.gz || true"])?
        .stdout()?;

    Ok(stdout)
}
