#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol, Vec, BytesN};

#[contracttype]
#[derive(Clone)]
pub struct Certificate {
    pub recipient: Address,
    pub course_name: String,
    pub issued_at: u64,
    pub metadata_uri: String,
}

#[contracttype]
pub enum DataKey {
    CertificateById(BytesN<32>),
    CertificatesByUser(Address),
}

#[contract]
pub struct CertificateNFTContract;

#[contractimpl]
impl CertificateNFTContract {
    pub fn issue_certificate(
        env: Env,
        to: Address,
        course_name: String,
        metadata_uri: String,
        cert_id: BytesN<32>,
    ) {
        to.require_auth();

        let certificate = Certificate {
            recipient: to.clone(),
            course_name,
            issued_at: env.ledger().timestamp(),
            metadata_uri,
        };

        env.storage().instance().set(&DataKey::CertificateById(cert_id.clone()), &certificate);

        let mut certs: Vec<BytesN<32>> = env
            .storage()
            .instance()
            .get(&DataKey::CertificatesByUser(to.clone()))
            .unwrap_or(Vec::new(&env));

        certs.push_back(cert_id);
        env.storage().instance().set(&DataKey::CertificatesByUser(to), &certs);
    }

    pub fn get_certificate(env: Env, cert_id: BytesN<32>) -> Option<Certificate> {
        env.storage().instance().get(&DataKey::CertificateById(cert_id))
    }

    pub fn list_certificates(env: Env, user: Address) -> Vec<BytesN<32>> {
        env.storage()
            .instance()
            .get(&DataKey::CertificatesByUser(user))
            .unwrap_or(Vec::new(&env))
    }
}
