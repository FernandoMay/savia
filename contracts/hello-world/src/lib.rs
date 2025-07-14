#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, xdr::ToXdr, Address, Bytes, BytesN, Env, String, Vec, Map};

// Enhanced Savia Smart Contracts for Stellar - Mexican Compliance Version
// Implements SEP-24 KYC, medical documentation, and dynamic NFT system

// ========== ENHANCED DATA STRUCTURES ==========

#[derive(Clone)]
#[contracttype]
pub struct Campaign {
    pub id: BytesN<32>,
    pub title: String,
    pub description: String,
    pub beneficiary: Address,
    pub goal_amount: u64,
    pub current_amount: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub verified: bool,
    pub trust_score: u32,
    pub category: String,
    pub location: String,
    pub medical_condition: String,
    pub kyc_verified: bool,
    pub medical_docs_verified: bool,
    pub last_proof_submitted: u64,
    pub proof_deadline: u64,
    pub funds_locked: bool,
    pub etherfuse_account: String,
    pub peso_exchange_rate: u64, // Rate per 1 XLM in Mexican pesos (scaled by 10000)
}

#[derive(Clone)]
#[contracttype]
pub struct Donation {
    pub id: BytesN<32>,
    pub campaign_id: BytesN<32>,
    pub donor: Address,
    pub amount: u64,
    pub peso_amount: u64,
    pub timestamp: u64,
    pub nft_minted: bool,
    pub anonymous: bool,
    pub refunded: bool,
    pub etherfuse_tx_id: String,
}

#[derive(Clone)]
#[contracttype]
pub struct KYCRecord {
    pub entity: Address,
    pub curp: String,
    pub full_name: String,
    pub phone_number: String,
    pub email: String,
    pub address: String,
    pub verification_level: KYCLevel,
    pub verified_at: u64,
    pub expires_at: u64,
    pub medical_license: Option<String>, // For medical professionals
    pub institution: Option<String>,
}

#[derive(Clone)]
#[contracttype]
pub enum KYCLevel {
    Unverified,
    BasicVerified,    // CURP + Phone
    MedicalVerified,  // Medical license + Institution
    FullyVerified,    // All documents + background check
}

#[derive(Clone)]
#[contracttype]
pub struct MedicalDocumentation {
    pub campaign_id: BytesN<32>,
    pub document_type: MedicalDocType,
    pub document_hash: BytesN<32>,
    pub document_url: String,
    pub submitted_at: u64,
    pub verified_by: Option<Address>,
    pub verification_status: DocumentStatus,
    pub expiry_date: u64,
    pub notes: String,
}

#[derive(Clone)]
#[contracttype]
pub enum MedicalDocType {
    MedicalDiagnosis,
    TreatmentPlan,
    MedicalInvoice,
    HospitalBill,
    PharmacyReceipt,
    LabResults,
    DoctorPrescription,
    InsuranceClaimDenial,
}

#[derive(Clone)]
#[contracttype]
pub enum DocumentStatus {
    Pending,
    Verified,
    Rejected,
    Expired,
}

#[derive(Clone)]
#[contracttype]
pub struct DynamicNFT {
    pub id: BytesN<32>,
    pub owner: Address,
    pub campaign_id: BytesN<32>,
    pub tree_level: u32,
    pub total_donated: u64,
    pub donation_count: u32,
    pub created_at: u64,
    pub last_updated: u64,
    pub metadata_uri: String,
    pub growth_stage: TreeGrowthStage,
    pub special_achievements: Vec<String>,
}

#[derive(Clone)]
#[contracttype]
pub enum TreeGrowthStage {
    Seed,        // 0-499 pesos
    Sprout,      // 500-1499 pesos
    Sapling,     // 1500-4999 pesos
    YoungTree,   // 5000-9999 pesos
    MatureTree,  // 10000-24999 pesos
    MightyTree,  // 25000+ pesos
}

#[derive(Clone)]
#[contracttype]
pub struct TrustScore {
    pub entity: Address,
    pub score: u32,
    pub verification_level: u32,
    pub donation_count: u32,
    pub total_donated: u64,
    pub campaigns_created: u32,
    pub medical_docs_submitted: u32,
    pub docs_verified_on_time: u32,
    pub late_submissions: u32,
    pub fraud_reports: u32,
    pub last_updated: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct EtherFuseTransaction {
    pub id: BytesN<32>,
    pub campaign_id: BytesN<32>,
    pub donor: Address,
    pub peso_amount: u64,
    pub xlm_amount: u64,
    pub exchange_rate: u64,
    pub etherfuse_tx_id: String,
    pub status: TransactionStatus,
    pub timestamp: u64,
}

#[derive(Clone)]
#[contracttype]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
    Refunded,
}

// ========== ENHANCED STORAGE KEYS ==========

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Campaign(BytesN<32>),
    Donation(BytesN<32>),
    KYCRecord(Address),
    MedicalDoc(BytesN<32>),
    DynamicNFT(BytesN<32>),
    TrustScore(Address),
    EtherFuseTransaction(BytesN<32>),
    PlatformFee,
    CampaignCounter,
    DonationCounter,
    NFTCounter,
    MedicalDocCounter,
    EtherFuseConfig,
    PesoExchangeRate,
    KYCVerifiers,
    MedicalVerifiers,
}

// ========== ENHANCED ERROR CODES ==========

