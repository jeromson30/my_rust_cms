use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

/// Performance tracking service for frontend metrics
#[derive(Debug, Clone)]
pub struct PerformanceService {
    metrics: Arc<Mutex<PerformanceMetrics>>,
}

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub page_load_time: f64,
    pub time_to_interactive: f64,
    pub first_contentful_paint: f64,
    pub largest_contentful_paint: f64,
    pub cumulative_layout_shift: f64,
    pub network_requests: Vec<NetworkRequest>,
    pub component_render_times: HashMap<String, f64>,
    pub dom_nodes_count: u32,
    pub memory_usage: f64,
    pub wasm_bundle_size: f64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NetworkRequest {
    #[allow(dead_code)]
    pub url: String,
    pub duration: f64,
    pub size: f64,
    pub status: u16,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            page_load_time: 0.0,
            time_to_interactive: 0.0,
            first_contentful_paint: 0.0,
            largest_contentful_paint: 0.0,
            cumulative_layout_shift: 0.0,
            network_requests: Vec::new(),
            component_render_times: HashMap::new(),
            dom_nodes_count: 0,
            memory_usage: 0.0,
            wasm_bundle_size: 0.0,
        }
    }
}

impl PerformanceService {
    pub fn new() -> Result<Self, String> {
        let service = Self {
            metrics: Arc::new(Mutex::new(PerformanceMetrics::default())),
        };
        
        // Initialize with mock metrics
        service.update_basic_metrics()?;
        
        Ok(service)
    }
    
    fn update_basic_metrics(&self) -> Result<(), String> {
        let mut metrics = self.metrics.lock().unwrap();
        
        // Set reasonable mock values for demonstration
        metrics.page_load_time = 2.1;
        metrics.time_to_interactive = 2.8;
        metrics.first_contentful_paint = 1.2;
        metrics.largest_contentful_paint = 2.4;
        metrics.cumulative_layout_shift = 0.08;
        metrics.wasm_bundle_size = 1250.4;
        metrics.memory_usage = 45.6;
        metrics.dom_nodes_count = 1850;
        
        // Add some mock network requests
        metrics.network_requests.clear();
        metrics.network_requests.push(NetworkRequest {
            url: "/api/stats".to_string(),
            duration: 120.5,
            size: 2048.0,
            status: 200,
        });
        metrics.network_requests.push(NetworkRequest {
            url: "/api/performance".to_string(),
            duration: 95.2,
            size: 4096.0,
            status: 200,
        });
        
        Ok(())
    }
    
    /// Track a component render time
    #[allow(dead_code)]
    pub fn track_component_render(&self, component_name: &str, render_time: f64) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.component_render_times.insert(component_name.to_string(), render_time);
    }
    
    /// Track a network request
    #[allow(dead_code)]
    pub fn track_network_request(&self, url: String, duration: f64, size: f64, status: u16) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.network_requests.push(NetworkRequest {
            url,
            duration,
            size,
            status,
        });
    }
    
    /// Update DOM nodes count (simplified)
    pub fn update_dom_nodes_count(&self) -> Result<(), String> {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.dom_nodes_count = 1850; // Mock value
        Ok(())
    }
    
    /// Update memory usage (simplified version)
    pub fn update_memory_usage(&self) -> Result<(), String> {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.memory_usage = 45.6; // MB - placeholder
        Ok(())
    }
    
    /// Estimate WASM bundle size
    pub fn estimate_wasm_bundle_size(&self) -> Result<(), String> {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.wasm_bundle_size = 1250.4; // KB - placeholder
        Ok(())
    }
    
    /// Get current metrics
    pub fn get_metrics(&self) -> PerformanceMetrics {
        self.metrics.lock().unwrap().clone()
    }
    
    /// Get aggregated network metrics
    pub fn get_network_metrics(&self) -> NetworkMetrics {
        let metrics = self.metrics.lock().unwrap();
        
        if metrics.network_requests.is_empty() {
            return NetworkMetrics {
                avg_request_time: 0.0,
                total_requests: 0,
                total_data_transferred: 0.0,
                error_rate: 0.0,
            };
        }
        
        let total_time: f64 = metrics.network_requests.iter().map(|req| req.duration).sum();
        let total_size: f64 = metrics.network_requests.iter().map(|req| req.size).sum();
        let error_count = metrics.network_requests.iter().filter(|req| req.status >= 400).count();
        
        NetworkMetrics {
            avg_request_time: total_time / metrics.network_requests.len() as f64,
            total_requests: metrics.network_requests.len(),
            total_data_transferred: total_size / 1024.0, // Convert to KB
            error_rate: (error_count as f64 / metrics.network_requests.len() as f64) * 100.0,
        }
    }
    
    /// Get average component render time
    pub fn get_avg_component_render_time(&self) -> f64 {
        let metrics = self.metrics.lock().unwrap();
        
        if metrics.component_render_times.is_empty() {
            return 0.0;
        }
        
        let total: f64 = metrics.component_render_times.values().sum();
        total / metrics.component_render_times.len() as f64
    }
    
    /// Force update all metrics
    pub fn update_all_metrics(&self) -> Result<(), String> {
        self.update_basic_metrics()?;
        self.update_dom_nodes_count()?;
        self.update_memory_usage()?;
        self.estimate_wasm_bundle_size()?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NetworkMetrics {
    pub avg_request_time: f64,
    #[allow(dead_code)]
    pub total_requests: usize,
    #[allow(dead_code)]
    pub total_data_transferred: f64,
    #[allow(dead_code)]
    pub error_rate: f64,
}

/// Create a global performance service instance
static PERFORMANCE_SERVICE: OnceLock<PerformanceService> = OnceLock::new();

/// Initialize the global performance service
pub fn init_performance_service() -> Result<(), String> {
    let service = PerformanceService::new()?;
    PERFORMANCE_SERVICE.set(service).map_err(|_| "Performance service already initialized".to_string())?;
    Ok(())
}

/// Get the global performance service instance
pub fn get_performance_service() -> Option<&'static PerformanceService> {
    PERFORMANCE_SERVICE.get()
}

/// Helper function for tracking component render time (simplified)
#[allow(dead_code)]
pub fn track_render_time<F, R>(component_name: &str, f: F) -> R 
where
    F: FnOnce() -> R,
{
    let result = f();
    
    // Use a mock render time for demonstration
    let render_time = 12.3; // ms - placeholder
    
    if let Some(service) = get_performance_service() {
        service.track_component_render(component_name, render_time);
    }
    
    result
}