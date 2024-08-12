// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

mod rspack_bundle;
use std::sync::Arc;

use deno_core::error::AnyError;

use deno_terminal::colors;

use crate::args::BundleFlags;
use crate::args::Flags;

use crate::factory::CliFactory;
use crate::util;
use crate::tools::pack::rspack_bundle::rspack;
pub async fn pack(
  flags: Arc<Flags>,
  bundle_flags: BundleFlags,
) -> Result<(), AnyError> {
  log::info!(
    "{}",
    colors::yellow("⚠️ Warning: `deno bundle` is deprecated and will be removed in Deno 2.0.\nUse an alternative bundler like \"deno_emit\", \"esbuild\" or \"rollup\" instead."),
  );

  if let Some(watch_flags) = &bundle_flags.watch {
    util::file_watcher::watch_func(
      flags,
      util::file_watcher::PrintConfig::new(
        "Bundle",
        !watch_flags.no_clear_screen,
      ),
      move |flags, watcher_communicator, _changed_paths| {
        let bundle_flags = bundle_flags.clone();
        Ok(async move {
          let factory = CliFactory::from_flags_for_watcher(
            flags,
            watcher_communicator.clone(),
          );
          let cli_options = factory.cli_options()?;
          let _ = watcher_communicator.watch_paths(cli_options.watch_paths());
          rspack(factory, &bundle_flags).await?;

          Ok(())
        })
      },
    )
    .await?;
  } else {
    let factory = CliFactory::from_flags(flags);
    rspack(factory, &bundle_flags).await?;
  }

  Ok(())
}
