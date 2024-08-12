use std::path::Path;
use std::sync::Arc;
use deno_core::error::AnyError;
use rspack_core::ResolverFactory;
use rspack_ids::NaturalChunkIdsPlugin;
use rspack_ids::NamedModuleIdsPlugin;
use rspack_core::JavascriptParserOptions;
use rspack_core::ModuleType;
use rspack_core::ParserOptions;
use rspack_core::ParserOptionsByModuleType;
use rspack_plugin_runtime::{RuntimePlugin};
use rspack_core::{
    CacheOptions, ChunkLoading, ChunkLoadingType, Compiler, CompilerOptions, Context,
    CrossOriginLoading, DevServerOptions, EntryOptions, Environment, Experiments, Filename,
    HashDigest, HashFunction, HashSalt, MangleExportsOption, Mode, ModuleOptions, Optimization,
    OutputOptions, PathInfo, Plugin, PublicPath, Resolve, SideEffectOption, SnapshotOptions,
    StatsOptions, Target, UsedExportsOption, WasmLoading,
};
use rspack_fs::AsyncNativeFileSystem;
use rspack_plugin_entry::EntryPlugin;
use rspack_plugin_javascript::JsPlugin;
use rspack_plugin_schemes::DataUriPlugin;
use crate::{CliFactory};
use crate::args::BundleFlags;

pub async fn rspack(
    factory: CliFactory,
    bundle_flags: &BundleFlags,
) -> Result<(), AnyError> {
    let output_filesystem = AsyncNativeFileSystem {};
    let root = std::env::current_dir().unwrap();
    let context = Context::from(root.clone());
    let entry_request: String = root
        .join(bundle_flags.source_file.clone())
        .canonicalize()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let options = CompilerOptions {
        context: Context::from(root.clone()),
        dev_server: DevServerOptions::default(),
        output: OutputOptions {
            chunk_load_timeout: Default::default(),
            charset: Default::default(),
            css_head_data_compression: Default::default(),
            import_meta_name: Default::default(),
            path: root.clone(),
            pathinfo: PathInfo::Bool(false),
            clean: false,
            public_path: PublicPath::Auto,
            asset_module_filename: Filename::from(String::from("asset-[name].js")),
            wasm_loading: WasmLoading::Disable,
            webassembly_module_filename: Filename::from(String::from("webassembly.js")),
            unique_name: "main".into(),
            chunk_loading: ChunkLoading::Enable(ChunkLoadingType::Import),
            chunk_loading_global: String::new(),
            filename: Filename::from(String::from(bundle_flags.out_file.clone().unwrap_or("output.js".to_string()))),
            chunk_filename: Filename::from(String::from("[id].js")),
            cross_origin_loading: CrossOriginLoading::Disable,
            css_filename: Filename::from(String::from("[name].css")),
            css_chunk_filename: Filename::from(String::from("[id].css")),
            hot_update_main_filename: Filename::from(String::from("[name].[hash].hot-update.js")),
            hot_update_chunk_filename: Filename::from(String::from("[id].[hash].hot-update.js")),
            hot_update_global: String::new(),
            library: None,
            enabled_library_types: None,
            strict_module_error_handling: false,
            global_object: String::from("window"),
            import_function_name: String::from("import"),
            iife: false,
            module: false,
            trusted_types: None,
            source_map_filename: Filename::from(String::from("[file].map")),
            hash_function: HashFunction::MD4,
            hash_digest: HashDigest::Hex,
            hash_digest_length: 20,
            hash_salt: HashSalt::Salt(String::from("salt")),
            async_chunks: false,
            worker_chunk_loading: ChunkLoading::Disable,
            worker_wasm_loading: WasmLoading::Disable,
            worker_public_path: String::new(),
            script_type: String::from("text/javascript"),
            environment: Environment {
                r#const: Some(true),
                arrow_function: Some(true),
            },
        },
        target: Target::new(&vec!["es2022".to_string(), "node".to_string()]).unwrap(),
        mode: Mode::Development,
        resolve: Resolve {
            extensions: Some(vec![".js".to_string()]),
            ..Default::default()
        },
        resolve_loader: Resolve {
            extensions: Some(vec![".js".to_string()]),
            ..Default::default()
        },
        module: ModuleOptions {
            parser: Some(ParserOptionsByModuleType::from_iter([(
                ModuleType::JsAuto,
                ParserOptions::Javascript(JavascriptParserOptions {
                    dynamic_import_fetch_priority: Default::default(),
                    override_strict: Default::default(),
                    import_meta: Default::default(),
                    dynamic_import_mode: rspack_core::DynamicImportMode::Eager,
                    dynamic_import_prefetch: rspack_core::JavascriptParserOrder::Order(1),
                    dynamic_import_preload: rspack_core::JavascriptParserOrder::Order(1),
                    url: rspack_core::JavascriptParserUrl::Disable,
                    expr_context_critical: false,
                    wrapped_context_critical: false,
                    exports_presence: None,
                    import_exports_presence: None,
                    reexport_exports_presence: None,
                    strict_export_presence: false,
                    worker: vec![],
                }),
            )])),
            //    generator: Some(GeneratorOptionsByModuleType::from_iter(generator.iter())),
            ..Default::default()
        },
        stats: StatsOptions::default(),
        snapshot: SnapshotOptions,
        cache: CacheOptions::default(),
        experiments: Experiments::default(),
        optimization: Optimization {
            concatenate_modules: false,
            remove_available_modules: false,
            provided_exports: false,
            mangle_exports: MangleExportsOption::False,
            inner_graph: true,
            used_exports: UsedExportsOption::default(),
            side_effects: SideEffectOption::default(),
        },
        profile: false,
        bail: false,
        __references:Default::default(),
        node: None,
    };
    let mut plugins: Vec<Box<dyn Plugin>> = Vec::new();

    let plugin_options = EntryOptions {
        name: Some("main".to_string()),
        runtime: None,
        chunk_loading: None,
        async_chunks: None,
        public_path: None,
        base_uri: None,
        filename:None,
        library: None,
        depend_on: None,
        layer: Default::default(),
    };
    
    let entry_plugin = Box::new(EntryPlugin::new(context, entry_request, plugin_options));
    plugins.push(Box::<JsPlugin>::default());
    plugins.push(entry_plugin);
    plugins.push(Box::<NaturalChunkIdsPlugin>::default());
    plugins.push(Box::<NamedModuleIdsPlugin>::default());
    plugins.push(Box::<DataUriPlugin>::default());
    plugins.push(Box::<RuntimePlugin>::default());
    let resolver_factory = Arc::new(ResolverFactory::new(options.resolve.clone()));
    let loader_resolver_factory = Arc::new(ResolverFactory::new(options.resolve_loader.clone()));
    let mut compiler = Compiler::new(options, plugins, output_filesystem, resolver_factory, loader_resolver_factory);
    compiler.build().await.expect("build failed");
    Ok(())
}
