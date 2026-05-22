use crate::runtime::{NaveRuntime, Value};
use icu::locid::Locale;

pub fn register(rt: &mut NaveRuntime) {
    rt.register_async("intl_format_currency", |args| async move {
        let amount = args.get(0).and_then(|v| if let Value::Number(n) = v { Some(*n) } else { None }).unwrap_or(0.0);
        let currency = args.get(1).and_then(|v| v.as_str()).unwrap_or("USD");
        let _locale = args.get(2).and_then(|v| v.as_str()).unwrap_or("en-US");
        
        // Simplified: in a real implementation we would use icu::decimal or similar
        Value::String(format!("{} {:.2}", currency, amount))
    });

    rt.register_async("intl_get_locale_info", |args| async move {
        let locale_str = args.get(0).and_then(|v| v.as_str()).unwrap_or("en-US");
        let locale: Locale = locale_str.parse().unwrap();
        
        let mut res = std::collections::HashMap::new();
        res.insert("language".to_string(), Value::String(locale.id.language.to_string()));
        res.insert("region".to_string(), Value::String(locale.id.region.map(|r| r.to_string()).unwrap_or_default()));
        Value::Object(res)
    });
}
