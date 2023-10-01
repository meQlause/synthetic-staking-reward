use scrypto::prelude::*;

#[blueprint]
mod staking_reward {
    struct StakingReward {
        staked_token: Vault,
        reward_token: Vault,
        reward_before_added_new_stake: HashMap<ComponentAddress, Vault>,
        user_balances: HashMap<ComponentAddress, Decimal>,
        user_reward_per_token: HashMap<ComponentAddress, Decimal>,
        reward_per_token: Decimal,
        reward_rate_per_epoch: Decimal,
        last_epoch_state: Epoch,
    }

    impl StakingReward {
        pub fn instantiate(reward_per_epoch: Decimal) -> Global<StakingReward> {
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(StakingReward::blueprint_id());
            let token = ResourceBuilder::new_fungible(OwnerRole::None)
                .mint_roles(mint_roles! {
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all);
                })
                .create_with_no_initial_supply()
                .create_empty_bucket();

            Self {
                user_balances: HashMap::new(),
                user_reward_per_token: HashMap::new(),
                reward_before_added_new_stake: HashMap::new(),
                staked_token: Vault::new(XRD),
                reward_token: Vault::with_bucket(token),
                reward_per_token: Decimal::ZERO,
                reward_rate_per_epoch: reward_per_epoch,
                last_epoch_state: Runtime::current_epoch(),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .with_address(address_reservation)
            .globalize()
        }

        pub fn stake(&mut self, address: ComponentAddress, xrd_to_staked: Bucket) {
            assert!(
                self.staked_token.resource_address() == xrd_to_staked.resource_address(),
                "Staked token must be in XRD"
            );
            assert!(
                xrd_to_staked.amount() > dec!(0),
                "Please provided more than 0 XRD(s)"
            );

            let stake_amount = xrd_to_staked.amount();

            if let Some(balances) = self.user_balances.get_mut(&address) {
                let new_balances = balances.to_owned() + stake_amount;
                let (reward, stake) = self.withdraw(address);
                self.set_user_reward_per_token_for_address(&address);
                self.user_balances.insert(address, new_balances);
                if let Some(vault) = self.reward_before_added_new_stake.get_mut(&address) {
                    vault.put(reward);
                } else {
                    self.reward_before_added_new_stake
                        .insert(address, Vault::with_bucket(reward));
                }
                self.staked_token.put(stake)
            } else {
                self.set_user_reward_per_token_for_address(&address);
                self.user_balances.insert(address, stake_amount);
            }
            self.staked_token.put(xrd_to_staked);
            self.set_reward_per_token();
        }

        pub fn withdraw(&mut self, address: ComponentAddress) -> (Bucket, Bucket) {
            let user_staked_amount = self
                .user_balances
                .get(&address)
                .expect("Address not found on staked list")
                .to_owned();

            let user_reward_pertoken = self
                .user_reward_per_token
                .get(&address)
                .expect("Address not found on staked list")
                .to_owned();
            self.set_reward_per_token();
            let mut reward = self.reward_token.resource_manager().mint(
                self.reward_rate_per_epoch
                    * user_staked_amount.to_owned()
                    * (self.reward_per_token - user_reward_pertoken.to_owned()),
            );
            if let Some(vault) = self.reward_before_added_new_stake.get_mut(&address) {
                reward.put(vault.take_all());
            }

            let stake = self.staked_token.take(user_staked_amount.to_owned());

            self.user_balances.remove(&address);
            self.user_reward_per_token.remove(&address);
            (reward, stake)
        }

        fn set_user_reward_per_token_for_address(&mut self, address: &ComponentAddress) {
            self.user_reward_per_token
                .entry(address.to_owned())
                .or_insert(self.reward_per_token.to_owned());
        }

        fn set_reward_per_token(&mut self) {
            let user_stake_duration = Decimal::from(
                Runtime::current_epoch()
                    .number()
                    .checked_sub(self.last_epoch_state.number().to_owned())
                    .expect("Error Calculating Epoch Duration"),
            );
            self.last_epoch_state = Runtime::current_epoch().to_owned();
            self.reward_per_token += (dec!(1) / self.staked_token.amount()) * user_stake_duration;
        }
    }
}
