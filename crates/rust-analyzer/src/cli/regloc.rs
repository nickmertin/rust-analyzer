#![cfg(feature = "regloc")]

use std::env;

// use ide::StaticIndex;
use project_model::{CargoConfig, ProjectManifest, ProjectWorkspace, RustLibSource};
// use tokio::runtime;
use vfs::AbsPathBuf;

use crate::cli::{
    flags,
    load_cargo::{load_workspace, LoadCargoConfig, ProcMacroServerChoice},
    Result,
};

impl flags::Regloc {
    pub fn run(self) -> Result<()> {
        let mut cargo_config = CargoConfig::default();
        cargo_config.sysroot = Some(RustLibSource::Discover);
        let no_progress = &|_| ();
        let load_cargo_config = LoadCargoConfig {
            load_out_dirs_from_check: true,
            with_proc_macro_server: ProcMacroServerChoice::Sysroot,
            prefill_caches: false,
        };
        let path = AbsPathBuf::assert(env::current_dir()?.join(&self.path));
        let manifest = ProjectManifest::discover_single(&path)?;

        eprintln!("Loading workspace...");

        let workspace = ProjectWorkspace::load(manifest, &cargo_config, no_progress)?;

        let (host, _vfs, _proc_macro) =
            load_workspace(workspace, &cargo_config.extra_env, &load_cargo_config)?;
        let db = host.raw_database();
        let analysis = host.analysis();

        regloc::run(&analysis, db, self.regression)?;

        // let runtime = runtime::Builder::new_multi_thread().enable_all().build()?;

        // let si = runtime.block_on(StaticIndex::compute_par(&analysis));

        // eprintln!("{} files", si.files.len());

        Ok(())
    }
}
