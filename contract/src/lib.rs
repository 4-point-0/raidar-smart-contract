use near_contract_standards::non_fungible_token::core::{
    NonFungibleTokenCore, NonFungibleTokenResolver,
};
use near_contract_standards::non_fungible_token::enumeration::NonFungibleTokenEnumeration;
use near_contract_standards::non_fungible_token::events::{NftBurn, NftMint, NftTransfer};
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_tools::{owner::*, Owner};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, UnorderedMap};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::env::predecessor_account_id;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, near_bindgen, require, AccountId, Balance, BorshStorageKey,
    PanicOnDefault, Promise, PromiseOrValue,
};
use std::collections::HashMap;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SongCreation {
    pub token_id: TokenId,
    pub name: String,
    pub description: String,
    pub extra: Option<String>,
    pub price: U128,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SimpleNonFungibleToken {
    pub token_id: TokenId,
    pub name: String,
    pub description: String,
    pub extra: Option<String>,
    pub price: U128,
    pub creator: AccountId,
}

#[near_bindgen]
#[derive(Owner, BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub tokens: LookupMap<TokenId, SimpleNonFungibleToken>,
    pub metadata: LazyOption<NFTContractMetadata>,
    pub tokens_per_owner: UnorderedMap<AccountId, String>,
    pub whitelisted_creators: UnorderedSet<AccountId>,
}