#[derive(Clone, Copy, Debug, PartialEq)]
#[contracttype]
pub enum SaviaError {
    InvalidFee = 1,
    InvalidGoal = 2,
    InvalidDuration = 3,
    CampaignNotFound = 4,
    CampaignEnded = 5,
    InvalidAmount = 6,
    ScoreExists = 7,
    InsufficientFunds = 8,
    NotApproved = 9,
    KYCNotVerified = 10,
    InvalidCURP = 11,
    InvalidPhoneNumber = 12,
    MedicalDocsExpired = 13,
    ProofDeadlineExceeded = 14,
    FundsLocked = 15,
    EtherFuseError = 16,
    InvalidMedicalDoc = 17,
    NotAuthorized = 18,
    DocumentExpired = 19,
    RefundPeriodExpired = 20,
}

// ========== ENHANCED MAIN CONTRACT ==========

#[contract]
pub struct SaviaContract;

#[contractimpl]
impl SaviaContract {
    
    /// Initialize the contract with enhanced Mexican compliance
    pub fn initialize(
        env: Env,
        platform_fee: u64,
        etherfuse_config: String,
        initial_peso_rate: u64,
    ) -> Result<(), soroban_sdk::Error> {
        if platform_fee > 1000 {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::InvalidFee as u32));
        }
        
        env.storage().instance().set(&DataKey::PlatformFee, &platform_fee);
        env.storage().instance().set(&DataKey::CampaignCounter, &0u64);
        env.storage().instance().set(&DataKey::DonationCounter, &0u64);
        env.storage().instance().set(&DataKey::NFTCounter, &0u64);
        env.storage().instance().set(&DataKey::MedicalDocCounter, &0u64);
        env.storage().instance().set(&DataKey::EtherFuseConfig, &etherfuse_config);
        env.storage().instance().set(&DataKey::PesoExchangeRate, &initial_peso_rate);
        
        // Initialize verifier lists
        let empty_vec: Vec<Address> = Vec::new(&env);
        env.storage().instance().set(&DataKey::KYCVerifiers, &empty_vec);
        env.storage().instance().set(&DataKey::MedicalVerifiers, &empty_vec);
        
        Ok(())
    }

    /// Register KYC information with Mexican CURP validation
    pub fn register_kyc(
        env: Env,
        entity: Address,
        curp: String,
        full_name: String,
        phone_number: String,
        email: String,
        address: String,
        medical_license: Option<String>,
        institution: Option<String>,
    ) -> Result<(), soroban_sdk::Error> {
        // Validate CURP format (18 characters)
        if curp.len() != 18 {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::InvalidCURP as u32));
        }

        // Validate Mexican phone number format (10 digits)
        if phone_number.len() != 10 {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::InvalidPhoneNumber as u32));
        }

        let current_time = env.ledger().timestamp();
        let expiry_time = current_time + (365 * 24 * 60 * 60); // 1 year validity

        let verification_level = match (&medical_license, &institution) {
            (Some(_), Some(_)) => KYCLevel::MedicalVerified,
            _ => KYCLevel::BasicVerified,
        };

        let kyc_record = KYCRecord {
            entity: entity.clone(),
            curp,
            full_name,
            phone_number,
            email,
            address,
            verification_level,
            verified_at: current_time,
            expires_at: expiry_time,
            medical_license,
            institution,
        };

        env.storage().persistent().set(&DataKey::KYCRecord(entity), &kyc_record);
        Ok(())
    }

    /// Create a new campaign with enhanced medical requirements
    pub fn create_campaign(
        env: Env,
        beneficiary: Address,
        title: String,
        description: String,
        medical_condition: String,
        goal_amount: u64,
        duration_days: u64,
        category: String,
        location: String,
        etherfuse_account: String,
    ) -> Result<BytesN<32>, soroban_sdk::Error> {
        // Validate KYC verification
        let kyc_record: KYCRecord = env.storage().persistent().get(&DataKey::KYCRecord(beneficiary.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::KYCNotVerified as u32))?;

        if kyc_record.expires_at < env.ledger().timestamp() {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::KYCNotVerified as u32));
        }

        // Validate inputs
        if goal_amount == 0 {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::InvalidGoal as u32));
        }
        
        if duration_days == 0 || duration_days > 365 {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::InvalidDuration as u32));
        }

        // Get current peso exchange rate
        let peso_rate: u64 = env.storage().instance().get(&DataKey::PesoExchangeRate).unwrap_or(180000); // Default ~18 pesos per XLM

        // Get and increment campaign counter
        let counter: u64 = env.storage().instance().get(&DataKey::CampaignCounter).unwrap_or(0);
        let new_counter = counter + 1;
        env.storage().instance().set(&DataKey::CampaignCounter, &new_counter);

        // Generate campaign ID
        let current_time = env.ledger().timestamp();
        let mut hash_input = Bytes::new(&env);
        
        let beneficiary_bytes = beneficiary.clone().to_xdr(&env);
        let title_bytes = title.to_val().to_xdr(&env);
               
        hash_input.append(&beneficiary_bytes);
        hash_input.append(&title_bytes);
        hash_input.append(&Bytes::from_slice(&env, &goal_amount.to_be_bytes()));
        hash_input.append(&Bytes::from_slice(&env, &current_time.to_be_bytes()));
        hash_input.append(&Bytes::from_slice(&env, &new_counter.to_be_bytes()));
        
        let campaign_id: BytesN<32> = env.crypto().sha256(&hash_input).into();

        let end_time = current_time + (duration_days * 24 * 60 * 60);
        let proof_deadline = current_time + (30 * 24 * 60 * 60); // 30 days for first proof

        let campaign = Campaign {
            id: campaign_id.clone(),
            title,
            description,
            beneficiary,
            goal_amount,
            current_amount: 0,
            start_time: current_time,
            end_time,
            verified: false,
            trust_score: 0,
            category,
            location,
            medical_condition,
            kyc_verified: true,
            medical_docs_verified: false,
            last_proof_submitted: 0,
            proof_deadline,
            funds_locked: false,
            etherfuse_account,
            peso_exchange_rate: peso_rate,
        };

        env.storage().persistent().set(&DataKey::Campaign(campaign_id.clone()), &campaign);
        Ok(campaign_id)
    }

    /// Submit medical documentation
    pub fn submit_medical_documentation(
        env: Env,
        campaign_id: BytesN<32>,
        document_type: MedicalDocType,
        document_url: String,
        notes: String,
    ) -> Result<BytesN<32>, soroban_sdk::Error> {
        let mut campaign: Campaign = env.storage().persistent().get(&DataKey::Campaign(campaign_id.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::CampaignNotFound as u32))?;

        // Get and increment medical doc counter
        let counter: u64 = env.storage().instance().get(&DataKey::MedicalDocCounter).unwrap_or(0);
        let new_counter = counter + 1;
        env.storage().instance().set(&DataKey::MedicalDocCounter, &new_counter);

        // Generate document hash from URL and content
        let mut hash_input = Bytes::new(&env);
        hash_input.append(&Bytes::from_slice(&env, campaign_id.to_array().as_slice()));
        hash_input.append(&document_url.to_val().to_xdr(&env));
        hash_input.append(&Bytes::from_slice(&env, &new_counter.to_be_bytes()));
        
        let document_hash: BytesN<32> = env.crypto().sha256(&hash_input).into();

        let current_time = env.ledger().timestamp();
        let expiry_date = current_time + (90 * 24 * 60 * 60); // 90 days validity

        let medical_doc = MedicalDocumentation {
            campaign_id: campaign_id.clone(),
            document_type,
            document_hash: document_hash.clone(),
            document_url,
            submitted_at: current_time,
            verified_by: None,
            verification_status: DocumentStatus::Pending,
            expiry_date,
            notes,
        };

        // Update campaign
        campaign.last_proof_submitted = current_time;
        campaign.proof_deadline = current_time + (30 * 24 * 60 * 60); // Reset 30-day deadline
        campaign.funds_locked = false;

        env.storage().persistent().set(&DataKey::Campaign(campaign_id), &campaign);
        env.storage().persistent().set(&DataKey::MedicalDoc(document_hash.clone()), &medical_doc);

        Ok(document_hash)
    }

    /// Verify medical documentation (admin function)
    pub fn verify_medical_documentation(
        env: Env,
        document_hash: BytesN<32>,
        verifier: Address,
        approved: bool,
    ) -> Result<(), soroban_sdk::Error> {
        // Check if verifier is authorized
        let verifiers: Vec<Address> = env.storage().instance().get(&DataKey::MedicalVerifiers).unwrap_or(Vec::new(&env));
        if !verifiers.contains(&verifier) {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32));
        }

        let mut medical_doc: MedicalDocumentation = env.storage().persistent().get(&DataKey::MedicalDoc(document_hash.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::InvalidMedicalDoc as u32))?;

        medical_doc.verified_by = Some(verifier);
        medical_doc.verification_status = if approved { DocumentStatus::Verified } else { DocumentStatus::Rejected };

        // Update campaign verification status
        let mut campaign: Campaign = env.storage().persistent().get(&DataKey::Campaign(medical_doc.campaign_id.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::CampaignNotFound as u32))?;

        if approved {
            campaign.medical_docs_verified = true;
            campaign.verified = true;
        }

        env.storage().persistent().set(&DataKey::MedicalDoc(document_hash), &medical_doc);
        env.storage().persistent().set(&DataKey::Campaign(medical_doc.campaign_id), &campaign);

        Ok(())
    }

    /// Process donation with peso conversion and dynamic NFT
    pub fn donate(
        env: Env,
        campaign_id: BytesN<32>,
        donor: Address,
        xlm_amount: u64,
        anonymous: bool,
        mint_nft: bool,
    ) -> Result<BytesN<32>, soroban_sdk::Error> {
        // Validate campaign exists and is active
        let mut campaign: Campaign = env.storage().persistent().get(&DataKey::Campaign(campaign_id.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::CampaignNotFound as u32))?;

        let current_time = env.ledger().timestamp();
        
        // Check if campaign has ended
        if current_time > campaign.end_time {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::CampaignEnded as u32));
        }

        // Check if funds are locked due to missing documentation
        if campaign.funds_locked {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::FundsLocked as u32));
        }

        // Check if medical documentation is expired
        if campaign.last_proof_submitted > 0 && (current_time > campaign.proof_deadline) {
            campaign.funds_locked = true;
            env.storage().persistent().set(&DataKey::Campaign(campaign_id.clone()), &campaign);
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::ProofDeadlineExceeded as u32));
        }

        if xlm_amount == 0 {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::InvalidAmount as u32));
        }

        // Convert XLM to pesos
        let peso_amount = (xlm_amount * campaign.peso_exchange_rate) / 10000;

        // Get platform fee
        let platform_fee_rate: u64 = env.storage().instance().get(&DataKey::PlatformFee).unwrap_or(200);
        let platform_fee = (xlm_amount * platform_fee_rate) / 10000;
        let net_xlm_amount = xlm_amount - platform_fee;
        let net_peso_amount = (net_xlm_amount * campaign.peso_exchange_rate) / 10000;

        // Generate donation ID
        let counter: u64 = env.storage().instance().get(&DataKey::DonationCounter).unwrap_or(0);
        let new_counter = counter + 1;
        env.storage().instance().set(&DataKey::DonationCounter, &new_counter);

        let mut hash_input = Bytes::new(&env);
        hash_input.append(&Bytes::from_slice(&env, campaign_id.to_array().as_slice()));
        hash_input.append(&donor.clone().to_xdr(&env));
        hash_input.append(&Bytes::from_slice(&env, &xlm_amount.to_be_bytes()));
        hash_input.append(&Bytes::from_slice(&env, &current_time.to_be_bytes()));
        hash_input.append(&Bytes::from_slice(&env, &new_counter.to_be_bytes()));
        
        let donation_id: BytesN<32> = env.crypto().sha256(&hash_input).into();

        // Create EtherFuse transaction
        let etherfuse_tx_id = Self::create_etherfuse_transaction(
            env.clone(),
            campaign_id.clone(),
            donor.clone(),
            net_peso_amount,
            net_xlm_amount,
            campaign.peso_exchange_rate,
            campaign.etherfuse_account.clone(),
        )?;

        // Create donation record
        let donation = Donation {
            id: donation_id.clone(),
            campaign_id: campaign_id.clone(),
            donor: donor.clone(),
            amount: net_xlm_amount,
            peso_amount: net_peso_amount,
            timestamp: current_time,
            nft_minted: mint_nft,
            anonymous,
            refunded: false,
            etherfuse_tx_id: etherfuse_tx_id.clone(),
        };

        // Update campaign progress
        campaign.current_amount += net_xlm_amount;
        env.storage().persistent().set(&DataKey::Campaign(campaign_id.clone()), &campaign);

        // Store donation
        env.storage().persistent().set(&DataKey::Donation(donation_id.clone()), &donation);

        // Update trust score
        Self::update_donor_trust_score(env.clone(), donor.clone(), net_peso_amount)?;

        // Handle dynamic NFT
        if mint_nft {
            Self::mint_or_update_dynamic_nft(env.clone(), donor, campaign_id, net_peso_amount)?;
        }

        Ok(donation_id)
    }

    /// Create EtherFuse transaction for peso conversion
    fn create_etherfuse_transaction(
        env: Env,
        campaign_id: BytesN<32>,
        donor: Address,
        peso_amount: u64,
        xlm_amount: u64,
        exchange_rate: u64,
        etherfuse_account: String,
    ) -> Result<String, soroban_sdk::Error> {
        // Generate transaction hash
        let mut hash_input = Bytes::new(&env);
        hash_input.append(&Bytes::from_slice(&env, campaign_id.to_array().as_slice()));
        hash_input.append(&donor.to_xdr(&env));
        hash_input.append(&Bytes::from_slice(&env, &peso_amount.to_be_bytes()));
        hash_input.append(&Bytes::from_slice(&env, &env.ledger().timestamp().to_be_bytes()));
        
        let tx_id: BytesN<32> = env.crypto().sha256(&hash_input).into();

        let etherfuse_tx = EtherFuseTransaction {
            id: tx_id.clone(),
            campaign_id,
            donor,
            peso_amount,
            xlm_amount,
            exchange_rate,
            etherfuse_tx_id: String::from_str(&env, "ETF_"),
            status: TransactionStatus::Pending,
            timestamp: env.ledger().timestamp(),
        };

        env.storage().persistent().set(&DataKey::EtherFuseTransaction(tx_id), &etherfuse_tx);
        
        // Return simulated EtherFuse transaction ID
        Ok(String::from_str(&env, "ETF_TX_123456"))
    }

    /// Mint or update dynamic NFT tree
    fn mint_or_update_dynamic_nft(
        env: Env,
        donor: Address,
        campaign_id: BytesN<32>,
        peso_amount: u64,
    ) -> Result<(), soroban_sdk::Error> {
        // Try to find existing NFT for this donor-campaign pair
        let mut existing_nft: Option<DynamicNFT> = None;
        
        // In a real implementation, we'd search through NFTs
        // For now, we'll create a simple key based on donor + campaign
        let mut nft_key_input = Bytes::new(&env);
        nft_key_input.append(&donor.to_xdr(&env));
        nft_key_input.append(&Bytes::from_slice(&env, campaign_id.to_array().as_slice()));
        let nft_key: BytesN<32> = env.crypto().sha256(&nft_key_input).into();

        // Check if NFT already exists
        if let Some(mut nft) = env.storage().persistent().get::<DataKey, DynamicNFT>(&DataKey::DynamicNFT(nft_key.clone())) {
            // Update existing NFT
            nft.total_donated += peso_amount;
            nft.donation_count += 1;
            nft.last_updated = env.ledger().timestamp();
            
            // Update growth stage based on total donated
            nft.growth_stage = Self::calculate_growth_stage(nft.total_donated);
            
            // Check for achievements
            if nft.donation_count == 10 {
                nft.special_achievements.push_back(String::from_str(&env, "Consistent Supporter"));
            }
            if nft.total_donated >= 25000 {
                nft.special_achievements.push_back(String::from_str(&env, "Major Donor"));
            }

            env.storage().persistent().set(&DataKey::DynamicNFT(nft_key), &nft);
        } else {
            // Create new NFT
            let current_time = env.ledger().timestamp();
            let growth_stage = Self::calculate_growth_stage(peso_amount);
            
            let nft = DynamicNFT {
                id: nft_key.clone(),
                owner: donor,
                campaign_id,
                tree_level: 1,
                total_donated: peso_amount,
                donation_count: 1,
                created_at: current_time,
                last_updated: current_time,
                metadata_uri: String::from_str(&env, "https://drive.google.com/file/d/1RadoLAjnG00YPC3F2PNnB49PfuMorNZ8/view"),
                growth_stage,
                special_achievements: Vec::new(&env),
            };

            env.storage().persistent().set(&DataKey::DynamicNFT(nft_key), &nft);
        }

        Ok(())
    }

    /// Calculate tree growth stage based on total donated pesos
    fn calculate_growth_stage(total_pesos: u64) -> TreeGrowthStage {
        match total_pesos {
            0..=499 => TreeGrowthStage::Seed,
            500..=1499 => TreeGrowthStage::Sprout,
            1500..=4999 => TreeGrowthStage::Sapling,
            5000..=9999 => TreeGrowthStage::YoungTree,
            10000..=24999 => TreeGrowthStage::MatureTree,
            _ => TreeGrowthStage::MightyTree,
        }
    }

    /// Check and process expired proof deadlines
    pub fn check_proof_deadlines(env: Env, campaign_id: BytesN<32>) -> Result<(), soroban_sdk::Error> {
        let mut campaign: Campaign = env.storage().persistent().get(&DataKey::Campaign(campaign_id.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::CampaignNotFound as u32))?;

        let current_time = env.ledger().timestamp();

        // Check if 30 days have passed without proof submission
        if campaign.last_proof_submitted > 0 && current_time > campaign.proof_deadline {
            campaign.funds_locked = true;
            
            // Reduce trust score for late submission
            if let Some(mut trust_score) = env.storage().persistent().get::<DataKey, TrustScore>(&DataKey::TrustScore(campaign.beneficiary.clone())) {
                trust_score.late_submissions += 1;
                trust_score.score = if trust_score.score > 20 { trust_score.score - 20 } else { 0 };
                env.storage().persistent().set(&DataKey::TrustScore(campaign.beneficiary.clone()), &trust_score);
            }

            // TODO: Implement refund logic for donations
            Self::initiate_refund_process(env.clone(), campaign_id.clone())?;
        }

        env.storage().persistent().set(&DataKey::Campaign(campaign_id), &campaign);
        Ok(())
    }

    /// Initiate refund process for donors
    fn initiate_refund_process(env: Env, campaign_id: BytesN<32>) -> Result<(), soroban_sdk::Error> {
        // In a real implementation, this would iterate through all donations
        // and initiate refunds through EtherFuse
        
        // For now, we'll just mark the campaign as requiring refunds
        // The actual refund logic would be implemented separately
        Ok(())
    }

    /// Update peso exchange rate
    pub fn update_peso_exchange_rate(env: Env, new_rate: u64) -> Result<(), soroban_sdk::Error> {
        env.storage().instance().set(&DataKey::PesoExchangeRate, &new_rate);
        Ok(())
    }

    /// Add KYC verifier
    pub fn add_kyc_verifier(env: Env, verifier: Address) -> Result<(), soroban_sdk::Error> {
        let mut verifiers: Vec<Address> = env.storage().instance().get(&DataKey::KYCVerifiers).unwrap_or(Vec::new(&env));
        if !verifiers.contains(&verifier) {
            verifiers.push_back(verifier);
            env.storage().instance().set(&DataKey::KYCVerifiers, &verifiers);
        }
        Ok(())
    }

    /// Add medical verifier
    pub fn add_medical_verifier(env: Env, verifier: Address) -> Result<(), soroban_sdk::Error> {
        let mut verifiers: Vec<Address> = env.storage().instance().get(&DataKey::MedicalVerifiers).unwrap_or(Vec::new(&env));
        if !verifiers.contains(&verifier) {
            verifiers.push_back(verifier);
            env.storage().instance().set(&DataKey::MedicalVerifiers, &verifiers);
        }
        Ok(())
    }

    /// Get campaign details
    pub fn get_campaign(env: Env, campaign_id: BytesN<32>) -> Option<Campaign> {
        env.storage().persistent().get(&DataKey::Campaign(campaign_id))
    }

    /// Get donation details
    pub fn get_donation(env: Env, donation_id: BytesN<32>) -> Option<Donation> {
        env.storage().persistent().get(&DataKey::Donation(donation_id))
    }

    /// Get KYC record
    pub fn get_kyc_record(env: Env, entity: Address) -> Option<KYCRecord> {
        env.storage().persistent().get(&DataKey::KYCRecord(entity))
    }

    /// Get medical documentation
    pub fn get_medical_documentation(env: Env, doc_hash: BytesN<32>) -> Option<MedicalDocumentation> {
        env.storage().persistent().get(&DataKey::MedicalDoc(doc_hash))
    }

    /// Get dynamic NFT
    pub fn get_dynamic_nft(env: Env, nft_id: BytesN<32>) -> Option<DynamicNFT> {
        env.storage().persistent().get(&DataKey::DynamicNFT(nft_id))
    }

    /// Get donor's NFT for a specific campaign
    pub fn get_donor_nft(env: Env, donor: Address, campaign_id: BytesN<32>) -> Option<DynamicNFT> {
        let mut nft_key_input = Bytes::new(&env);
        nft_key_input.append(&donor.to_xdr(&env));
        nft_key_input.append(&Bytes::from_slice(&env, campaign_id.to_array().as_slice()));
        let nft_key: BytesN<32> = env.crypto().sha256(&nft_key_input).into();
        
        env.storage().persistent().get(&DataKey::DynamicNFT(nft_key))
    }

    /// Get trust score
    pub fn get_trust_score(env: Env, entity: Address) -> Option<TrustScore> {
        env.storage().persistent().get(&DataKey::TrustScore(entity))
    }

    /// Get EtherFuse transaction
    pub fn get_etherfuse_transaction(env: Env, tx_id: BytesN<32>) -> Option<EtherFuseTransaction> {
        env.storage().persistent().get(&DataKey::EtherFuseTransaction(tx_id))
    }

    /// Initialize enhanced trust score
    pub fn initialize_trust_score(env: Env, entity: Address) -> Result<(), soroban_sdk::Error> {
        if env.storage().persistent().has(&DataKey::TrustScore(entity.clone())) {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::ScoreExists as u32));
        }

        let trust_score = TrustScore {
            entity: entity.clone(),
            score: 50, // Start with neutral score
            verification_level: 0,
            donation_count: 0,
            total_donated: 0,
            campaigns_created: 0,
            medical_docs_submitted: 0,
            docs_verified_on_time: 0,
            late_submissions: 0,
            fraud_reports: 0,
            last_updated: env.ledger().timestamp(),
        };

        env.storage().persistent().set(&DataKey::TrustScore(entity), &trust_score);
        Ok(())
    }

    /// Enhanced trust score update
    fn update_donor_trust_score(env: Env, donor: Address, peso_amount: u64) -> Result<(), soroban_sdk::Error> {
        let mut trust_score: TrustScore = env.storage().persistent().get(&DataKey::TrustScore(donor.clone()))
            .unwrap_or(TrustScore {
                entity: donor.clone(),
                score: 50,
                verification_level: 0,
                donation_count: 0,
                total_donated: 0,
                campaigns_created: 0,
                medical_docs_submitted: 0,
                docs_verified_on_time: 0,
                late_submissions: 0,
                fraud_reports: 0,
                last_updated: env.ledger().timestamp(),
            });

        trust_score.donation_count += 1;
        trust_score.total_donated += peso_amount;
        trust_score.last_updated = env.ledger().timestamp();

        // Enhanced scoring algorithm
        let donation_factor = if trust_score.donation_count > 100 { 100 } else { trust_score.donation_count };
        let amount_factor = if trust_score.total_donated > 100000 { 100000 } else { trust_score.total_donated };
        let consistency_factor = if trust_score.donation_count > 1 { 120u64 } else { 100u64 };
        let verification_bonus = match trust_score.verification_level {
            0 => 0,
            1 => 10,
            2 => 20,
            _ => 30,
        };

        // Calculate new score with peso amounts
        let base_score = 50u64;
        let donation_component = (25u64 * donation_factor as u64) / 100u64;
        let amount_component = (20u64 * amount_factor) / 100000u64;
        let consistency_component = (consistency_factor * 5u64) / 100u64;
        
        let new_score = base_score + donation_component + amount_component + consistency_component + verification_bonus as u64;
        
        // Apply penalties
        let penalty = (trust_score.late_submissions * 10) + (trust_score.fraud_reports * 30);
        let final_score = if new_score > penalty as u64 { new_score - penalty as u64 } else { 0 };
        
        trust_score.score = if final_score > 100 { 100 } else { final_score as u32 };

        env.storage().persistent().set(&DataKey::TrustScore(donor), &trust_score);
        Ok(())
    }

    /// Report fraud
    pub fn report_fraud(env: Env, reported_entity: Address, reporter: Address) -> Result<(), soroban_sdk::Error> {
        // Verify reporter has some trust score
        let reporter_trust: TrustScore = env.storage().persistent().get(&DataKey::TrustScore(reporter))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32))?;

        if reporter_trust.score < 30 {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32));
        }

        // Update reported entity's trust score
        if let Some(mut trust_score) = env.storage().persistent().get::<DataKey, TrustScore>(&DataKey::TrustScore(reported_entity.clone())) {
            trust_score.fraud_reports += 1;
            trust_score.score = if trust_score.score > 30 { trust_score.score - 30 } else { 0 };
            env.storage().persistent().set(&DataKey::TrustScore(reported_entity), &trust_score);
        }

        Ok(())
    }

    /// Process refund for donation
    pub fn process_refund(env: Env, donation_id: BytesN<32>) -> Result<(), soroban_sdk::Error> {
        let mut donation: Donation = env.storage().persistent().get(&DataKey::Donation(donation_id.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::CampaignNotFound as u32))?;

        if donation.refunded {
            return Ok(()); // Already refunded
        }

        let campaign: Campaign = env.storage().persistent().get(&DataKey::Campaign(donation.campaign_id.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::CampaignNotFound as u32))?;

        let current_time = env.ledger().timestamp();
        let refund_deadline = campaign.proof_deadline + (7 * 24 * 60 * 60); // 7 days after proof deadline

        if current_time > refund_deadline {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::RefundPeriodExpired as u32));
        }

        // Mark donation as refunded
        donation.refunded = true;
        env.storage().persistent().set(&DataKey::Donation(donation_id), &donation);

        // Update EtherFuse transaction status
        // In real implementation, this would trigger actual refund through EtherFuse
        
        Ok(())
    }

    /// Get campaign statistics
    pub fn get_campaign_stats(env: Env, campaign_id: BytesN<32>) -> Result<Map<String, u64>, soroban_sdk::Error> {
        let campaign: Campaign = env.storage().persistent().get(&DataKey::Campaign(campaign_id))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::CampaignNotFound as u32))?;

        let mut stats = Map::new(&env);
        stats.set(String::from_str(&env, "goal_amount"), campaign.goal_amount);
        stats.set(String::from_str(&env, "current_amount"), campaign.current_amount);
        stats.set(String::from_str(&env, "peso_goal"), (campaign.goal_amount * campaign.peso_exchange_rate) / 10000);
        stats.set(String::from_str(&env, "peso_raised"), (campaign.current_amount * campaign.peso_exchange_rate) / 10000);
        stats.set(String::from_str(&env, "days_remaining"), 
            if env.ledger().timestamp() < campaign.end_time { 
                (campaign.end_time - env.ledger().timestamp()) / (24 * 60 * 60) 
            } else { 0 }
        );
        stats.set(String::from_str(&env, "trust_score"), campaign.trust_score as u64);
        stats.set(String::from_str(&env, "kyc_verified"), if campaign.kyc_verified { 1 } else { 0 });
        stats.set(String::from_str(&env, "medical_verified"), if campaign.medical_docs_verified { 1 } else { 0 });
        stats.set(String::from_str(&env, "funds_locked"), if campaign.funds_locked { 1 } else { 0 });

        Ok(stats)
    }

    /// Get donor dashboard data
    pub fn get_donor_dashboard(env: Env, donor: Address) -> Result<Map<String, u64>, soroban_sdk::Error> {
        let mut dashboard = Map::new(&env);

        // Get trust score
        if let Some(trust_score) = env.storage().persistent().get::<DataKey, TrustScore>(&DataKey::TrustScore(donor.clone())) {
            dashboard.set(String::from_str(&env, "trust_score"), trust_score.score as u64);
            dashboard.set(String::from_str(&env, "total_donated"), trust_score.total_donated);
            dashboard.set(String::from_str(&env, "donation_count"), trust_score.donation_count as u64);
        }

        // Get KYC status
        if let Some(kyc_record) = env.storage().persistent().get::<DataKey, KYCRecord>(&DataKey::KYCRecord(donor.clone())) {
            let kyc_level = match kyc_record.verification_level {
                KYCLevel::Unverified => 0,
                KYCLevel::BasicVerified => 1,
                KYCLevel::MedicalVerified => 2,
                KYCLevel::FullyVerified => 3,
            };
            dashboard.set(String::from_str(&env, "kyc_level"), kyc_level);
            dashboard.set(String::from_str(&env, "kyc_expires"), kyc_record.expires_at);
        }

        Ok(dashboard)
    }

    /// Verify campaign (admin function)
    pub fn verify_campaign(
        env: Env,
        campaign_id: BytesN<32>,
        trust_score: u32,
        verifier: Address,
    ) -> Result<(), soroban_sdk::Error> {
        // Check if verifier is authorized
        let verifiers: Vec<Address> = env.storage().instance().get(&DataKey::KYCVerifiers).unwrap_or(Vec::new(&env));
        if !verifiers.contains(&verifier) {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32));
        }

        let mut campaign: Campaign = env.storage().persistent().get(&DataKey::Campaign(campaign_id.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::CampaignNotFound as u32))?;

        campaign.verified = true;
        campaign.trust_score = trust_score;

        env.storage().persistent().set(&DataKey::Campaign(campaign_id), &campaign);
        Ok(())
    }

    /// Get current peso exchange rate
    pub fn get_peso_exchange_rate(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::PesoExchangeRate).unwrap_or(180000)
    }

    /// Emergency pause campaign
    pub fn emergency_pause_campaign(env: Env, campaign_id: BytesN<32>) -> Result<(), soroban_sdk::Error> {
        let mut campaign: Campaign = env.storage().persistent().get(&DataKey::Campaign(campaign_id.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::CampaignNotFound as u32))?;

        campaign.funds_locked = true;
        env.storage().persistent().set(&DataKey::Campaign(campaign_id), &campaign);
        Ok(())
    }

    /// Resume campaign after emergency pause
    pub fn resume_campaign(env: Env, campaign_id: BytesN<32>) -> Result<(), soroban_sdk::Error> {
        let mut campaign: Campaign = env.storage().persistent().get(&DataKey::Campaign(campaign_id.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::CampaignNotFound as u32))?;

        // Only resume if medical docs are verified and within deadline
        if campaign.medical_docs_verified && env.ledger().timestamp() <= campaign.proof_deadline {
            campaign.funds_locked = false;
            env.storage().persistent().set(&DataKey::Campaign(campaign_id), &campaign);
        }
        Ok(())
    }
}

