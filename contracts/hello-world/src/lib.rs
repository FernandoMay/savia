#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, xdr::ToXdr, Address, Bytes, BytesN, Env, String, Vec, Map, Symbol
};

// Enhanced Savia Smart Contracts for Stellar - Production Ready Version
// Features: KYC-first flow, SPEI integration, wallet connection, 1% fee, staking rewards

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
    pub spei_account: String,
    pub peso_exchange_rate: u64,
    pub total_donations: u32,
    pub platform_fee_collected: u64,
    pub staking_rewards_distributed: u64,
    pub emergency_paused: bool,
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
    pub spei_tx_id: String,
    pub platform_fee: u64,
    pub staking_reward: u64,
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
    pub birth_date: String,
    pub nationality: String,
    pub verification_level: KYCLevel,
    pub verified_at: u64,
    pub expires_at: u64,
    pub medical_license: Option<String>,
    pub institution: Option<String>,
    pub rfc: Option<String>,
    pub bank_account: Option<String>,
    pub spei_clabe: Option<String>,
    pub wallet_connected: bool,
    pub wallet_address: Option<String>,
    pub identity_document_hash: Option<BytesN<32>>,
    pub proof_of_address_hash: Option<BytesN<32>>,
}

#[derive(Clone)]
#[contracttype]
pub enum KYCLevel {
    Unverified,
    BasicVerified,    // CURP + Phone + Email
    BankVerified,     // + Bank account + SPEI
    MedicalVerified,  // + Medical license + Institution
    FullyVerified,    // + All documents + background check
}

#[derive(Clone)]
#[contracttype]
pub struct MedicalDocumentation {
    pub campaign_id: BytesN<32>,
    pub document_type: MedicalDocType,
    pub document_hash: BytesN<32>,
    pub document_url: String,
    pub ipfs_hash: Option<String>,
    pub submitted_at: u64,
    pub verified_by: Option<Address>,
    pub verification_status: DocumentStatus,
    pub expiry_date: u64,
    pub notes: String,
    pub cost_estimate: Option<u64>,
    pub urgency_level: UrgencyLevel,
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
    MedicalEmergency,
    SurgeryQuote,
    TherapyPlan,
}

#[derive(Clone)]
#[contracttype]
pub enum UrgencyLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone)]
#[contracttype]
pub enum DocumentStatus {
    Pending,
    UnderReview,
    Verified,
    Rejected,
    Expired,
    RequiresUpdate,
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
    pub staking_rewards_earned: u64,
    pub nft_locked_for_staking: bool,
    pub boost_multiplier: u32,
}

#[derive(Clone)]
#[contracttype]
pub enum TreeGrowthStage {
    Seed,        // 0-499 pesos
    Sprout,      // 500-1499 pesos
    Sapling,     // 1500-4999 pesos
    YoungTree,   // 5000-9999 pesos
    MatureTree,  // 10000-24999 pesos
    MightyTree,  // 25000-49999 pesos
    LegendaryTree, // 50000+ pesos
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
    pub community_endorsements: u32,
    pub last_updated: u64,
    pub reputation_tier: ReputationTier,
}

#[derive(Clone)]
#[contracttype]
pub enum ReputationTier {
    Newcomer,
    Trusted,
    Verified,
    Champion,
    Legend,
}

#[derive(Clone)]
#[contracttype]
pub struct SPEITransaction {
    pub id: BytesN<32>,
    pub campaign_id: BytesN<32>,
    pub donor: Address,
    pub peso_amount: u64,
    pub xlm_amount: u64,
    pub exchange_rate: u64,
    pub spei_reference: String,
    pub bank_account: String,
    pub clabe: String,
    pub status: TransactionStatus,
    pub timestamp: u64,
    pub confirmation_code: Option<String>,
    pub bank_confirmation: Option<String>,
    pub processing_fee: u64,
}

#[derive(Clone)]
#[contracttype]
pub enum TransactionStatus {
    Pending,
    Processing,
    Confirmed,
    Failed,
    Refunded,
    Cancelled,
}

#[derive(Clone)]
#[contracttype]
pub struct StakingPool {
    pub id: BytesN<32>,
    pub total_staked: u64,
    pub total_rewards: u64,
    pub participants: u32,
    pub apy: u32,
    pub lock_period: u64,
    pub min_stake: u64,
    pub max_stake: u64,
    pub created_at: u64,
    pub active: bool,
}

#[derive(Clone)]
#[contracttype]
pub struct StakingPosition {
    pub user: Address,
    pub pool_id: BytesN<32>,
    pub staked_amount: u64,
    pub staked_at: u64,
    pub unlock_time: u64,
    pub rewards_earned: u64,
    pub nft_boost: bool,
    pub multiplier: u32,
}

#[derive(Clone)]
#[contracttype]
pub struct WalletConnection {
    pub user: Address,
    pub wallet_type: WalletType,
    pub public_key: String,
    pub connected_at: u64,
    pub last_activity: u64,
    pub verified: bool,
    pub permissions: Vec<String>,
}

#[derive(Clone)]
#[contracttype]
pub enum WalletType {
    Freighter,
    Albedo,
    Rabet,
    Lobstr,
    WalletConnect,
    Hardware,
}

#[derive(Clone)]
#[contracttype]
pub struct PlatformStats {
    pub total_campaigns: u64,
    pub total_donations: u64,
    pub total_raised_xlm: u64,
    pub total_raised_pesos: u64,
    pub total_users: u64,
    pub kyc_verified_users: u64,
    pub active_campaigns: u64,
    pub successful_campaigns: u64,
    pub platform_fees_collected: u64,
    pub staking_rewards_distributed: u64,
    pub last_updated: u64,
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
    SPEITransaction(BytesN<32>),
    StakingPool(BytesN<32>),
    StakingPosition(Address, BytesN<32>),
    WalletConnection(Address),
    PlatformStats,
    PlatformFee,
    StakingRewardRate,
    CampaignCounter,
    DonationCounter,
    NFTCounter,
    MedicalDocCounter,
    SPEIConfig,
    PesoExchangeRate,
    KYCVerifiers,
    MedicalVerifiers,
    AdminAddresses,
    EmergencyPause,
    MaxCampaignDuration,
    MinDonationAmount,
    KYCRequiredForDonation,
    StakingPoolCounter,
    TotalStakingRewards,
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
    SPEIError = 16,
    InvalidMedicalDoc = 17,
    NotAuthorized = 18,
    DocumentExpired = 19,
    RefundPeriodExpired = 20,
    WalletNotConnected = 21,
    InvalidWalletType = 22,
    StakingPoolNotFound = 23,
    InsufficientStakingAmount = 24,
    StakingLockPeriodActive = 25,
    EmergencyPauseActive = 26,
    InvalidRFC = 27,
    InvalidCLABE = 28,
    BankAccountNotVerified = 29,
    KYCExpired = 30,
    MaxStakingExceeded = 31,
    InvalidRewardRate = 32,
}

// ========== MAIN CONTRACT ==========

#[contract]
pub struct SaviaContract;

#[contractimpl]
impl SaviaContract {
    
