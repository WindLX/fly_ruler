use fly_ruler_plugin::{AsPlugin, PluginError, PluginInfo, PluginState};
use fly_ruler_utils::error::FrError;
use log::{debug, warn};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct PluginManager<Pl: AsPlugin> {
    pub plugins: HashMap<usize, Pl>,
}

impl<Pl: AsPlugin> PluginManager<Pl> {
    pub fn new<P, F>(path: P, builder: F) -> PluginManager<Pl>
    where
        P: AsRef<Path>,
        F: Fn(&Path) -> Result<Pl, PluginError>,
    {
        let mut plugins = HashMap::new();
        if let Ok(entries) = fs::read_dir(path) {
            for (idx, entry) in entries.into_iter().filter_map(|e| e.ok()).enumerate() {
                if entry.file_type().is_ok_and(|f| f.is_dir()) {
                    let sub_dir = entry.path();
                    debug!("found plugin directory: {}", sub_dir.display());
                    let plugin = builder(&sub_dir);
                    match plugin {
                        Ok(m) => {
                            plugins.insert(idx, m);
                        }
                        Err(e) => {
                            warn!("not plugin directory: {}", e);
                        }
                    }
                }
            }
        }

        PluginManager { plugins }
    }
}

pub trait AsPluginManager<Pl: AsPlugin> {
    fn plugin_manager(&self) -> &PluginManager<Pl>;

    fn plugin_manager_mut(&mut self) -> &mut PluginManager<Pl>;

    fn infos(&self) -> HashMap<usize, PluginInfo> {
        self.plugin_manager()
            .plugins
            .iter()
            .map(|(k, v)| (*k, v.info()))
            .collect()
    }

    fn info(&self, index: usize) -> Option<PluginInfo> {
        self.plugin_manager().plugins.get(&index).map(|p| p.info())
    }

    fn states(&self) -> HashMap<usize, PluginState> {
        self.plugin_manager()
            .plugins
            .iter()
            .map(|(k, v)| (*k, v.state()))
            .collect()
    }

    fn state(&self, index: usize) -> Option<PluginState> {
        self.plugin_manager().plugins.get(&index).map(|p| p.state())
    }

    fn plugin_count(&self) -> usize {
        self.plugin_manager().plugins.len()
    }

    fn plugin(&self, idx: usize) -> Option<&Pl> {
        self.plugin_manager().plugins.get(&idx)
    }

    fn plugin_mut(&mut self, idx: usize) -> Option<&mut Pl> {
        self.plugin_manager_mut().plugins.get_mut(&idx)
    }

    fn enable(&mut self, index: usize, args: &[impl ToString]) -> Result<(), FrError> {
        let plugin = self.plugin_mut(index);
        match plugin {
            Some(pl) => {
                if pl.state() == PluginState::Disable {
                    match pl.plugin().install(args) {
                        Ok(Ok(())) => {
                            pl.plugin_mut().set_state(PluginState::Enable);
                            Ok(())
                        }
                        Ok(Err(e)) => {
                            pl.plugin_mut().set_state(PluginState::Failed);
                            warn!("{}", e);
                            Ok(())
                        }
                        Err(e) => {
                            pl.plugin_mut().set_state(PluginState::Failed);
                            Err(FrError::Plugin(e))
                        }
                    }
                } else {
                    warn!("plugin {} already enabled", pl.info().name);
                    Ok(())
                }
            }
            None => {
                warn!("invalid plugin index");
                Ok(())
            }
        }
    }

    fn disable(&mut self, index: usize) -> Result<(), FrError> {
        let plugin = self.plugin_mut(index);
        match plugin {
            Some(pl) => {
                if pl.state() == PluginState::Enable {
                    match pl.plugin().uninstall() {
                        Ok(Ok(())) => {
                            pl.plugin_mut().set_state(PluginState::Disable);
                            Ok(())
                        }
                        Ok(Err(e)) => {
                            pl.plugin_mut().set_state(PluginState::Failed);
                            warn!("{}", e);
                            Ok(())
                        }
                        Err(e) => {
                            pl.plugin_mut().set_state(PluginState::Failed);
                            Err(FrError::Plugin(e))
                        }
                    }
                } else {
                    Ok(())
                }
            }
            None => {
                warn!("model {} not found", index);
                Ok(())
            }
        }
    }

    fn disable_all(&mut self) -> Result<(), FrError> {
        let keys = self
            .plugin_manager()
            .plugins
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        for k in keys {
            self.disable(k)?;
        }
        Ok(())
    }
}

impl<Pl> AsPluginManager<Pl> for PluginManager<Pl>
where
    Pl: AsPlugin,
{
    fn plugin_manager(&self) -> &PluginManager<Pl> {
        self
    }
    fn plugin_manager_mut(&mut self) -> &mut PluginManager<Pl> {
        self
    }
}
