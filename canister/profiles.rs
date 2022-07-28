use candid::Principal;
use ic_cdk::export::candid::CandidType;
use ic_ledger_types::AccountIdentifier;
use serde::*;
use std::cell::RefCell;
use std::collections::HashMap;

// Map<pid, Profile>
thread_local! {
    static PROFILE: RefCell<HashMap<Principal, Profile>>  = RefCell::new(HashMap::new());
}

#[derive(PartialEq, Clone, Serialize, Deserialize, CandidType)]
pub struct PID(Principal);

impl Default for PID {
    fn default() -> Self {
        Self(Principal::anonymous())
    }
}

impl PID {
    fn new(pid: Principal) -> Self {
        Self(pid)
    }
}

#[derive(PartialEq, Clone, Default, Serialize, Deserialize, CandidType)]
pub struct Profile {
    pub url: String,
    pub pid: PID,
    pub aid: String,
    pub nick_name: String,
    pub description: String,
    pub level: String,
    pub state: String,
}

impl Profile {
    fn new(nick_name: String, pid: PID, aid: String) -> Self {
        Self {
            pid,
            aid,
            nick_name,
            ..Default::default()
        }
    }
}

pub fn register(nick_name: String) -> Option<Profile> {
    let pid = ic_cdk::api::caller();
    let sub_acc = ic_ledger_types::Subaccount([0u8; 32]);
    let aid = AccountIdentifier::new(&pid, &sub_acc).to_string();

    let my_profile = Profile::new(nick_name, PID::new(pid), aid);

    PROFILE.with(|profile| {
        profile
            .borrow_mut()
            .entry(pid)
            .or_insert(my_profile.clone());
    });

    Some(my_profile)
}

pub fn all() -> Option<Vec<Principal>> {
    Some(PROFILE.with(|map| map.borrow().keys().map(|&pid| pid).collect()))
}

// get someone's profile
pub fn get_profile(pid: Principal) -> Option<Profile> {
    PROFILE.with(|profile| profile.borrow().get(&pid).cloned())
}

pub fn set_description(pid: Principal, desc: String) -> Option<Profile> {
    let caller = ic_cdk::api::caller();
    assert_eq!(pid, caller);

    PROFILE.with(|profile| {
        if let Some(mut profile) = profile.borrow_mut().get_mut(&pid) {
            profile.description = desc;
            Some(profile.clone())
        } else {
            None
        }
    })
}

pub fn set_url(pid: Principal, urlink: String) -> Option<Profile> {
    let caller = ic_cdk::api::caller();
    assert_eq!(pid, caller);
    PROFILE.with(|profile| {
        if let Some(mut profile) = profile.borrow_mut().get_mut(&pid) {
            profile.url = urlink;
            Some(profile.clone())
        } else {
            None
        }
    })
}

pub fn set_nickname_desc(pid: Principal, nickname: String, desc: String) -> Option<Profile> {
    let caller = ic_cdk::api::caller();
    assert_eq!(pid, caller);

    PROFILE.with(|profile| {
        if let Some(mut profile) = profile.borrow_mut().get_mut(&pid) {
            profile.nick_name = nickname;
            profile.description = desc;
            Some(profile.clone())
        } else {
            None
        }
    })
}

pub fn get(pid: Principal) -> Option<Profile> {
    PROFILE.with(|profile| profile.borrow().get(&pid).cloned())
}

pub fn get_url(pid: Principal) -> Option<String> {
    PROFILE.with(|profile| Some(profile.borrow().get(&pid)?.url.clone()))
}
