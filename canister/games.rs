use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::sync::atomic::{self, AtomicUsize};
pub type URL = String;
pub type PID = Principal;

static TID: AtomicUsize = AtomicUsize::new(0);
static GID: AtomicUsize = AtomicUsize::new(0);

const NUM_OF_PAGE: usize = 10;
const NUM_OF_TOP: usize = 5;

// generate new team id
fn new_tid() -> String {
    let cid = ic_cdk::api::id();

    let tid = TID.fetch_add(1, atomic::Ordering::SeqCst);
    format!("{}-{}", cid.to_text(), tid + 1)
}

// generate new game id
fn new_gid() -> String {
    let cid = ic_cdk::api::id();

    let gid = GID.fetch_add(1, atomic::Ordering::SeqCst);
    format!("{}-{}", cid.to_text(), gid + 1)
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct TeamInfo {
    pub id: String,
    pub operator: PID,
    pub logo: URL,
    pub name: String,
    pub details: Option<Details>,
}

impl TeamInfo {
    pub fn new(operator: PID, logo: URL, name: String, details: Option<Details>) -> Self {
        Self {
            id: new_tid(),
            operator,
            logo,
            name,
            details,
        }
    }
}

impl TeamInfo {
    pub fn set_operator(&mut self, id: String, operator: PID) -> &Self {
        assert_eq!(self.id, id);
        self.operator = operator;
        self
    }
    pub fn set_logo(&mut self, id: String, logo: URL) -> &Self {
        assert_eq!(self.id, id);
        self.logo = logo;
        self
    }
    pub fn set_name(&mut self, id: String, name: String) -> &Self {
        assert_eq!(self.id, id);
        self.name = name;
        self
    }
    pub fn set_details(&mut self, id: String, details: Option<Details>) -> &Self {
        assert_eq!(self.id, id);
        self.details = details;
        self
    }
}

#[derive(Debug, Clone, Default, CandidType, Serialize, Deserialize)]
pub struct Details {
    pub description: Option<String>,
    pub title: Option<String>,
    pub url: Option<URL>,
}

#[derive(Debug, Clone, Default, CandidType, Serialize, Deserialize)]
pub struct GameInfo {
    pub id: String,
    pub name: String,
    pub details: Vec<Details>,
    pub url: URL,
    pub developer: Option<TeamInfo>,
    pub publisher: Option<TeamInfo>,
    pub description: String,
    pub summary: String,
}

impl GameInfo {
    pub fn new(
        name: String,
        detail: Details,
        url: URL,
        developer: Option<TeamInfo>,
        publisher: Option<TeamInfo>,
        description: String,
        summary: String,
    ) -> Self {
        Self {
            id: new_gid(),
            name,
            details: vec![detail],
            url,
            developer,
            publisher,
            description,
            summary,
        }
    }
}

impl GameInfo {
    pub fn add_detail(&mut self, detail: Details) -> &Self {
        self.details.push(detail);
        self
    }

    pub fn set_developer(&mut self, developer: TeamInfo) -> &Self {
        self.developer = Some(developer);
        self
    }

    pub fn set_publisher(&mut self, publisher: TeamInfo) -> &Self {
        self.publisher = Some(publisher);
        self
    }

    pub fn set_name(&mut self, name: String) -> &Self {
        self.name = name;
        self
    }

    pub fn set_url(&mut self, url: URL) -> &Self {
        self.url = url;

        self
    }

    pub fn set_summary(&mut self, _summary: String) -> &Self {
        self.summary = _summary;

        self
    }
}

thread_local! {
    static GAMES: RefCell<Vec<GameInfo>> = RefCell::new(Vec::new());
    static LEVEL_ONE_GAMES: RefCell<Vec<GameInfo>> = RefCell::new(Vec::new());
    static LEVEL_TWO_GAMES: RefCell<Vec<GameInfo>> = RefCell::new(Vec::new());
}

pub fn add(
    name: String,
    detail: Details,
    url: URL,
    developer: Option<TeamInfo>,
    publisher: Option<TeamInfo>,
    description: String,
    summary: String,
) -> GameInfo {
    let game = GameInfo::new(
        name,
        detail,
        url,
        developer,
        publisher,
        description,
        summary,
    );
    GAMES.with(|games| games.borrow_mut().push(game.clone()));

    game
}

pub fn add_level_one(
    name: String,
    detail: Details,
    url: URL,
    developer: Option<TeamInfo>,
    publisher: Option<TeamInfo>,
    description: String,
    summary: String,
) -> GameInfo {
    let game = GameInfo::new(
        name,
        detail,
        url,
        developer,
        publisher,
        description,
        summary,
    );
    LEVEL_ONE_GAMES.with(|games| games.borrow_mut().push(game.clone()));
    GAMES.with(|games| games.borrow_mut().push(game.clone()));

    game
}

pub fn add_level_two(
    name: String,
    detail: Details,
    url: URL,
    developer: Option<TeamInfo>,
    publisher: Option<TeamInfo>,
    description: String,
    summary: String,
) -> GameInfo {
    let game = GameInfo::new(
        name,
        detail,
        url,
        developer,
        publisher,
        description,
        summary,
    );
    LEVEL_TWO_GAMES.with(|games| games.borrow_mut().push(game.clone()));
    GAMES.with(|games| games.borrow_mut().push(game.clone()));

    game
}

pub fn info_level_one() -> Vec<GameInfo> {
    LEVEL_ONE_GAMES.with(|games| {
        let v = games.clone().into_inner();
        let i = std::cmp::min(v.len(), NUM_OF_TOP);
        v[..i].to_vec()
    })
}

pub fn info_level_two() -> Vec<GameInfo> {
    LEVEL_TWO_GAMES.with(|games| {
        let v = games.clone().into_inner();
        let i = std::cmp::min(v.len(), NUM_OF_TOP);
        v[..i].to_vec()
    })
}

pub fn info(page: usize, num_of_page: Option<usize>) -> Vec<GameInfo> {
    assert!(page > 0);

    let num_of_page = num_of_page.unwrap_or(NUM_OF_PAGE);
    GAMES.with(|games| {
        let v = games.clone().into_inner();
        let start = (page - 1) * num_of_page;
        let end = std::cmp::min(v.len(), page * num_of_page);
        assert!(start < end);
        v[start..end].to_vec()
    })
}

pub fn info_by_id(id: String) -> Option<GameInfo> {
    GAMES.with(|games| {
        games
            .clone()
            .into_inner()
            .iter()
            .find(|&g| g.id == id)
            .cloned()
    })
}

pub fn add_detail(id: String, detail: Details) -> Option<GameInfo> {
    GAMES.with(|games| {
        games
            .borrow_mut()
            .iter_mut()
            .filter(|game| id == game.id)
            .map(|game| game.add_detail(detail.clone()))
            .cloned()
            .nth(0)
    })
}

pub fn set_developer(id: String, developer: TeamInfo) -> Option<GameInfo> {
    let _developer = TeamInfo::new(
        developer.operator,
        developer.logo,
        developer.name,
        developer.details,
    );

    GAMES.with(|games| {
        games
            .borrow_mut()
            .iter_mut()
            .filter(|game| id == game.id)
            .map(|game| game.set_developer(_developer.clone()))
            .cloned()
            .nth(0)
    })
}

pub fn set_publisher(id: String, publisher: TeamInfo) -> Option<GameInfo> {
    let _publisher = TeamInfo::new(
        publisher.operator,
        publisher.logo,
        publisher.name,
        publisher.details,
    );

    GAMES.with(|games| {
        games
            .borrow_mut()
            .iter_mut()
            .filter(|game| id == game.id)
            .map(|game| game.set_publisher(_publisher.clone()))
            .cloned()
            .nth(0)
    })
}

pub fn set_name(id: String, name: String) -> Option<GameInfo> {
    GAMES.with(|games| {
        games
            .borrow_mut()
            .iter_mut()
            .filter(|game| id == game.id)
            .map(|game| game.set_name(name.clone()))
            .cloned()
            .nth(0)
    })
}

pub fn set_url(id: String, url: URL) -> Option<GameInfo> {
    GAMES.with(|games| {
        games
            .borrow_mut()
            .iter_mut()
            .filter(|game| id == game.id)
            .map(|game| game.set_url(url.clone()))
            .cloned()
            .nth(0)
    })
}
