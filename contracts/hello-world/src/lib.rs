#![no_std]
use soroban_sdk::{contractimpl, Env, Symbol, Address};

pub struct RWAContractManager;

#[contractimpl]
impl RWAContractManager {
    /// Khởi tạo contract quản lý với admin
    pub fn init(env: Env, admin: Address) {
        env.storage().persistent().set(&Symbol::short("admin"), &admin);
    }

    /// Đăng ký contract con (ví dụ AssetToken, Compliance, Yield…)
    pub fn register_contract(env: Env, name: Symbol, contract_id: Address) {
        let admin: Address = env.storage().persistent().get(&Symbol::short("admin")).unwrap();
        if env.invoker() != admin {
            panic!("Not authorized");
        }
        env.storage().persistent().set(&(Symbol::short("child"), name), &contract_id);
    }

    /// Lấy địa chỉ contract con theo tên
    pub fn get_contract(env: Env, name: Symbol) -> Address {
        env.storage().persistent().get(&(Symbol::short("child"), name)).unwrap()
    }

    /// Cập nhật contract con
    pub fn update_contract(env: Env, name: Symbol, new_contract_id: Address) {
        let admin: Address = env.storage().persistent().get(&Symbol::short("admin")).unwrap();
        if env.invoker() != admin {
            panic!("Not authorized");
        }
        env.storage().persistent().set(&(Symbol::short("child"), name), &new_contract_id);
    }

    /// Xoá contract con
    pub fn remove_contract(env: Env, name: Symbol) {
        let admin: Address = env.storage().persistent().get(&Symbol::short("admin")).unwrap();
        if env.invoker() != admin {
            panic!("Not authorized");
        }
        env.storage().persistent().remove(&(Symbol::short("child"), name));
    }
}