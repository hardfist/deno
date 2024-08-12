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
    let factory = CliFactory::from_flags(flags);
    rspack(factory, &bundle_flags).await?;
  Ok(())
}
