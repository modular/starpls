use anyhow::anyhow;
use log::debug;
use log::info;
use starpls_bazel::client::BazelClient;
use starpls_bazel::client::BazelInfo;
use starpls_bazel::Builtins;

use crate::server::load_bazel_build_language;

/// Contains information about the current Bazel configuration as fetched from
/// various `bazel info` commands.
#[derive(Default)]
pub(crate) struct BazelContext {
    pub(crate) info: BazelInfo,
    pub(crate) rules: Builtins,
    pub(crate) bzlmod_enabled: bool,
}

impl BazelContext {
    pub(crate) fn new(client: &dyn BazelClient) -> anyhow::Result<BazelContext> {
        let info = client
            .info()
            .map_err(|err| anyhow!("failed to run `bazel info`: {}", err))?;

        info!("workspace root: {:?}", info.workspace);
        info!("workspace name: {:?}", info.workspace_name);
        info!("starlark-semantics: {:?}", info.starlark_semantics);

        // Check if bzlmod is enabled for the current workspace.
        let bzlmod_enabled = {
            let bzlmod_enabled_by_default = true;

            // Check starlark-semantics to determine whether bzlmod has been explicitly
            // enabled/disabled, e.g. in a .bazelrc file.
            if info.starlark_semantics.contains("enable_bzlmod=true") {
                info!("found enable_bzlmod=true in starlark semantics");
                true
            } else if info.starlark_semantics.contains("enable_bzlmod=false") {
                info!("found enable_bzlmod=false in starlark semantics");
                false
            } else {
                bzlmod_enabled_by_default
            }
        };

        info!("bzlmod_enabled = {}", bzlmod_enabled);

        // If bzlmod is enabled, we also need to check if the `bazel mod dump_repo_mapping` command is supported.
        if bzlmod_enabled {
            debug!("checking for `bazel mod dump_repo_mapping` capability");
            client
                .dump_repo_mapping("")
                .map_err(|err| anyhow!("failed to run `bazel mod dump_repo_mapping`: {}", err))?;
        }

        debug!("fetching builtin rules via `bazel info build-language`");
        let rules = load_bazel_build_language(client)
            .map_err(|err| anyhow!("failed to run `bazel info build-language`: {}", err))?;

        Ok(BazelContext {
            info,
            rules,
            bzlmod_enabled,
        })
    }
}