const DATA_ICON_SVG: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGcAAABnCAMAAAAqn6zLAAABmFBMVEXuJDzuJDz////92t/uKD/6wsnuJDzuK0PwQFX1iZXuJDzuJT3uJj7uJz7uJz/uKUHuKkHvK0LvLEPvLUTvLkXvMUfvMkjvM0nvM0rvNEvvN03wOU7wOU/wOlDwO1HwPVLwPlPwP1TwQ1jxRVrxRlvxSV3xSl7xS17xTF/xTGDxTmHxT2LxUGPyUmXyVWjyV2nyW23zXnDzX3HzYXLzYnPzZHXzZXbzaHjzanv0cH/0dIP1fIr1fIv1foz1f431gI71g5H2hpP2iJX2ipf2jpr2jpv2kJz2kZ33kp73laD3laH3mqX3m6b3nKf3naj4oKr4oaz4p7H5rLX5rrf5r7j5sLn5sbn5s7v5tb35tr75uMD6u8P6v8b6wsn6w8n6xMr6xcv7x837yc/7y9H70NX70db809j81tr819v819z82d382t7829/83eD83uL93+P94OP94eT94uX95Of95+r96Or96ev96uz+7O7+7e/+7/D+8fP+8/T+9PX+9fb+9vf+9/j/+fr/+vr//Pz//P3//f3//v7///+AcWUyAAAACnRSTlPH3d3f4eHl5evvxwPreAAAAAFiS0dEh/vZC8sAAAHhSURBVGje7dpVU8NAFAXg4nBL0eJOseLBCe4UK1DcHYq7O4T92zxQIClJyszuFpl7Xvfcfg/7kLszNfgo4h9ClAn2YRKDAZRpdHPigUkC0UEHHXTQQQcddNBBB53f4aQ2jK+vLXbmcHXM4vzzW0UaS+LlhJYM3chKR1lcnLz+U7fWcRxzJ6XVSb5mjNZJUJxGi3PPRC1SOqWT/XlkKhu5JVppp3Rs7we5fSdEJxOUzkM5AEByywbRzwqlQ6SpplaNS5Fni9b5ZtBB588718Ol5lXejrQgmgFggK+z1eb6ePJ0Tu0FH0Vuzv1kVZisyMeRluvMyiIPZ7sr48skc+fMXqI2ydhZqjCpTzJ28rUmGTvZ6PxNZ9g7TtiBd5xu4g0nc5Twd2LFhRfC2zEJjjse+4HSKbSfc9pDZE5a2y6/fefdiVG7FNaO1qUwdKwARYMX/PfE2Z4d3K/R+Wlnv9PJ37lyCEZYonPqPSGP0zURAEDrVHt6/iS6ipSOVQc5slk+i5SO8VDrTeoQjPIipQMdasjTTG2UW4/WidzUfpOydMB6qfiBk171lYfaAcvex/StQwjVmKR3ILx5nxBC7ufEaO1JBg4AZAiVxeG6k2wcz0EHHXTQ+a9OgK8ifkH6/7d8BeLgry/RqHfGAAAAAElFTkSuQmCC";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Tokens,
    Metadata,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta() -> Self {
        Self::new(NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(),
            name: "Raidar".to_string(),
            symbol: "RAIDR".to_string(),
            icon: Some(DATA_ICON_SVG.to_string()),
            base_uri: Some("https://raidar.us/api/v1".to_string()),
            reference: None,
            reference_hash: None,
        })
    }

    #[init]
    pub fn new(metadata: NFTContractMetadata) -> Self {
        require!(!env::state_exists(), "Already initialized");

        metadata.assert_valid();

        let mut contract = Self {
            tokens: LookupMap::new(StorageKey::Tokens),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            tokens_per_owner: UnorderedMap::new(b"t".to_vec()),
            whitelisted_creators: UnorderedSet::new(b"w".to_vec()),
        };

        Owner::init(&mut contract, &predecessor_account_id());

        contract
    }

    // Whitelisting
    #[payable]
    pub fn add_whitelisted_creator(&mut self, creator: AccountId) {
        assert_one_yocto();
        Self::require_owner();
        self.whitelisted_creators.insert(&creator);
    }

    #[payable]
    pub fn remove_whitelisted_creator(&mut self, creator: AccountId) {
        assert_one_yocto();
        Self::require_owner();
        self.whitelisted_creators.remove(&creator);
    }

    pub fn get_whitelist(&self) -> Vec<AccountId> {
        self.whitelisted_creators.to_vec()
    }

    #[payable]
    pub fn update_base_url(&mut self, url: String) -> NFTContractMetadata {
        Self::require_owner();

        self.metadata = LazyOption::new(
            StorageKey::Metadata,
            Some(&NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: self.metadata.get().unwrap().name.to_string(),
                symbol: self.metadata.get().unwrap().symbol.to_string(),
                icon: self.metadata.get().unwrap().icon,
                base_uri: Some(url),
                reference: None,
                reference_hash: None,
            }),
        );

        self.metadata.get().unwrap()
    }

    #[payable]
    pub fn update_icon(&mut self, svg_data: String) -> NFTContractMetadata {
        Self::require_owner();

        self.metadata = LazyOption::new(
            StorageKey::Metadata,
            Some(&NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: self.metadata.get().unwrap().name.to_string(),
                symbol: self.metadata.get().unwrap().symbol.to_string(),
                icon: Some(svg_data),
                base_uri: self.metadata.get().unwrap().base_uri,
                reference: None,
                reference_hash: None,
            }),
        );

        self.metadata.get().unwrap()
    }

    #[payable]
    pub fn mint_nft(&mut self, data: SongCreation) -> SimpleNonFungibleToken {
        let initial_storage_usage = env::storage_usage();
        let receiver_id = env::predecessor_account_id();

        // assert!(
        //     self.whitelisted_creators.contains(&receiver_id),
        //     "The account is not whitelisted"
        // );

        let token_data = SimpleNonFungibleToken {
            token_id: data.token_id.clone(),
            name: data.name.clone(),
            description: data.description.clone(),
            extra: data.extra.clone(),
            price: data.price.clone(),
            creator: receiver_id,
        };

        self.tokens.insert(&data.token_id, &token_data);

        self.refund_deposit(env::storage_usage() - initial_storage_usage, 0);

        token_data
    }

    #[payable]
    pub fn buy_for_user(&mut self, token_id: &TokenId, account_id: &AccountId) -> TokenMetadata {
        Self::require_owner();

        let _token = self.tokens.get(token_id).expect("Token not found");

        let metadata = self.internal_as_nft_metadata(token_id.clone());

        //get the set of tokens for the given account
        let tokens_set = self
            .tokens_per_owner
            .get(&account_id)
            .unwrap_or("".to_string());

        // check if tokens_set is empty string and create a vector of tokens
        let mut ids = if tokens_set == "" {
            vec![]
        } else {
            tokens_set.split(":").collect::<Vec<&str>>()
        };

        //we insert the token ID into the set
        ids.push(&token_id);

        self.tokens_per_owner.insert(&account_id, &ids.join(":"));

        let actual_token_id = format!("{}:{}", account_id.clone(), token_id.clone());

        NftMint {
            owner_id: &account_id,
            token_ids: &[actual_token_id.as_ref()],
            memo: None,
        }
        .emit();

        metadata
    }

    #[payable]
    pub fn buy_nft(&mut self, token_id: &TokenId) -> TokenMetadata {
        let initial_storage_usage = env::storage_usage();
        let attached_deposit = env::attached_deposit();
        let receiver_id = env::predecessor_account_id();

        let token = self.tokens.get(token_id).expect("Token not found");

        assert!(
            attached_deposit >= token.price.0,
            "Marketplace: attached deposit is less than price : {}",
            token.price.0
        );

        Promise::new(token.creator).transfer(token.price.0);

        let metadata = self.internal_as_nft_metadata(token_id.clone());

        //get the set of tokens for the given account
        let tokens_set = self
            .tokens_per_owner
            .get(&receiver_id)
            .unwrap_or("".to_string());

        // check if tokens_set is empty string and create a vector of tokens
        let mut ids = if tokens_set == "" {
            vec![]
        } else {
            tokens_set.split(":").collect::<Vec<&str>>()
        };

        //we insert the token ID into the set
        ids.push(&token_id);

        self.tokens_per_owner.insert(&receiver_id, &ids.join(":"));

        let actual_token_id = format!("{}:{}", receiver_id.clone(), token_id.clone());

        NftMint {
            owner_id: &receiver_id,
            token_ids: &[actual_token_id.as_ref()],
            memo: None,
        }
        .emit();

        self.refund_deposit(env::storage_usage() - initial_storage_usage, token.price.0);

        metadata
    }

    #[payable]
    pub fn burn_nft(&mut self, account_id: &AccountId, token_id: &TokenId) {
        Self::require_owner();

        let binding = self
            .tokens_per_owner
            .get(account_id)
            .expect("The account doesn't have any tokens");

        let tokens_set = binding.split(":");

        assert!(
            tokens_set
                .clone()
                .collect::<Vec<&str>>()
                .contains(&token_id.as_str()),
            "Token should be owned by the account",
        );

        let filtered_set = tokens_set
            .filter(|x| x.clone().ne(token_id))
            .collect::<Vec<&str>>();

        //if the token set is now empty, we remove the owner from the tokens_per_owner collection
        if filtered_set.is_empty() {
            self.tokens_per_owner.remove(account_id);
        } else {
            //if the token set is not empty, we simply insert it back for the account ID.
            self.tokens_per_owner
                .insert(account_id, &filtered_set.join(":"));
        }

        let actual_token_id = format!("{}:{}", account_id.clone(), token_id.clone());

        NftBurn {
            owner_id: &account_id,
            token_ids: &[actual_token_id.as_ref()],
            authorized_id: None,
            memo: None,
        }
        .emit();
    }

    pub fn nft_token_metadata(&self, token_id: TokenId) -> TokenMetadata {
        //we split the token_id into the account_id and the token_id
        let parts: Vec<&str> = token_id.split(':').collect();

        //we get the account_id from the first part of the split
        let owner_id: AccountId = parts[0].to_string().parse().unwrap();

        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "Invalid account"
        );

        // we get the token_id from the second part of the split
        let actual_token_id = parts[1].to_string();

        let _token = self.tokens.get(&actual_token_id).expect("Token not found");

        self.internal_as_nft_metadata(actual_token_id.clone())
    }

    // Private methods (internal)
    fn internal_as_nft_metadata(&self, token_id: TokenId) -> TokenMetadata {
        let token = self
            .tokens
            .get(&token_id)
            .expect("Token doesn't exists")
            .clone();

        TokenMetadata {
            title: Some(token.name),
            description: Some(token.description),
            media: Some(format!("song/{}/media", token.token_id)),
            media_hash: None,
            copies: None,
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: token.extra,
            reference: None,
            reference_hash: None,
        }
    }

    fn refund_deposit(&self, storage_used: u64, extra_spend: Balance) {
        let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
        let attached_deposit = env::attached_deposit() - extra_spend;

        assert!(
            required_cost <= attached_deposit,
            "Must attach {} yoctoNEAR to cover storage",
            required_cost,
        );

        let refund = attached_deposit - required_cost;
        if refund > 1 {
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }
    }
}

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {
    // token_id is the unique identifier for the token and for this method it will come in the format of "account_id:token_id"
    fn nft_token(&self, token_id: TokenId) -> Option<Token> {
        //we split the token_id into the account_id and the token_id
        let parts: Vec<&str> = token_id.split(':').collect();

        //we get the account_id from the first part of the split
        let owner_id: AccountId = parts[0].to_string().parse().unwrap();

        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "Invalid account"
        );

        // we get the token_id from the second part of the split
        let actual_token_id = parts[1].to_string();

        //if there is some token ID in the tokens_by_id collection
        if let Some(_token) = self.tokens.get(&actual_token_id) {
            //we'll get the metadata for that token
            let metadata = self.internal_as_nft_metadata(actual_token_id.clone());
            //we return the Token (wrapped by Some since we return an option)
            Some(Token {
                token_id,
                owner_id,
                metadata: Some(metadata),
                approved_account_ids: None,
            })
        } else {
            //if there wasn't a token with the specified ID we return None
            None
        }
    }

    #[payable]
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
        env::panic_str("NFT transfer is not supported for soulbound NFTs");
    }

    #[payable]
    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool> {
        env::panic_str("NFT transfer is not supported for soulbound NFTs");
    }
}