// ========== TESTS ==========

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn test_initialize_enhanced_contract() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SaviaContract);
        let client = SaviaContractClient::new(&env, &contract_id);

        let result = client.initialize(
            &200,
            &String::from_str(&env, "etherfuse_config"),
            &180000,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_kyc_registration() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SaviaContract);
        let client = SaviaContractClient::new(&env, &contract_id);

        client.initialize(&200, &String::from_str(&env, "etherfuse_config"), &180000);

        let user = Address::generate(&env);
        let result = client.register_kyc(
            &user,
            &String::from_str(&env, "ABCD123456HDFGHI01"), // Valid CURP format
            &String::from_str(&env, "Juan Pérez"),
            &String::from_str(&env, "5551234567"), // Valid Mexican phone
            &String::from_str(&env, "juan@example.com"),
            &String::from_str(&env, "Mexico City"),
            &None,
            &None,
        );

        assert!(result.is_ok());
        let kyc_record = client.get_kyc_record(&user);
        assert!(kyc_record.is_some());
    }

    #[test]
    fn test_enhanced_campaign_creation() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SaviaContract);
        let client = SaviaContractClient::new(&env, &contract_id);

        client.initialize(&200, &String::from_str(&env, "etherfuse_config"), &180000);

        let beneficiary = Address::generate(&env);
        
        // Register KYC first
        client.register_kyc(
            &beneficiary,
            &String::from_str(&env, "ABCD123456HDFGHI01"),
            &String::from_str(&env, "Maria González"),
            &String::from_str(&env, "5551234567"),
            &String::from_str(&env, "maria@example.com"),
            &String::from_str(&env, "Guadalajara"),
            &None,
            &None,
        ).unwrap();

        let result = client.create_campaign(
            &beneficiary,
            &String::from_str(&env, "Tratamiento de Cáncer"),
            &String::from_str(&env, "Necesito ayuda para mi tratamiento"),
            &String::from_str(&env, "Cáncer de mama"),
            &500000, // 50,000 pesos goal
            &60,
            &String::from_str(&env, "Salud"),
            &String::from_str(&env, "Guadalajara"),
            &String::from_str(&env, "ETF_ACCOUNT_123"),
        );

        assert!(result.is_ok());
        let campaign_id = result.unwrap();
        let campaign = client.get_campaign(&campaign_id);
        assert!(campaign.is_some());
        assert!(campaign.unwrap().kyc_verified);
    }

    #[test]
    fn test_donation_with_peso_conversion() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SaviaContract);
        let client = SaviaContractClient::new(&env, &contract_id);

        client.initialize(&200, &String::from_str(&env, "etherfuse_config"), &180000);

        let beneficiary = Address::generate(&env);
        let donor = Address::generate(&env);

        // Register KYC for beneficiary
        client.register_kyc(
            &beneficiary,
            &String::from_str(&env, "ABCD123456HDFGHI01"),
            &String::from_str(&env, "Maria González"),
            &String::from_str(&env, "5551234567"),
            &String::from_str(&env, "maria@example.com"),
            &String::from_str(&env, "Guadalajara"),
            &None,
            &None,
        ).unwrap();

        // Create campaign
        let campaign_id = client.create_campaign(
            &beneficiary,
            &String::from_str(&env, "Tratamiento Médico"),
            &String::from_str(&env, "Ayuda médica urgente"),
            &String::from_str(&env, "Cirugía"),
            &500000,
            &60,
            &String::from_str(&env, "Salud"),
            &String::from_str(&env, "Mexico City"),
            &String::from_str(&env, "ETF_ACCOUNT_123"),
        ).unwrap();

        // Make donation (1000 XLM = 18,000 pesos at rate 180000)
        let donation_id = client.donate(
            &campaign_id,
            &donor,
            &10000000, // 1000 XLM (in stroops)
            &false,
            &true,
        ).unwrap();

        // Verify donation
        let donation = client.get_donation(&donation_id);
        assert!(donation.is_some());
        let donation_data = donation.unwrap();
        assert_eq!(donation_data.peso_amount, 176400000); // 1000 XLM * 18 pesos * 0.98 (fee) * 10000 (scaling)
    }

    #[test]
    fn test_dynamic_nft_growth() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SaviaContract);
        let client = SaviaContractClient::new(&env, &contract_id);

        client.initialize(&200, &String::from_str(&env, "etherfuse_config"), &180000);

        let beneficiary = Address::generate(&env);
        let donor = Address::generate(&env);

        // Setup campaign
        client.register_kyc(
            &beneficiary,
            &String::from_str(&env, "ABCD123456HDFGHI01"),
            &String::from_str(&env, "Maria González"),
            &String::from_str(&env, "5551234567"),
            &String::from_str(&env, "maria@example.com"),
            &String::from_str(&env, "Guadalajara"),
            &None,
            &None,
        ).unwrap();

        let campaign_id = client.create_campaign(
            &beneficiary,
            &String::from_str(&env, "Tratamiento"),
            &String::from_str(&env, "Ayuda médica"),
            &String::from_str(&env, "Cirugía"),
            &500000,
            &60,
            &String::from_str(&env, "Salud"),
            &String::from_str(&env, "Mexico City"),
            &String::from_str(&env, "ETF_ACCOUNT_123"),
        ).unwrap();

        // Make small donation (seed stage)
        client.donate(&campaign_id, &donor, &555556, &false, &true).unwrap(); // ~100 pesos

        let nft = client.get_donor_nft(&donor, &campaign_id);
        assert!(nft.is_some());
        let nft_data = nft.unwrap();
        assert_eq!(nft_data.growth_stage, TreeGrowthStage::Seed);

        // Make larger donation (sprout stage)
        client.donate(&campaign_id, &donor, &2777778, &false, &true).unwrap(); // ~500 pesos

        let updated_nft = client.get_donor_nft(&donor, &campaign_id);
        assert!(updated_nft.is_some());
        let updated_nft_data = updated_nft.unwrap();
        assert_eq!(updated_nft_data.growth_stage, TreeGrowthStage::Sprout);
        assert_eq!(updated_nft_data.donation_count, 2);
    }

    #[test]
    fn test_medical_documentation_flow() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SaviaContract);
        let client = SaviaContractClient::new(&env, &contract_id);

        client.initialize(&200, &String::from_str(&env, "etherfuse_config"), &180000);

        let beneficiary = Address::generate(&env);
        let verifier = Address::generate(&env);

        // Add medical verifier
        client.add_medical_verifier(&verifier);

        // Register KYC
        client.register_kyc(
            &beneficiary,
            &String::from_str(&env, "ABCD123456HDFGHI01"),
            &String::from_str(&env, "Maria González"),
            &String::from_str(&env, "5551234567"),
            &String::from_str(&env, "maria@example.com"),
            &String::from_str(&env, "Guadalajara"),
            &None,
            &None,
        ).unwrap();

        // Create campaign
        let campaign_id = client.create_campaign(
            &beneficiary,
            &String::from_str(&env, "Tratamiento"),
            &String::from_str(&env, "Ayuda médica"),
            &String::from_str(&env, "Cáncer"),
            &500000,
            &60,
            &String::from_str(&env, "Salud"),
            &String::from_str(&env, "Mexico City"),
            &String::from_str(&env, "ETF_ACCOUNT_123"),
        ).unwrap();

        // Submit medical documentation
        let doc_hash = client.submit_medical_documentation(
            &campaign_id,
            &MedicalDocType::MedicalDiagnosis,
            &String::from_str(&env, "https://example.com/medical-report.pdf"),
            &String::from_str(&env, "Diagnóstico médico oficial"),
        ).unwrap();

        // Verify documentation
        let result = client.verify_medical_documentation(&doc_hash, &verifier, &true);
        assert!(result.is_ok());

        // Check campaign verification status
        let campaign = client.get_campaign(&campaign_id);
        assert!(campaign.is_some());
        assert!(campaign.unwrap().medical_docs_verified);
    }
}