    /// Initialize the contract with enhanced features
    pub fn initialize(
        env: Env,
        admin: Address,
        platform_fee: u64,
        staking_reward_rate: u64,
        spei_config: String,
        initial_peso_rate: u64,
        kyc_required_for_donation: bool,
    ) -> Result<(), soroban_sdk::Error> {
        admin.require_auth();
        
        if platform_fee > 300 { // Max 3% fee
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::InvalidFee as u32));
        }
        
        // Initialize platform settings
        env.storage().instance().set(&DataKey::PlatformFee, &platform_fee);
        env.storage().instance().set(&DataKey::StakingRewardRate, &staking_reward_rate);
        env.storage().instance().set(&DataKey::SPEIConfig, &spei_config);
        env.storage().instance().set(&DataKey::PesoExchangeRate, &initial_peso_rate);
        env.storage().instance().set(&DataKey::KYCRequiredForDonation, &kyc_required_for_donation);
        
        // Initialize counters
        env.storage().instance().set(&DataKey::CampaignCounter, &0u64);
        env.storage().instance().set(&DataKey::DonationCounter, &0u64);
        env.storage().instance().set(&DataKey::NFTCounter, &0u64);
        env.storage().instance().set(&DataKey::MedicalDocCounter, &0u64);
        env.storage().instance().set(&DataKey::StakingPoolCounter, &0u64);
        
        // Initialize settings
        env.storage().instance().set(&DataKey::MaxCampaignDuration, &365u64); // 1 year max
        env.storage().instance().set(&DataKey::MinDonationAmount, &1000000u64); // 1 XLM min
        env.storage().instance().set(&DataKey::EmergencyPause, &false);
        env.storage().instance().set(&DataKey::TotalStakingRewards, &0u64);
        
        // Initialize empty lists
        let empty_vec: Vec<Address> = Vec::new(&env);
        env.storage().instance().set(&DataKey::KYCVerifiers, &empty_vec);
        env.storage().instance().set(&DataKey::MedicalVerifiers, &empty_vec);
        
        let mut admin_vec = Vec::new(&env);
        admin_vec.push_back(admin);
        env.storage().instance().set(&DataKey::AdminAddresses, &admin_vec);
        
        // Initialize platform stats
        let stats = PlatformStats {
            total_campaigns: 0,
            total_donations: 0,
            total_raised_xlm: 0,
            total_raised_pesos: 0,
            total_users: 0,
            kyc_verified_users: 0,
            active_campaigns: 0,
            successful_campaigns: 0,
            platform_fees_collected: 0,
            staking_rewards_distributed: 0,
            last_updated: env.ledger().timestamp(),
        };
        env.storage().instance().set(&DataKey::PlatformStats, &stats);
        
        Ok(())
    }
    
    /// Enhanced KYC registration with optional wallet connection
    pub fn register_kyc(
        env: Env,
        entity: Address,
        curp: String,
        full_name: String,
        phone_number: String,
        email: String,
        address: String,
        birth_date: String,
        nationality: String,
        medical_license: Option<String>,
        institution: Option<String>,
        rfc: Option<String>,
        bank_account: Option<String>,
        spei_clabe: Option<String>,
        wallet_connection: Option<WalletConnection>,
    ) -> Result<(), soroban_sdk::Error> {
        entity.require_auth();
        
        // Validate CURP format (18 characters)
        if curp.len() != 18 {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::InvalidCURP as u32));
        }
        
        // Validate Mexican phone number format (10 digits)
        if phone_number.len() != 10 {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::InvalidPhoneNumber as u32));
        }
        
        // Validate RFC if provided
        if let Some(ref rfc_value) = rfc {
            if rfc_value.len() < 12 || rfc_value.len() > 13 {
                return Err(soroban_sdk::Error::from_contract_error(SaviaError::InvalidRFC as u32));
            }
        }
        
        // Validate CLABE if provided
        if let Some(ref clabe_value) = spei_clabe {
            if clabe_value.len() != 18 {
                return Err(soroban_sdk::Error::from_contract_error(SaviaError::InvalidCLABE as u32));
            }
        }
        
        let current_time = env.ledger().timestamp();
        let expiry_time = current_time + (365 * 24 * 60 * 60); // 1 year validity
        
        // Determine verification level
        let verification_level = match (&medical_license, &institution, &bank_account, &spei_clabe) {
            (Some(_), Some(_), _, _) => KYCLevel::MedicalVerified,
            (_, _, Some(_), Some(_)) => KYCLevel::BankVerified,
            _ => KYCLevel::BasicVerified,
        };
        
        let kyc_record = KYCRecord {
            entity: entity.clone(),
            curp,
            full_name,
            phone_number,
            email,
            address,
            birth_date,
            nationality,
            verification_level,
            verified_at: current_time,
            expires_at: expiry_time,
            medical_license,
            institution,
            rfc,
            bank_account,
            spei_clabe,
            wallet_connected: wallet_connection.is_some(),
            wallet_address: wallet_connection.as_ref().map(|w| w.public_key.clone()),
            identity_document_hash: None,
            proof_of_address_hash: None,
        };
        
        env.storage().persistent().set(&DataKey::KYCRecord(entity.clone()), &kyc_record);
        
        // Store wallet connection if provided
        if let Some(wallet) = wallet_connection {
            env.storage().persistent().set(&DataKey::WalletConnection(entity.clone()), &wallet);
        }
        
        // Initialize trust score
        let _ = Self::initialize_trust_score(env.clone(), entity.clone());
        
        // Update platform stats
        Self::update_platform_stats(env, |stats| {
            stats.total_users += 1;
            stats.kyc_verified_users += 1;
        })?;
        
        Ok(())
    }
    
    /// Connect wallet to existing KYC account
    pub fn connect_wallet(
        env: Env,
        user: Address,
        wallet_type: WalletType,
        public_key: String,
        permissions: Vec<String>,
    ) -> Result<(), soroban_sdk::Error> {
        user.require_auth();
        
        // Check if user has KYC
        let mut kyc_record: KYCRecord = env.storage().persistent().get(&DataKey::KYCRecord(user.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::KYCNotVerified as u32))?;
        
        let current_time = env.ledger().timestamp();
        let wallet_connection = WalletConnection {
            user: user.clone(),
            wallet_type,
            public_key: public_key.clone(),
            connected_at: current_time,
            last_activity: current_time,
            verified: true,
            permissions,
        };
        
        // Update KYC record
        kyc_record.wallet_connected = true;
        kyc_record.wallet_address = Some(public_key);
        
        env.storage().persistent().set(&DataKey::KYCRecord(user.clone()), &kyc_record);
        env.storage().persistent().set(&DataKey::WalletConnection(user), &wallet_connection);
        
        Ok(())
    }
    
    /// Create campaign with enhanced validation
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
        spei_account: String,
        urgency_level: UrgencyLevel,
    ) -> Result<BytesN<32>, soroban_sdk::Error> {
        beneficiary.require_auth();
        
        // Check emergency pause
        let emergency_pause: bool = env.storage().instance().get(&DataKey::EmergencyPause).unwrap_or(false);
        if emergency_pause {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::EmergencyPauseActive as u32));
        }
        
        // Validate KYC verification
        let kyc_record: KYCRecord = env.storage().persistent().get(&DataKey::KYCRecord(beneficiary.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::KYCNotVerified as u32))?;
        
        if kyc_record.expires_at < env.ledger().timestamp() {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::KYCExpired as u32));
        }
        
        // Validate inputs
        if goal_amount == 0 {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::InvalidGoal as u32));
        }
        
        let max_duration: u64 = env.storage().instance().get(&DataKey::MaxCampaignDuration).unwrap_or(365);
        if duration_days == 0 || duration_days > max_duration {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::InvalidDuration as u32));
        }
        
        // Get current peso exchange rate
        let peso_rate: u64 = env.storage().instance().get(&DataKey::PesoExchangeRate).unwrap_or(180000);
        
        // Generate campaign ID
        let counter: u64 = env.storage().instance().get(&DataKey::CampaignCounter).unwrap_or(0);
        let new_counter = counter + 1;
        env.storage().instance().set(&DataKey::CampaignCounter, &new_counter);
        
        let current_time = env.ledger().timestamp();
        let mut hash_input = Bytes::new(&env);
        
        hash_input.append(&beneficiary.clone().to_xdr(&env));
        hash_input.append(&title.to_val().to_xdr(&env));
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
            beneficiary: beneficiary.clone(),
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
            spei_account,
            peso_exchange_rate: peso_rate,
            total_donations: 0,
            platform_fee_collected: 0,
            staking_rewards_distributed: 0,
            emergency_paused: false,
        };
        
        env.storage().persistent().set(&DataKey::Campaign(campaign_id.clone()), &campaign);
        
        // Update trust score for campaign creation
        Self::update_beneficiary_trust_score(env.clone(), beneficiary)?;
        
        // Update platform stats
        Self::update_platform_stats(env, |stats| {
            stats.total_campaigns += 1;
            stats.active_campaigns += 1;
        })?;
        
        Ok(campaign_id)
    }
    
    /// Enhanced donation with SPEI integration
    pub fn donate(
        env: Env,
        campaign_id: BytesN<32>,
        donor: Address,
        xlm_amount: u64,
        anonymous: bool,
        mint_nft: bool,
        spei_reference: Option<String>,
    ) -> Result<BytesN<32>, soroban_sdk::Error> {
        donor.require_auth();
        
        // Check emergency pause
        let emergency_pause: bool = env.storage().instance().get(&DataKey::EmergencyPause).unwrap_or(false);
        if emergency_pause {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::EmergencyPauseActive as u32));
        }
        
        // Validate campaign exists and is active
        let mut campaign: Campaign = env.storage().persistent().get(&DataKey::Campaign(campaign_id.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::CampaignNotFound as u32))?;
        
        let current_time = env.ledger().timestamp();
        
        // Check if campaign has ended
        if current_time > campaign.end_time {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::CampaignEnded as u32));
        }
        
        // Check if funds are locked
        if campaign.funds_locked || campaign.emergency_paused {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::FundsLocked as u32));
        }
        
        // Check minimum donation amount
        let min_amount: u64 = env.storage().instance().get(&DataKey::MinDonationAmount).unwrap_or(1000000);
        if xlm_amount < min_amount {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::InvalidAmount as u32));
        }
        
        // Check KYC requirement
        let kyc_required: bool = env.storage().instance().get(&DataKey::KYCRequiredForDonation).unwrap_or(true);
        if kyc_required {
            let kyc_record: KYCRecord = env.storage().persistent().get(&DataKey::KYCRecord(donor.clone()))
                .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::KYCNotVerified as u32))?;
            
            if kyc_record.expires_at < current_time {
                return Err(soroban_sdk::Error::from_contract_error(SaviaError::KYCExpired as u32));
            }
        }
        
        // Convert XLM to pesos
        let peso_amount = (xlm_amount * campaign.peso_exchange_rate) / 10000;
        
        // Calculate fees (1% platform fee)
        let platform_fee_rate: u64 = env.storage().instance().get(&DataKey::PlatformFee).unwrap_or(100);
        let platform_fee = (xlm_amount * platform_fee_rate) / 10000;
        let net_xlm_amount = xlm_amount - platform_fee;
        let net_peso_amount = (net_xlm_amount * campaign.peso_exchange_rate) / 10000;
        
        // Calculate staking reward (0.1% of donation)
        let staking_reward_rate: u64 = env.storage().instance().get(&DataKey::StakingRewardRate).unwrap_or(10);
        let staking_reward = (xlm_amount * staking_reward_rate) / 10000;
        
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
        
        // Create SPEI transaction if reference provided
        let spei_tx_id = if let Some(ref reference) = spei_reference {
            Self::create_spei_transaction(
                env.clone(),
                campaign_id.clone(),
                donor.clone(),
                net_peso_amount,
                net_xlm_amount,
                campaign.peso_exchange_rate,
                reference.clone(),
                campaign.spei_account.clone(),
            )?
        } else {
            String::from_str(&env, "NO_SPEI")
        };
        
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
            spei_tx_id,
            platform_fee,
            staking_reward,        };
        
        env.storage().persistent().set(&DataKey::Donation(donation_id.clone()), &donation);
        
        // Update campaign
        campaign.current_amount += net_xlm_amount;
        campaign.total_donations += 1;
        campaign.platform_fee_collected += platform_fee;
        campaign.staking_rewards_distributed += staking_reward;
        env.storage().persistent().set(&DataKey::Campaign(campaign_id.clone()), &campaign);
        
        // Update donor's trust score
        Self::update_donor_trust_score(env.clone(), donor.clone(), net_xlm_amount)?;
        
        // Mint NFT if requested
        if mint_nft {
            Self::mint_dynamic_nft(env.clone(), donor.clone(), campaign_id.clone(), net_xlm_amount)?;
        }
        
        // Update total staking rewards
        let mut total_rewards: u64 = env.storage().instance().get(&DataKey::TotalStakingRewards).unwrap_or(0);
        total_rewards += staking_reward;
        env.storage().instance().set(&DataKey::TotalStakingRewards, &total_rewards);
        
        // Update platform stats
        Self::update_platform_stats(env, |stats| {
            stats.total_donations += 1;
            stats.total_raised_xlm += net_xlm_amount;
            stats.total_raised_pesos += net_peso_amount;
            stats.platform_fees_collected += platform_fee;
            stats.staking_rewards_distributed += staking_reward;
        })?;
        
        Ok(donation_id)
    }
    
    /// Submit medical documentation for a campaign
    pub fn submit_medical_doc(
        env: Env,
        campaign_id: BytesN<32>,
        beneficiary: Address,
        document_type: MedicalDocType,
        document_hash: BytesN<32>,
        document_url: String,
        ipfs_hash: Option<String>,
        expiry_date: u64,
        notes: String,
        cost_estimate: Option<u64>,
        urgency_level: UrgencyLevel,
    ) -> Result<BytesN<32>, soroban_sdk::Error> {
        beneficiary.require_auth();
        
        // Check emergency pause
        let emergency_pause: bool = env.storage().instance().get(&DataKey::EmergencyPause).unwrap_or(false);
        if emergency_pause {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::EmergencyPauseActive as u32));
        }
        
        // Validate campaign exists and beneficiary matches
        let campaign: Campaign = env.storage().persistent().get(&DataKey::Campaign(campaign_id.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::CampaignNotFound as u32))?;
        
        if campaign.beneficiary != beneficiary {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32));
        }
        
        let current_time = env.ledger().timestamp();
        
        // Generate medical document ID
        let counter: u64 = env.storage().instance().get(&DataKey::MedicalDocCounter).unwrap_or(0);
        let new_counter = counter + 1;
        env.storage().instance().set(&DataKey::MedicalDocCounter, &new_counter);
        
        let mut hash_input = Bytes::new(&env);
        hash_input.append(&Bytes::from_slice(&env, campaign_id.to_array().as_slice()));
        hash_input.append(&Bytes::from_slice(&env, document_hash.to_array().as_slice()));
        hash_input.append(&Bytes::from_slice(&env, &current_time.to_be_bytes()));
        hash_input.append(&Bytes::from_slice(&env, &new_counter.to_be_bytes()));
        
        let doc_id: BytesN<32> = env.crypto().sha256(&hash_input).into();
        
        let medical_doc = MedicalDocumentation {
            campaign_id,
            document_type,
            document_hash,
            document_url,
            ipfs_hash,
            submitted_at: current_time,
            verified_by: None,
            verification_status: DocumentStatus::Pending,
            expiry_date,
            notes,
            cost_estimate,
            urgency_level,
        };
        
        env.storage().persistent().set(&DataKey::MedicalDoc(doc_id.clone()), &medical_doc);
        
        // Update beneficiary's trust score for submitting docs
        Self::update_beneficiary_trust_score(env.clone(), beneficiary)?;
        
        Ok(doc_id)
    }
    
    /// Verify medical documentation (only by authorized verifiers)
    pub fn verify_medical_doc(
        env: Env,
        verifier: Address,
        doc_id: BytesN<32>,
        status: DocumentStatus,
        notes: String,
    ) -> Result<(), soroban_sdk::Error> {
        verifier.require_auth();
        
        // Check emergency pause
        let emergency_pause: bool = env.storage().instance().get(&DataKey::EmergencyPause).unwrap_or(false);
        if emergency_pause {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::EmergencyPauseActive as u32));
        }
        
        // Check if verifier is authorized
        let medical_verifiers: Vec<Address> = env.storage().instance().get(&DataKey::MedicalVerifiers).unwrap_or(Vec::new(&env));
        if !medical_verifiers.contains(&verifier) {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32));
        }
        
        let mut medical_doc: MedicalDocumentation = env.storage().persistent().get(&DataKey::MedicalDoc(doc_id.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::InvalidMedicalDoc as u32))?;
        
        // Only allow status change to Verified or Rejected
        if status != DocumentStatus::Verified && status != DocumentStatus::Rejected {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::InvalidMedicalDoc as u32));
        }
        
        medical_doc.verification_status = status.clone();
        medical_doc.verified_by = Some(verifier.clone());
        medical_doc.notes = notes;
        
        env.storage().persistent().set(&DataKey::MedicalDoc(doc_id.clone()), &medical_doc);
        
        // Update campaign's medical_docs_verified status if all required docs are verified
        if status == DocumentStatus::Verified {
            let mut campaign: Campaign = env.storage().persistent().get(&DataKey::Campaign(medical_doc.campaign_id.clone()))
                .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::CampaignNotFound as u32))?;
            
            // For simplicity, let's assume one verified medical doc is enough for now.
            // In a real scenario, you'd check for a set of required documents.
            campaign.medical_docs_verified = true;
            env.storage().persistent().set(&DataKey::Campaign(medical_doc.campaign_id), &campaign);
            
            // Update verifier's trust score
            Self::update_verifier_trust_score(env.clone(), verifier)?;
        }
        
        Ok(())
    }
    
    /// Update campaign status (e.g., mark as verified, lock funds)
    pub fn update_campaign_status(
        env: Env,
        admin: Address,
        campaign_id: BytesN<32>,
        verified: Option<bool>,
        funds_locked: Option<bool>,
        emergency_paused: Option<bool>,
    ) -> Result<(), soroban_sdk::Error> {
        admin.require_auth();
        
        // Check if admin is authorized
        let admins: Vec<Address> = env.storage().instance().get(&DataKey::AdminAddresses).unwrap_or(Vec::new(&env));
        if !admins.contains(&admin) {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32));
        }
        
        let mut campaign: Campaign = env.storage().persistent().get(&DataKey::Campaign(campaign_id.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::CampaignNotFound as u32))?;

        if let Some(v) = verified {
            campaign.verified = v;
        }
        if let Some(fl) = funds_locked {
            campaign.funds_locked = fl;
        }
        if let Some(ep) = emergency_paused {
            campaign.emergency_paused = ep;
        }

        env.storage().persistent().set(&DataKey::Campaign(campaign_id), &campaign);

        Ok(())
    }

    /// Withdraw funds from a campaign (only by beneficiary after verification)
    pub fn withdraw_funds(
        env: Env,
        beneficiary: Address,
        campaign_id: BytesN<32>,
        amount: u64,
    ) -> Result<(), soroban_sdk::Error> {
        beneficiary.require_auth();

        // Check emergency pause
        let emergency_pause: bool = env.storage().instance().get(&DataKey::EmergencyPause).unwrap_or(false);
        if emergency_pause {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::EmergencyPauseActive as u32));
        }

        let mut campaign: Campaign = env.storage().persistent().get(&DataKey::Campaign(campaign_id.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::CampaignNotFound as u32))?;

        // Ensure beneficiary is the campaign owner
        if campaign.beneficiary != beneficiary {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32));
        }

        // Ensure campaign is verified and medical docs are verified
        if !campaign.verified || !campaign.medical_docs_verified {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotApproved as u32));
        }

        // Ensure funds are not locked
        if campaign.funds_locked {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::FundsLocked as u32));
        }

        // Ensure        // Ensure there are sufficient funds
        if campaign.current_amount < amount {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::InsufficientFunds as u32));
        }

        // Transfer funds (simulate by reducing current_amount)
        campaign.current_amount -= amount;
        env.storage().persistent().set(&DataKey::Campaign(campaign_id), &campaign);

        // TODO: Implement actual XLM transfer to beneficiary's address
        // This would involve using `env.transfer()` or similar functionality
        // For now, we just update the internal balance.

        Ok(())
    }

    /// Refund a donation (only by admin or if specific conditions met)
    pub fn refund_donation(
        env: Env,
        caller: Address, // Can be admin or donor requesting refund
        donation_id: BytesN<32>,
    ) -> Result<(), soroban_sdk::Error> {
        caller.require_auth();

        // Check emergency pause
        let emergency_pause: bool = env.storage().instance().get(&DataKey::EmergencyPause).unwrap_or(false);
        if emergency_pause {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::EmergencyPauseActive as u32));
        }

        let mut donation: Donation = env.storage().persistent().get(&DataKey::Donation(donation_id.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::CampaignNotFound as u32))?; // Reusing error code for now

        if donation.refunded {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::RefundPeriodExpired as u32)); // Already refunded
        }

        let current_time = env.ledger().timestamp();
        // Allow refund within 7 days of donation, or if called by an admin
        let refund_period_end = donation.timestamp + (7 * 24 * 60 * 60); // 7 days

        let admins: Vec<Address> = env.storage().instance().get(&DataKey::AdminAddresses).unwrap_or(Vec::new(&env));
        let    is_admin = admins.contains(&caller);

        if !is_admin && current_time > refund_period_end {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::RefundPeriodExpired as u32));
        }

        // Only donor or admin can request refund
        if !is_admin && donation.donor != caller {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32));
        }

        let mut campaign: Campaign = env.storage().persistent().get(&DataKey::Campaign(donation.campaign_id.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::CampaignNotFound as u32))?;

        // Deduct refunded amount from campaign's current_amount
        campaign.current_amount -= donation.amount;
        env.storage().persistent().set(&DataKey::Campaign(donation.campaign_id), &campaign);

        // Mark donation as refunded
        donation.refunded = true;
        env.storage().persistent().set(&DataKey::Donation(donation_id), &donation);

        // TODO: Implement actual XLM transfer back to donor
        // This would involve using `env.transfer()` or similar functionality

        // Update platform stats (deduct from total raised)
        Self::update_platform_stats(env, |stats| {
            stats.total_raised_xlm -= donation.amount;
            stats.total_raised_pesos -= donation.peso_amount; // Assuming peso_amount was calculated based on XLM
        })?;

        Ok(())
    }

    /// Set or update platform fee (only by admin)
    pub fn set_platform_fee(env: Env, admin: Address, new_fee: u64) -> Result<(), soroban_sdk::Error> {
        admin.require_auth();

        let admins: Vec<Address> = env.storage().instance().get(&DataKey::AdminAddresses).unwrap_or(Vec::new(&env));
        if !admins.contains(&admin) {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32));
        }

        if new_fee > 300 { // Max 3% fee
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::InvalidFee as u32));
        }

        env.storage().instance().set(&DataKey::PlatformFee, &new_fee);
        Ok(())
    }

    /// Set or update staking reward rate (only by admin)
    pub fn set_staking_reward_rate(env: Env, admin: Address, new_rate: u64) -> Result<(), soroban_sdk::Error> {
        admin.require_auth();

        let admins: Vec<Address> = env.storage().instance().get(&DataKey::AdminAddresses).unwrap_or(Vec::new(&env));
        if !admins.contains(&admin) {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32));
        }

        if new_rate > 100 { // Max 1% reward rate
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::InvalidRewardRate as u32));
        }

        env.storage().instance().set(&DataKey::StakingRewardRate, &new_rate);
        Ok(())
    }

    /// Set or update peso exchange rate (only by admin)
    pub fn set_peso_exchange_rate(env: Env, admin: Address, new_rate: u64) -> Result<(), soroban_sdk::Error> {
        admin.require_auth();

        let admins: Vec<Address> = env.storage().instance().get(&DataKey::AdminAddresses).unwrap_or(Vec::new(&env));
        if !admins.contains(&admin) {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32));
        }

        env.storage().instance().set(&DataKey::PesoExchangeRate, &new_rate);
        Ok(())
    }

    /// Toggle emergency pause (only by admin)
    pub fn toggle_emergency_pause(env: Env, admin: Address) -> Result<(), soroban_sdk::Error> {
        admin.require_auth();

        let admins: Vec<Address> = env.storage().instance().get(&DataKey::AdminAddresses).unwrap_or(Vec::new(&env));
        if !admins.contains(&admin) {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32));
        }

        let current_pause_status: bool = env.storage().instance().get(&DataKey::EmergencyPause).unwrap_or(false);
        env.storage().instance().set(&DataKey::EmergencyPause, &!current_pause_status);
        Ok(())
    }

    /// Add a new admin address (only by existing admin)
    pub fn add_admin(env: Env, admin: Address, new_admin: Address) -> Result<(), soroban_sdk::Error> {
        admin.require_auth();

        let mut admins: Vec<Address> = env.storage().instance().get(&DataKey::AdminAddresses).unwrap_or(Vec::new(&env));
        if !admins.contains(&admin) {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32));
        }

        if !admins.contains(&new_admin) {
            admins.push_back(new_admin);
            env.storage().instance().set(&DataKey::AdminAddresses, &admins);
        }
        Ok(())
    }

    /// Remove an admin address (only by existing admin)
    pub fn remove_admin(env: Env, admin: Address, old_admin: Address) -> Result<(), soroban_sdk::Error> {
        admin.require_auth();

        let mut admins: Vec<Address> = env.storage().instance().get(&DataKey::AdminAddresses).unwrap_or(Vec::new(&env));
        if !admins.contains(&admin) {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32));
        }

        let initial_len = admins.len();
        admins = admins.into_iter().filter(|a| a != &old_admin).collect();
        if admins.len() == initial_len {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32)); // Admin not found
        }
        env.storage().instance().set(&DataKey::AdminAddresses, &admins);
        Ok(())
    }

    /// Add a new KYC verifier (only by admin)
    pub fn add_kyc_verifier(env: Env, admin: Address, new_verifier: Address) -> Result<(), soroban_sdk::Error> {
        admin.require_auth();

        let admins: Vec<Address> = env.storage().instance().get(&DataKey::AdminAddresses).unwrap_or(Vec::new(&env));
        if !admins.contains(&admin) {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32));
        }

        let mut verifiers: Vec<Address> = env.storage().instance().get(&DataKey::KYCVerifiers).unwrap_or(Vec::new(&env));
        if !verifiers.contains(&new_verifier) {
            verifiers.push_back(new_verifier);
            env.storage().instance().set(&DataKey::KYCVerifiers, &verifiers);
        }
        Ok(())
    }

    /// Remove a KYC verifier (only by admin)
    pub fn remove_kyc_verifier(env: Env, admin: Address, old_verifier: Address) -> Result<(), soroban_sdk::Error> {
        admin.require_auth();

        let admins: Vec<Address> = env.storage().instance().get(&DataKey::AdminAddresses).unwrap_or(Vec::new(&env));
        if !admins.contains(&admin) {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32));
        }

        let mut verifiers: Vec<Address> = env.storage().instance().get(&DataKey::KYCVerifiers).unwrap_or(Vec::new(&env));
        let initial_len = verifiers.len();
        verifiers = verifiers.into_iter().filter(|a| a != &old_verifier).collect();
        if verifiers.len() == initial_len {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32)); // Verifier not found
        }
        env.storage().instance().set(&DataKey::KYCVerifiers, &verifiers);
        Ok(())
    }

    /// Add a new Medical verifier (only by admin)
    pub fn add_medical_verifier(env: Env, admin: Address, new_verifier: Address) -> Result<(), soroban_sdk::Error> {
        admin.require_auth();

        let admins: Vec<Address> = env.storage().instance().get(&DataKey::AdminAddresses).unwrap_or(Vec::new(&env));
        if !admins.contains(&admin) {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32));
        }

        let mut verifiers: Vec<Address> = env.storage().instance().get(&DataKey::MedicalVerifiers).unwrap_or(Vec::new(&env));
        if !verifiers.contains(&new_verifier) {
            verifiers.push_back(new_verifier);
            env.storage().instance().set(&DataKey::MedicalVerifiers, &verifiers);
        }
        Ok(())
    }

    /// Remove a Medical verifier (only by admin)
    pub fn remove_medical_verifier(env: Env, admin: Address, old_verifier: Address) -> Result<(), soroban_sdk::Error> {
        admin.require_auth();

        let admins: Vec<Address> = env.storage().instance().get(&DataKey::AdminAddresses).unwrap_or(Vec::new(&env));
        if !admins.contains(&admin) {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32));
        }

        let mut verifiers: Vec<Address> = env.storage().instance().get(&DataKey::MedicalVerifiers).unwrap_or(Vec::new(&env));
        let initial_len = verifiers.len();
        verifiers = verifiers.into_iter().filter(|a| a != &old_verifier).collect();
        if verifiers.len() == initial_len {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32)); // Verifier not found
        }
        env.storage().instance().set(&DataKey::MedicalVerifiers, &verifiers);
        Ok(())
    }

    /// Create a new staking pool
    pub fn create_staking_pool(
        env: Env,
        admin: Address,
        total_staked: u64,
        total_rewards: u64,
        apy: u32,
        lock_period: u64,
        min_stake: u64,
        max_stake: u64,
    ) -> Result<BytesN<32>, soroban_sdk::Error> {
        admin.require_auth();

        let admins: Vec<Address> = env.storage().instance().get(&DataKey::AdminAddresses).unwrap_or(Vec::new(&env));
        if !admins.contains(&admin) {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::NotAuthorized as u32));
        }

        let current_time = env.ledger().timestamp();
        let counter: u64 = env.storage().instance().get(&DataKey::StakingPoolCounter).unwrap_or(0);
        let new_counter = counter + 1;
        env.storage().instance().set(&DataKey::StakingPoolCounter, &new_counter);

        let mut hash_input = Bytes::new(&env);
        hash_input.append(&Bytes::from_slice(&env, &current_time.to_be_bytes()));
        hash_input.append(&Bytes::from_slice(&env, &new_counter.to_be_bytes()));
        hash_input.append(&Bytes::from_slice(&env, &apy.to_be_bytes()));

        let pool_id: BytesN<32> = env.crypto().sha256(&hash_input).into();

        let staking_pool = StakingPool {
            id: pool_id.clone(),
            total_staked,
            total_rewards,
            participants: 0,
            apy,
            lock_period,
            min_stake,
            max_stake,
            created_at: current_time,
            active: true,
        };

        env.storage().persistent().set(&DataKey::StakingPool(pool_id.clone()), &staking_pool);

        Ok(pool_id)
    }

    /// Stake funds into a staking pool
    pub fn stake(
        env: Env,
        user: Address,
        pool_id: BytesN<32>,
        amount: u64,
    ) -> Result<(), soroban_sdk::Error> {
        user.require_auth();

        // Check emergency pause
        let emergency_pause: bool = env.storage().instance().get(&DataKey::EmergencyPause).unwrap_or(false);
        if emergency_pause {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::EmergencyPauseActive as u32));
        }

        let mut pool: StakingPool = env.storage().persistent().get(&DataKey::StakingPool(pool_id.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::StakingPoolNotFound as u32))?;

        if !pool.active {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::StakingPoolNotFound as u32)); // Pool not active
        }

        if amount < pool.min_stake {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::InsufficientStakingAmount as u32));
        }

        if amount > pool.max_stake {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::MaxStakingExceeded as u32));
        }

        let current_time = env.ledger().timestamp();
        let unlock_time = current_time + pool.lock_period;

        let mut position = env.storage().persistent().get(&DataKey::StakingPosition(user.clone(), pool_id.clone()))
            .unwrap_or_else(|| StakingPosition {
                user: user.clone(),                pool_id: pool_id.clone(),
                staked_amount: 0,
                staked_at: current_time,
                unlock_time: 0, // Will be set below
                rewards_earned: 0,
                nft_boost: false, // Default
                multiplier: 1,    // Default
            });

        // If this is a new position or an existing one that's unlocked, update it
        if position.staked_amount == 0 || current_time >= position.unlock_time {
            position.staked_amount += amount;
            position.staked_at = current_time;
            position.unlock_time = unlock_time;
            position.rewards_earned = 0; // Reset rewards for new stake period
            // TODO: Implement NFT boost logic here if user owns a boosting NFT
        } else {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::StakingLockPeriodActive as u32));
        }

        env.storage().persistent().set(&DataKey::StakingPosition(user.clone(), pool_id.clone()), &position);

        // Update pool stats
        pool.total_staked += amount;
        if position.staked_amount == amount { // Only increment participants for new stakers
            pool.participants += 1;
        }
        env.storage().persistent().set(&DataKey::StakingPool(pool_id), &pool);

        // TODO: Transfer actual XLM from user to contract's staking balance
        // This would involve `env.transfer()` or similar.

        Ok(())
    }

    /// Unstake funds from a staking pool
    pub fn unstake(
        env: Env,
        user: Address,
        pool_id: BytesN<32>,
    ) -> Result<(), soroban_sdk::Error> {
        user.require_auth();

        // Check emergency pause
        let emergency_pause: bool = env.storage().instance().get(&DataKey::EmergencyPause).unwrap_or(false);
        if emergency_pause {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::EmergencyPauseActive as u32));
        }

        let mut position: StakingPosition = env.storage().persistent().get(&DataKey::StakingPosition(user.clone(), pool_id.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::StakingPoolNotFound as u32))?; // Reusing error code

        let current_time = env.ledger().timestamp();

        if current_time < position.unlock_time {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::StakingLockPeriodActive as u32));
        }

        let mut pool: StakingPool = env.storage().persistent().get(&DataKey::StakingPool(pool_id.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::StakingPoolNotFound as u32))?;

        // Calculate rewards earned
        let elapsed_time = current_time - position.staked_at;
        let rewards = (position.staked_amount * pool.apy as u64 * elapsed_time) / (365 * 24 * 60 * 60 * 10000); // Simple APY calculation

        position.rewards_earned += rewards;
        pool.total_rewards += rewards;
        pool.total_staked -= position.staked_amount;
        pool.participants -= 1;

        // TODO: Transfer staked amount + rewards to user
        // This would involve `env.transfer()` or similar.

        // Remove staking position
        env.storage().persistent().remove(&DataKey::StakingPosition(user.clone(), pool_id.clone()));
        env.storage().persistent().set(&DataKey::StakingPool(pool_id), &pool);

        // Update total staking rewards distributed
        let mut total_rewards_distributed: u64 = env.storage().instance().get(&DataKey::TotalStakingRewards).unwrap_or(0);
        total_rewards_distributed += rewards;
        env.storage().instance().set(&DataKey::TotalStakingRewards, &total_rewards_distributed);

        // Update platform stats
        Self::update_platform_stats(env, |stats| {
            stats.staking_rewards_distributed += rewards;
        })?;

        Ok(())
    }

    // ========== HELPER FUNCTIONS ==========

    /// Internal function to update platform statistics
    fn update_platform_stats<F>(env: Env, update_fn: F) -> Result<(), soroban_sdk::Error>
    where
        F: FnOnce(&mut PlatformStats),
    {
        let mut stats: PlatformStats = env.storage().instance().get(&DataKey::PlatformStats).unwrap_or_else(|| PlatformStats {
            total_campaigns: 0,
            total_donations: 0,
            total_raised_xlm: 0,
            total_raised_pesos: 0,
            total_users: 0,
            kyc_verified_users: 0,
            active_campaigns: 0,
            successful_campaigns: 0,
            platform_fees_collected: 0,
            staking_rewards_distributed: 0,
            last_updated: env.ledger().timestamp(),
        });

        update_fn(&mut stats);
        stats.last_updated = env.ledger().timestamp();
        env.storage().instance().set(&DataKey::PlatformStats, &stats);
        Ok(())
    }

    /// Internal function to initialize a user's trust score
    fn initialize_trust_score(env: Env, entity: Address) -> Result<(), soroban_sdk::Error> {
        if env.storage().persistent().has(&DataKey::TrustScore(entity.clone())) {
            return Err(soroban_sdk::Error::from_contract_error(SaviaError::ScoreExists as u32));
        }

        let kyc_record: KYCRecord = env.storage().persistent().get(&DataKey::KYCRecord(entity.clone()))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::KYCNotVerified as u32))?;

        let initial_score = match kyc_record.verification_level {
            KYCLevel::Unverified => 0,
            KYCLevel::BasicVerified => 100,
            KYCLevel::BankVerified => 200,
            KYCLevel::MedicalVerified => 300,
            KYCLevel::FullyVerified => 500,
        };

        let trust_score = TrustScore {
            entity: entity.clone(),
            score: initial_score,
            verification_level: initial_score, // Using initial score as base for verification level
            donation_count: 0,
            total_donated: 0,
            campaigns_created: 0,
            medical_docs_submitted: 0,
            docs_verified_on_time: 0,
            late_submissions: 0,
            fraud_reports: 0,
            community_endorsements: 0,
            last_updated: env.ledger().timestamp(),
            reputation_tier: ReputationTier::Newcomer,
        };

        env.storage().persistent().set(&DataKey::TrustScore(entity), &trust_score);
        Ok(())
    }

    /// Internal function to update a beneficiary's trust score
    fn update_beneficiary_trust_score(env: Env, beneficiary: Address) -> Result<(), soroban_sdk::Error> {
        let mut trust_score: TrustScore = env.storage().persistent().get(&DataKey::TrustScore(beneficiary.clone()))
            .unwrap_or_else(|| {
                // If trust score doesn't exist, initialize it (should ideally be done during KYC)
                Self::initialize_trust_score(env.clone(), beneficiary.clone()).unwrap();
                env.storage().persistent().get(&DataKey::TrustScore(beneficiary.clone())).unwrap()
            });

        trust_score.campaigns_created += 1;
        trust_score.score += 50; // Reward for creating a campaign
        trust_score.last_updated = env.ledger().timestamp();
        trust_score.reputation_tier = Self::determine_reputation_tier(&trust_score);

        env.storage().persistent().set(&DataKey::TrustScore(beneficiary), &trust_score);
        Ok(())
    }

    /// Internal function to update a donor's trust score
    fn update_donor_trust_score(env: Env, donor: Address, amount: u64) -> Result(), soroban_sdk::Error> {
        let mut trust_score: TrustScore = env.storage().persistent().get(&DataKey::TrustScore(donor.clone()))
            .unwrap_or_else(|| {
                // If trust score doesn't exist, initialize it (should ideally be done during KYC)
                Self::initialize_trust_score(env.clone(), donor.clone()).unwrap();
                env.storage().persistent().get(&DataKey::TrustScore(donor.clone())).unwrap()
            });

        trust_score.donation_count += 1;
        trust_score.total_donated += amount;
        trust_score.score += (amount / 1000000) as u32; // 1 point per XLM donated
        trust_score.last_updated = env.ledger().timestamp();
        trust_score.reputation_tier = Self::determine_reputation_tier(&trust_score);

        env.storage().persistent().set(&DataKey::TrustScore(donor), &trust_score);
        Ok(())
    }

    /// Internal function to update a verifier's trust score
    fn update_verifier_trust_score(env: Env, verifier: Address) -> Result<(), soroban_sdk::Error> {
        let mut trust_score: TrustScore = env.storage().persistent().get(&DataKey::TrustScore(verifier.clone()))
            .unwrap_or_else(|| {
                // If trust score doesn't exist, initialize it (should ideally be done during KYC)
                Self::initialize_trust_score(env.clone(), verifier.clone()).unwrap();
                env.storage().persistent().get(&DataKey::TrustScore(verifier.clone())).unwrap()
            });

        trust_score.docs_verified_on_time += 1;
        trust_score.score += 20; // Reward for verifying a document
        trust_score.last_updated = env.ledger().timestamp();
        trust_score.reputation_tier = Self::determine_reputation_tier(&trust_score);

        env.storage().persistent().set(&DataKey::TrustScore(verifier), &trust_score);
        Ok(())
    }

    /// Internal function to determine reputation tier basedon score
    fn determine_reputation_tier(score: &TrustScore) -> ReputationTier {
        if score.score >= 1000 {
            ReputationTier::Legend
        } else if score.score >= 500 {
            ReputationTier::Champion
        } else if score.score >= 200 {
            ReputationTier::Verified
        } else if score.score >= 50 {
            ReputationTier::Trusted
        } else {
            ReputationTier::Newcomer
        }
    }

    /// Internal function to mint a dynamic NFT
    fn mint_dynamic_nft(
        env: Env,
        owner: Address,
        campaign_id: BytesN<32>,
        donated_amount: u64,
    ) -> Result<BytesN<32>, soroban_sdk::Error> {
        let current_time = env.ledger().timestamp();
        let counter: u64 = env.storage().instance().get(&DataKey::NFTCounter).unwrap_or(0);
        let new_counter = counter + 1;
        env.storage().instance().set(&DataKey::NFTCounter, &new_counter);

        let mut hash_input = Bytes::new(&env);
        hash_input.append(&owner.to_xdr(&env));
        hash_input.append(&Bytes::from_slice(&env, campaign_id.to_array().as_slice()));
        hash_input.append(&Bytes::from_slice(&env, &current_time.to_be_bytes()));
        hash_input.append(&Bytes::from_slice(&env, &new_counter.to_be_bytes()));

        let nft_id: BytesN<32> = env.crypto().sha256(&hash_input).into();

        let mut existing_nft: Option<DynamicNFT> = env.storage().persistent().get(&DataKey::DynamicNFT(owner.clone()));

        let (tree_level, total_donated, donation_count, growth_stage) = if let Some(mut nft) = existing_nft {
            nft.total_donated += donated_amount;
            nft.donation_count += 1;
            nft.last_updated = env.ledger().timestamp();
            let new_growth_stage = Self::determine_tree_growth_stage(nft.total_donated);
            if new_growth_stage != nft.growth_stage {
                nft.growth_stage = new_growth_stage;
                // Potentially update metadata_uri here based on new stage
            }
            env.storage().persistent().set(&DataKey::DynamicNFT(nft_id.clone()), &nft);
            (nft.tree_level, nft.total_donated, nft.donation_count, nft.growth_stage)
        } else {
            let initial_growth_stage = Self::determine_tree_growth_stage(donated_amount);
            let new_nft = DynamicNFT {
                id: nft_id.clone(),
                owner: owner.clone(),
                campaign_id,
                tree_level: 1, // Initial level
                total_donated: donated_amount,
                donation_count: 1,
                created_at: current_time,
                last_updated: current_time,
                metadata_uri: String::from_str(&env, "ipfs://initial_metadata"), // Placeholder
                growth_stage: initial_growth_stage,
                special_achievements: Vec::new(&env),
                staking_rewards_earned: 0,
                nft_locked_for_staking: false,
                boost_multiplier: 1,
            };
            env.storage().persistent().set(&DataKey::DynamicNFT(nft_id.clone()), &new_nft);
            (new_nft.tree_level, new_nft.total_donated, new_nft.donation_count, new_nft.growth_stage)
        };

        Ok(nft_id)
    }

    /// Internal function to determine the tree growth stage based on total donated amount
    fn determine_tree_growth_stage(total_donated: u64) -> TreeGrowthStage {
        // Assuming 1 XLM = 1,000,000 stroops, and 1 peso = 180,000 stroops (example rate)
        // Convert XLM to pesos for comparison
        let total_donated_pesos = (total_donated * 18) / 1000000; // Example: 1 XLM = 18 pesos, adjust as needed

        if total_donated_pesos >= 50000 {
            TreeGrowthStage::LegendaryTree
        } else if total_donated_pesos >= 25000 {
            TreeGrowthStage::MightyTree
        } else if total_donated_pesos >= 10000 {
            TreeGrowthStage::MatureTree
        } else if total_donated_pesos >= 5000 {
            TreeGrowthStage::YoungTree
        } else if total_donated_pesos >= 1500 {
            TreeGrowthStage::Sapling
        } else if total_donated_pesos >= 500 {
            TreeGrowthStage::Sprout
        } else {
            TreeGrowthStage::Seed
        }
    }

    /// Internal function to create a SPEI transaction record
    fn create_spei_transaction(
        env: Env,
        campaign_id: BytesN<32>,
        donor: Address,
        peso_amount: u64,
        xlm_amount: u64,
        exchange_rate: u64,
        spei_reference: String,
        bank_account: String,
    ) -> Result<String, soroban_sdk::Error> {
        let current_time = env.ledger().timestamp();
        let mut hash_input = Bytes::new(&env);
        hash_input.append(&Bytes::from_slice(&env, campaign_id.to_array().as_slice()));
        hash_input.append(&donor.to_xdr(&env));
        hash_input.append(&Bytes::from_slice(&env, &peso_amount.to_be_bytes()));
        hash_input.append(&Bytes::from_slice(&env, &current_time.to_be_bytes()));

        let tx_id: BytesN<32> = env.crypto().sha256(&hash_input).into();

        let spei_tx = SPEITransaction {
            id: tx_id.clone(),
            campaign_id,            donor,
            peso_amount,
            xlm_amount,
            exchange_rate,
            spei_reference: spei_reference.clone(),
            bank_account,
            clabe: String::from_str(&env, "N/A"), // CLABE would typically come from the beneficiary's KYC
            status: TransactionStatus::Pending,
            timestamp: current_time,
            confirmation_code: None,
            bank_confirmation: None,
            processing_fee: 0, // To be determined by actual SPEI integration
        };

        env.storage().persistent().set(&DataKey::SPEITransaction(tx_id), &spei_tx);

        Ok(spei_reference)
    }

    // ========== VIEWS ==========

    /// Get campaign details
    pub fn get_campaign(env: Env, campaign_id: BytesN<32>) -> Result<Campaign, soroban_sdk::Error> {
        env.storage().persistent().get(&DataKey::Campaign(campaign_id))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::CampaignNotFound as u32))
    }

    /// Get donation details
    pub fn get_donation(env: Env, donation_id: BytesN<32>) -> Result<Donation, soroban_sdk::Error> {
        env.storage().persistent().get(&DataKey::Donation(donation_id))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::CampaignNotFound as u32)) // Reusing error code
    }

    /// Get KYC record for an address
    pub fn get_kyc_record(env: Env, entity: Address) -> Result<KYCRecord, soroban_sdk::Error> {
        env.storage().persistent().get(&DataKey::KYCRecord(entity))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::KYCNotVerified as u32))
    }

    /// Get medical documentation details
    pub fn get_medical_doc(env: Env, doc_id: BytesN<32>) -> Result<MedicalDocumentation, soroban_sdk::Error)> {
        env.storage().persistent().get(&DataKey::MedicalDoc(doc_id))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::InvalidMedicalDoc as u32))
    }

    /// Get Dynamic NFT details for a given ID
    pub fn get_dynamic_nft(env: Env, nft_id: BytesN<32>) -> Result<DynamicNFT, soroban_sdk::Error> {
        env.storage().persistent().get(&DataKey::DynamicNFT(nft_id))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::CampaignNotFound as u32)) // Reusing error code
    }

    /// Get Trust Score for an address
    pub fn get_trust_score(env: Env, entity: Address) -> Result<TrustScore, soroban_sdk::Error> {
        env.storage().persistent().get(&DataKey::TrustScore(entity))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::ScoreExists as u32)) // Reusing error code
    }

    /// Get SPEI Transaction details
    pub fn get_spei_transaction(env: Env, tx_id: BytesN<32>) -> Result<SPEITransaction, soroban_sdk::Error> {
        env.storage().persistent().get(&DataKey::SPEITransaction(tx_id))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::SPEIError as u32)) // Reusing error code
    }

    /// Get Staking Pool details
    pub fn get_staking_pool(env: Env, pool_id: BytesN<32>) -> Result<StakingPool, soroban_sdk::Error> {
        env.storage().persistent().get(&DataKey::StakingPool(pool_id))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::StakingPoolNotFound as u32))
    }

    /// Get Staking Position details for a user in a pool
    pub fn get_staking_position(env: Env, user: Address, pool_id: BytesN<32>) -> Result<StakingPosition, soroban_sdk::Error> {
        env.storage().persistent().get(&DataKey::StakingPosition(user, pool_id))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::StakingPoolNotFound as u32)) // Reusing error code
    }

    /// Get Wallet Connection details for a user
    pub fn get_wallet_connection(env: Env, user: Address) -> Result<WalletConnection, soroban_sdk::Error> {
        env.storage().persistent().get(&DataKey::WalletConnection(user))
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::WalletNotConnected as u32))
    }

    /// Get current platform statistics
    pub fn get_platform_stats(env: Env) -> Result<PlatformStats, soroban_sdk::Error> {
        env.storage().instance().get(&DataKey::PlatformStats)
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::CampaignNotFound as u32)) // Reusing error code
    }

    /// Get current platform fee
    pub fn get_platform_fee(env: Env) -> Result<u64, soroban_sdk::Error> {
        env.storage().instance().get(&DataKey::PlatformFee)
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::InvalidFee as u32)) // Reusing error code
    }

    /// Get current staking reward rate
    pub fn get_staking_reward_rate(env: Env) -> Result<u64, soroban_sdk::Error> {
        env.storage().instance().get(&DataKey::StakingRewardRate)
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::InvalidRewardRate as u32)) // Reusing error code
    }

    /// Get current peso exchange rate
    pub fn get_peso_exchange_rate(env: Env) -> Result<u64, soroban_sdk::Error> {
        env.storage().instance().get(&DataKey::PesoExchangeRate)
            .ok_or(soroban_sdk::Error::from_contract_error(SaviaError::SPEIError as u32)) // Reusing error code
    }

    /// Get current emergency pause status
    pub fn get_emergency_pause_status(env: Env) -> Result<bool, soroban_sdk::Error> {
        Ok(env.storage().instance().get(&DataKey::EmergencyPause).unwrap_or(false))
    }

    /// Get list of admin addresses
    pub fn get_admin_addresses(env: Env) -> Result<Vec<Address>, soroban_sdk::Error> {
        Ok(env.storage().instance().get(&DataKey::AdminAddresses).unwrap_or(Vec::new(&env)))
    }

    /// Get list of KYC verifier addresses
    pub fn get_kyc_verifiers(env: Env) -> Result<Vec<Address>, soroban_sdk::Error> {
        Ok(env.storage().instance().get(&DataKey::KYCVerifiers).unwrap_or(Vec::new(&env)))
    }

    /// Get list of Medical verifier addresses
    pub fn get_medical_verifiers(env: Env) -> Result<Vec<Address>, soroban_sdk::Error> {
        Ok(env.storage().instance().get(&DataKey::MedicalVerifiers).unwrap_or(Vec::new(&env)))
    }

    /// Get total staking rewards distributed
    pub fn get_total_staking_rewards(env: Env) -> Result<u64, soroban_sdk::Error> {
        Ok(env.storage().instance().get(&DataKey::TotalStakingRewards).unwrap_or(0))
    }

    /// Get max campaign duration
    pub fn get_max_campaign_duration(env: Env) -> Result<u64, soroban_sdk::Error> {
        Ok(env.storage().instance().get(&DataKey::MaxCampaignDuration).unwrap_or(365))
    }

    /// Get min donation amount
    pub fn get_min_donation_amount(env: Env) -> Result<u64, soroban_sdk::Error> {
        Ok(env.storage().instance().get(&DataKey::MinDonationAmount).unwrap_or(1000000))
    }

    /// Get KYC required for donation status
    pub fn get_kyc_required_for_donation(env: Env) -> Result<bool, soroban_sdk::Error> {
        Ok(env.storage().instance().get(&DataKey::KYCRequiredForDonation).unwrap_or(true))
    }
    /// Get the current campaign counter
    pub fn get_campaign_counter(env: Env) -> Result<u64, soroban_sdk::Error> {
        Ok(env.storage().instance().get(&DataKey::CampaignCounter).unwrap_or(0))
    }

    /// Get the current donation counter
    pub fn get_donation_counter(env: Env) -> Result<u64, soroban_sdk::Error> {
        Ok(env.storage().instance().get(&DataKey::DonationCounter).unwrap_or(0))
    }

    /// Get the current NFT counter
    pub fn get_nft_counter(env: Env) -> Result<u64, soroban_sdk::Error> {
        Ok(env.storage().instance().get(&DataKey::NFTCounter).unwrap_or(0))
    }

    /// Get the current medical document counter
    pub fn get_medical_doc_counter(env: Env) -> Result<u64, soroban_sdk::Error> {
        Ok(env.storage().instance().get(&DataKey::MedicalDocCounter).unwrap_or(0))
    }

    /// Get the current staking pool counter
    pub fn get_staking_pool_counter(env: Env) -> Result<u64, soroban_sdk::Error> {
        Ok(env.storage().instance().get(&DataKey::StakingPoolCounter).unwrap_or(0))
    }
}
mod test;
