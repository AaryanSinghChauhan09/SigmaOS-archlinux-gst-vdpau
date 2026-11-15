//! SigmaOS-archlinux-gst-vdpau: GStreamer VDPAU plugin model.

use std::collections::BTreeMap;
use std::fmt;

/// Primary domain entity for GStreamer VDPAU plugin.
pub struct VdpauDevice {
    pub id: String,
    pub name: String,
    pub device_id: String,
    pub enabled: bool,
}

/// Secondary domain entity for GStreamer VDPAU plugin.
pub struct VdpauSurface {
    pub id: String,
    pub format: String,
    pub description: String,
}

/// Errors for the GStreamer VDPAU plugin domain.
#[derive(Debug, PartialEq)]
pub enum GstVdpauError {
    EmptyName,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for GstVdpauError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
        }
    }
}

impl std::error::Error for GstVdpauError {}

/// Registry and manager for GStreamer VDPAU plugin.
pub struct GstVdpauManager {
    pub name: String,
    items: BTreeMap<String, VdpauDevice>,
    configs: BTreeMap<String, VdpauSurface>,
}

impl GstVdpauManager {
    /// Create a new manager instance.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: BTreeMap::new(),
            configs: BTreeMap::new(),
        }
    }

    /// Add a primary item with fail-fast validation.
    pub fn add_item(&mut self, item: VdpauDevice) -> Result<(), GstVdpauError> {
        if item.name.is_empty() {
            return Err(GstVdpauError::EmptyName);
        }
        if self.items.contains_key(&item.id) {
            return Err(GstVdpauError::Duplicate(item.id.clone()));
        }
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    /// Retrieve a primary item by id.
    pub fn get_item(&self, id: &str) -> Option<&VdpauDevice> {
        self.items.get(id)
    }

    /// Count of primary items.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Add a secondary configuration entry.
    pub fn add_config(&mut self, cfg: VdpauSurface) {
        self.configs.insert(cfg.id.clone(), cfg);
    }

    /// Count of configuration entries.
    pub fn config_count(&self) -> usize {
        self.configs.len()
    }

    /// Enable or disable an item.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> Result<(), GstVdpauError> {
        self.items
            .get_mut(id)
            .map(|item| item.enabled = enabled)
            .ok_or_else(|| GstVdpauError::NotFound(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_VdpauDevice(id: &str, name: &str) -> VdpauDevice {
        VdpauDevice {
            id: id.to_string(),
            name: name.to_string(),
            device_id: "archlinux-gst-vdpau-default".to_string(),
            enabled: true,
        }
    }

    fn make_VdpauSurface(id: &str, val: &str) -> VdpauSurface {
        VdpauSurface {
            id: id.to_string(),
            format: val.to_string(),
            description: format!("{} config", id),
        }
    }

    #[test]
    fn test_create_aaa() {
        let mgr = GstVdpauManager::new("test");
        assert_eq!(mgr.name, "test");
        assert_eq!(mgr.count(), 0);
        assert_eq!(mgr.config_count(), 0);
    }

    #[test]
    fn test_add_item_aaa() {
        let mut mgr = GstVdpauManager::new("test");
        let item = make_VdpauDevice("item1", "First Item");
        assert!(mgr.add_item(item).is_ok());
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut mgr = GstVdpauManager::new("test");
        let item = make_VdpauDevice("item1", "");
        assert_eq!(mgr.add_item(item), Err(GstVdpauError::EmptyName));
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut mgr = GstVdpauManager::new("test");
        let item1 = make_VdpauDevice("item1", "First");
        let item2 = make_VdpauDevice("item1", "Duplicate");
        mgr.add_item(item1).unwrap();
        assert_eq!(mgr.add_item(item2), Err(GstVdpauError::Duplicate("item1".to_string())));
    }

    #[test]
    fn test_config_addition_aaa() {
        let mut mgr = GstVdpauManager::new("test");
        let cfg = make_VdpauSurface("cfg1", "value1");
        mgr.add_config(cfg);
        assert_eq!(mgr.config_count(), 1);
    }

    #[test]
    fn test_get_item_aaa() {
        let mut mgr = GstVdpauManager::new("test");
        let item = make_VdpauDevice("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.get_item("item1").is_some());
        assert!(mgr.get_item("none").is_none());
    }

    #[test]
    fn test_set_enabled_aaa() {
        let mut mgr = GstVdpauManager::new("test");
        let item = make_VdpauDevice("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.set_enabled("item1", false).is_ok());
        assert!(!mgr.get_item("item1").unwrap().enabled);
        assert_eq!(mgr.set_enabled("none", true), Err(GstVdpauError::NotFound("none".to_string())));
    }
}
