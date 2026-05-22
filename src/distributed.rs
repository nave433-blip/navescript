use crate::polyglot::PolyglotBridge;

// #[cfg(test)]
// mod distributed_tests;

#[derive(Debug)]
pub enum DistributionMethod {
    PackageManagers,
    StandaloneBinaries,
    ContainerCloud,
    CloudFaaS,
    IDEExtensions,
    WebCDN,
    SDKs,
}

pub struct DistributionManager {
    bridge: PolyglotBridge,
}

impl DistributionManager {
    pub fn new() -> Result<Self, String> {
        let bridge = PolyglotBridge::new().map_err(|e| e.to_string())?;
        Ok(DistributionManager { bridge })
    }

    pub fn deploy(&self, method: DistributionMethod, config: &str) -> Result<String, String> {
        let deployment_code = match method {
            DistributionMethod::PackageManagers => format!("import navescript_pm\nnavescript_pm.deploy('{}')", config),
            DistributionMethod::StandaloneBinaries => format!("import navescript_bin\nnavescript_bin.build('{}')", config),
            DistributionMethod::ContainerCloud => format!("import navescript_docker\nnavescript_docker.push('{}')", config),
            DistributionMethod::CloudFaaS => format!("import navescript_faas\nnavescript_faas.deploy('{}')", config),
            DistributionMethod::IDEExtensions => format!("import navescript_ide\nnavescript_ide.publish('{}')", config),
            DistributionMethod::WebCDN => format!("import navescript_cdn\nnavescript_cdn.upload('{}')", config),
            DistributionMethod::SDKs => format!("import navescript_sdk\nnavescript_sdk.generate('{}')", config),
        };

        let res = self.bridge.eval_and_transmute("python", &deployment_code, crate::polyglot::ExecutionContext::default())
            .map_err(|e| e.to_string())?;
        
        Ok(res.to_string())
    }
}

pub struct DaskHost {
    bridge: PolyglotBridge,
    address: String,
}

impl DaskHost {
    pub fn new(address: &str) -> Result<Self, String> {
        let bridge = PolyglotBridge::new().map_err(|e| e.to_string())?;
        Ok(DaskHost { bridge, address: address.to_string() })
    }

    pub fn submit(&self, code: &str) -> Result<String, String> {
        let full_code = format!(
            "from dask.distributed import Client\nclient = Client('{}')\n{}\nresult = client.submit(lambda: task()).result()",
            self.address, code
        );
        let res = self.bridge.eval_and_transmute("python", &full_code, crate::polyglot::ExecutionContext::default())
            .map_err(|e| e.to_string())?;
        
        Ok(res.to_string())
    }
}