#[near_bindgen]
impl NonFungibleTokenResolver for Contract {
    #[private]
    fn nft_resolve_transfer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        approved_account_ids: Option<HashMap<AccountId, u64>>,
    ) -> bool {
        env::panic_str("NFT transfer is not supported for soulbound NFTs");
    }
}

#[near_bindgen]
impl NonFungibleTokenEnumeration for Contract {
    fn nft_total_supply(&self) -> U128 {
        U128(
            self.tokens_per_owner
                .values()
                .fold(0, |cumulative, current_set| {
                    cumulative + current_set.split(":").collect::<Vec<&str>>().len()
                }) as u128,
        )
    }

    fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Token> {
        let start = u128::from(from_index.unwrap_or(U128(0)));

        let accounts = self
            .tokens_per_owner
            .keys()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize);

        let tokens = accounts
            .map(|account_id| {
                self.tokens_per_owner
                    .get(&account_id)
                    .expect("Account ID not found")
                    .split(":")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|token_id| {
                        self.nft_token(format!("{}:{}", account_id.clone(), token_id.clone()))
                    })
                    .collect::<Vec<Option<Token>>>()
            })
            .flatten()
            .flatten()
            .collect::<Vec<Token>>();

        tokens
    }

    // Get the total supply of NFTs for a given owner
    fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
        //get the set of tokens for the given account
        let tokens_set = self
            .tokens_per_owner
            .get(&account_id)
            .unwrap_or("".to_string());

        // check if tokens_set is empty string and create a vector of tokens
        let ids = if tokens_set == "" {
            vec![]
        } else {
            tokens_set.split(":").collect::<Vec<&str>>()
        };

        U128(ids.len() as u128)
    }

    // Query for all the tokens for an owner
    fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Token> {
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id);

        let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            tokens_for_owner_set
        } else {
            return vec![];
        };

        let start = u128::from(from_index.unwrap_or(U128(0)));

        let list = tokens.split(":").collect::<Vec<&str>>();

        list.iter()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .map(|token_id| {
                self.nft_token(format!("{}:{}", account_id.clone(), token_id.clone()))
                    .expect(format!("Token {} not found. List: {:?}", token_id, list).as_str())
            })
            .collect()
    }
}
