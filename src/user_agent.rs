use crate::user_agent_data::{
    CHROME_USER_AGENTS, EDGE_USER_AGENTS, FIREFOX_USER_AGENTS, IE_USER_AGENTS, OPERA_USER_AGENTS,
    SAFARI_USER_AGENTS,
};
use fastrand;

pub fn get_random_user_agent() -> String {
    let browsers = [
        get_chrome_user_agent,
        get_firefox_user_agent,
        get_ie_user_agent,
        get_opera_user_agent,
        get_safari_user_agent,
        get_edge_user_agent,
    ];
    let random_browser = browsers[fastrand::usize(0..browsers.len())];
    random_browser()
}

pub fn get_chrome_user_agent() -> String {
    CHROME_USER_AGENTS[fastrand::usize(0..CHROME_USER_AGENTS.len())].to_string()
}

pub fn get_firefox_user_agent() -> String {
    FIREFOX_USER_AGENTS[fastrand::usize(0..FIREFOX_USER_AGENTS.len())].to_string()
}

pub fn get_ie_user_agent() -> String {
    IE_USER_AGENTS[fastrand::usize(0..IE_USER_AGENTS.len())].to_string()
}

pub fn get_opera_user_agent() -> String {
    OPERA_USER_AGENTS[fastrand::usize(0..OPERA_USER_AGENTS.len())].to_string()
}

pub fn get_safari_user_agent() -> String {
    SAFARI_USER_AGENTS[fastrand::usize(0..SAFARI_USER_AGENTS.len())].to_string()
}

pub fn get_edge_user_agent() -> String {
    EDGE_USER_AGENTS[fastrand::usize(0..EDGE_USER_AGENTS.len())].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_chrome_rua() {
        let rua = get_chrome_user_agent();
        println!("{}", rua);
        assert!(CHROME_USER_AGENTS.contains(&rua.as_str()));
    }

    #[test]
    fn test_get_opera_rua() {
        let rua = get_opera_user_agent();
        assert!(OPERA_USER_AGENTS.contains(&rua.as_str()));
    }

    #[test]
    fn test_get_firefox_rua() {
        let rua = get_firefox_user_agent();
        assert!(FIREFOX_USER_AGENTS.contains(&rua.as_str()));
    }

    #[test]
    fn test_get_safari_rua() {
        let rua = get_safari_user_agent();
        assert!(SAFARI_USER_AGENTS.contains(&rua.as_str()));
    }

    #[test]
    fn test_get_edge_rua() {
        let rua = get_edge_user_agent();
        assert!(EDGE_USER_AGENTS.contains(&rua.as_str()));
    }

    #[test]
    fn test_get_ie_rua() {
        let rua = get_ie_user_agent();
        assert!(IE_USER_AGENTS.contains(&rua.as_str()));
    }

    #[test]
    fn test_get_rua() {
        let rua = get_random_user_agent();
        assert!(
            CHROME_USER_AGENTS.contains(&rua.as_str())
                || OPERA_USER_AGENTS.contains(&rua.as_str())
                || FIREFOX_USER_AGENTS.contains(&rua.as_str())
                || SAFARI_USER_AGENTS.contains(&rua.as_str())
                || EDGE_USER_AGENTS.contains(&rua.as_str())
                || IE_USER_AGENTS.contains(&rua.as_str())
        );
    }
}
