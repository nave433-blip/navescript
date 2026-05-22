use crate::polyglot::PolyglotBridge;

pub struct LdapHost {
    bridge: PolyglotBridge,
    server: String,
    user: String,
    pass: String,
}

impl LdapHost {
    pub fn new(server: &str, user: &str, pass: &str) -> Result<Self, String> {
        let bridge = PolyglotBridge::new().map_err(|e| e.to_string())?;
        Ok(LdapHost { bridge, server: server.to_string(), user: user.to_string(), pass: pass.to_string() })
    }

    pub fn search(&self, base_dn: &str, filter: &str) -> Result<Vec<String>, String> {
        let code = format!(
            "from ldap3 import Server, Connection, ALL\nserver = Server('{}', get_info=ALL)\nconn = Connection(server, '{}', '{}', auto_bind=True)\nconn.search('{}', '{}')\nresult = [e.entry_dn for e in conn.entries]",
            self.server, self.user, self.pass, base_dn, filter
        );
        let res = self.bridge.eval_and_transmute("python", &code, crate::polyglot::ExecutionContext::default())
            .map_err(|e| e.to_string())?;
        
        let vec: Vec<String> = serde_json::from_value(res).map_err(|e| e.to_string())?;
        Ok(vec)
    }
}
