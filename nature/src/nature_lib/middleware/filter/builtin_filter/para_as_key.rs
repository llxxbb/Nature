use crate::domain::*;
use crate::nature_lib::middleware::filter::builtin_filter::FilterBefore;
use crate::util::*;

pub struct ParaAsKey;

#[async_trait]
impl FilterBefore for ParaAsKey {
    async fn filter(&self, ins: &mut Instance, cfg: &str) -> Result<()> {
        // deserialize Setting
        let cfg: Setting = match serde_json::from_str(cfg) {
            Ok(rtn) => rtn,
            Err(e) => {
                let msg = format!("ParaAsKey get cfg error: {}, cfg: {}", e, cfg);
                warn!("{}", msg);
                return Err(NatureError::VerifyError(msg));
            }
        };
        if cfg.part.len() == 0 {
            let msg = "ParaAsKey: para_part must be set!".to_string();
            warn!("{}", msg);
            return Err(NatureError::VerifyError(msg));
        }

        // get para part
        let (part, _) = get_para_and_key_from_para(&ins.para, &cfg.part)?;

        // construct result
        if cfg.plain {
            ins.content = format!("[\"{}\",{}]", part, ins.content);
        } else {
            ins.content = format!("[\"{}\",\"{}\"]", part, ins.content);
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Setting {
    /// if false add "" around the content.
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    plain: bool,
    /// where to get the part from the `Instance.para` which used to form a key for content
    part: Vec<u8>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn cfg_err() {
        let svc = ParaAsKey {};
        let mut ins = Instance::default();
        let rtn = svc.filter(&mut ins, "").await;
        assert_eq!(rtn.err().unwrap().to_string().contains("ParaAsKey get cfg error"), true)
    }

    #[tokio::test]
    async fn part_not_set() {
        let svc = ParaAsKey {};
        let mut ins = Instance::default();
        let rtn = svc.filter(&mut ins, r#"{"part":[]}"#).await;
        assert_eq!(rtn.err().unwrap().to_string().contains("para_part must be set"), true)
    }

    #[tokio::test]
    async fn part_out_index() {
        let svc = ParaAsKey {};
        let mut ins = Instance::default();
        let rtn = svc.filter(&mut ins, r#"{"part":[1]}"#).await;
        assert_eq!(rtn.err().unwrap().to_string().contains("outbound index"), true)
    }

    #[tokio::test]
    async fn empty_content() {
        let svc = ParaAsKey {};
        let mut ins = Instance::default();
        ins.para = "ll/xx/bb".to_string();
        svc.filter(&mut ins, r#"{"part":[1]}"#).await.unwrap();
        assert_eq!(ins.content, r#"["xx",""]"#)
    }

    #[tokio::test]
    async fn content_has_something() {
        let svc = ParaAsKey {};
        let mut ins = Instance::default();
        ins.content = "happy".to_string();
        ins.para = "ll/xx/bb".to_string();
        svc.filter(&mut ins, r#"{"part":[1]}"#).await.unwrap();
        assert_eq!(ins.content, r#"["xx","happy"]"#)
    }

    #[tokio::test]
    async fn plain_context_empty() {
        let svc = ParaAsKey {};
        let mut ins = Instance::default();
        ins.para = "ll/xx/bb".to_string();
        svc.filter(&mut ins, r#"{"part":[1],"plain":true}"#).await.unwrap();
        assert_eq!(ins.content, r#"["xx",]"#)
    }

    #[tokio::test]
    async fn plain_context_something() {
        let svc = ParaAsKey {};
        let mut ins = Instance::default();
        ins.content = "happy\"day\"".to_string();
        ins.para = "ll/xx/bb".to_string();
        svc.filter(&mut ins, r#"{"part":[1],"plain":true}"#).await.unwrap();
        assert_eq!(ins.content, r#"["xx",happy"day"]"#)
    }
}

