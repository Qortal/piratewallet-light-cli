use zcash_primitives::legacy::Script;
use zcash_primitives::transaction::{
    components::{amount::Amount, orchard, sapling, transparent},
    sighash::TransparentAuthorizingContext,
    Authorization as TxAuthorization,
};

#[derive(Debug, Clone)]
pub struct TransparentAuthContext {
    pub input_amounts: Vec<Amount>,
    pub input_scriptpubkeys: Vec<Script>,
}

impl transparent::Authorization for TransparentAuthContext {
    type ScriptSig = ();
}

impl TransparentAuthorizingContext for TransparentAuthContext {
    fn input_amounts(&self) -> Vec<Amount> {
        self.input_amounts.clone()
    }

    fn input_scriptpubkeys(&self) -> Vec<Script> {
        self.input_scriptpubkeys.clone()
    }
}

#[derive(Debug)]
pub struct CustomUnauthorized;

impl TxAuthorization for CustomUnauthorized {
    type TransparentAuth = TransparentAuthContext;
    type SaplingAuth = sapling::builder::Unauthorized;
    type OrchardAuth = orchard::Unauthorized;

    #[cfg(feature = "zfuture")]
    type TzeAuth = zcash_primitives::transaction::components::tze::builder::Unauthorized;
}

pub fn build_p2pkh_script_sig(sig_bytes: &[u8], pubkey: &[u8]) -> Script {
    Script::default() << sig_bytes << pubkey
}

pub fn build_p2sh_script_sig(sig_bytes: &[u8], secret: &[u8], redeem_script: &[u8]) -> Script {
    if secret.is_empty() {
        let is_refund = [0x51u8];
        Script::default() << sig_bytes << &is_refund[..] << redeem_script
    } else {
        let is_redeem = [0x00u8];
        Script::default() << sig_bytes << secret << &is_redeem[..] << redeem_script
    }
}

pub fn build_script_pubkey(script_bytes: &[u8]) -> Result<Script, String> {
    if script_bytes.is_empty() {
        return Err("Script pubkey bytes are empty".to_string());
    }

    Ok(Script(script_bytes.to_vec()))
}

pub fn build_op_return_script(script_bytes: &[u8]) -> Result<Script, String> {
    if script_bytes.is_empty() {
        return Err("OP_RETURN script payload is empty".to_string());
    }

    if script_bytes.len() > u8::MAX as usize {
        return Err("OP_RETURN script payload is too large".to_string());
    }

    if script_bytes.len() >= 2 && script_bytes[0] == 0x6a && script_bytes[1] == 0x4c {
        return Ok(Script(script_bytes.to_vec()));
    }

    let mut script = Vec::with_capacity(script_bytes.len() + 3);
    script.push(0x6a);
    script.push(0x4c);
    script.push(script_bytes.len() as u8);
    script.extend_from_slice(script_bytes);

    Ok(Script(script))
}
