use std::fmt::Debug;

use rspack_core::{
  ApplyContext, CompilerOptions, ContextModuleFactoryBeforeResolve, ModuleFactoryCreateData,
  NormalModuleFactoryBeforeResolve, Plugin, PluginContext,
};
use rspack_error::Result;
use rspack_hook::{plugin, plugin_hook};
use rspack_regex::RspackRegex;

#[derive(Debug)]
pub struct IgnorePluginOptions {
  pub resource_reg_exp: RspackRegex,
  pub context_reg_exp: Option<RspackRegex>,
}

#[plugin]
#[derive(Debug)]
pub struct IgnorePlugin {
  options: IgnorePluginOptions,
}

impl IgnorePlugin {
  pub fn new(options: IgnorePluginOptions) -> Self {
    Self::new_inner(options)
  }

  fn check_ignore(&self, data: &mut ModuleFactoryCreateData) -> Option<bool> {
    let resource_reg_exp: &RspackRegex = &self.options.resource_reg_exp;

    if resource_reg_exp.test(data.request()?) {
      if let Some(context_reg_exp) = &self.options.context_reg_exp {
        if context_reg_exp.test(&data.context) {
          return Some(false);
        }
      } else {
        return Some(false);
      }
    }
    None
  }
}

#[plugin_hook(NormalModuleFactoryBeforeResolve for IgnorePlugin)]
async fn nmf_before_resolve(&self, data: &mut ModuleFactoryCreateData) -> Result<Option<bool>> {
  Ok(self.check_ignore(data))
}

#[plugin_hook(ContextModuleFactoryBeforeResolve for IgnorePlugin)]
async fn cmf_before_resolve(&self, data: &mut ModuleFactoryCreateData) -> Result<Option<bool>> {
  Ok(self.check_ignore(data))
}

impl Plugin for IgnorePlugin {
  fn name(&self) -> &'static str {
    "IgnorePlugin"
  }

  fn apply(
    &self,
    ctx: PluginContext<&mut ApplyContext>,
    _options: &mut CompilerOptions,
  ) -> Result<()> {
    ctx
      .context
      .normal_module_factory_hooks
      .before_resolve
      .tap(nmf_before_resolve::new(self));

    ctx
      .context
      .context_module_factory_hooks
      .before_resolve
      .tap(cmf_before_resolve::new(self));

    Ok(())
  }
}