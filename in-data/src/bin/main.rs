use chrono::Utc;
use incore::graph::*;
use incore::user::*;

pub fn main() {
    let g = create_users();


    println!("{:#?}", g);
}

pub fn create_users() -> Graph<User, UserRelation> {
    let chris = User {
        data: UserData { exp: 0, level: 1 },
        username: "chris".into(),
        ..Default::default()
    };
    let keewa = User {
        data: UserData { exp: 33, level: 2 },
        username: "keewa".into(),
        ..Default::default()
    };
    let jazi = User {
        data: UserData { exp: 99, level: 5 },
        username: "jazzi".into(),
        ..Default::default()
    };
    let brian = User {
        data: UserData { exp: 3, level: 1 },
        username: "brian".into(),
        ..Default::default()
    };
    let fred = User {
        data: UserData { exp: 9, level: 2 },
        username: "frederic".into(),
        ..Default::default()
    };
    let loves = UserRelation {
        weight: 1000,
        text: Some(String::from("Loves")),
        created: Utc::now(),
        updated: Utc::now(),
    };
    let lovesq = UserRelation {
        weight: 1000,
        text: Some(String::from("Loves?")),
        created: Utc::now(),
        updated: Utc::now(),
    };
    let hates = UserRelation {
        weight: 1000,
        text: Some(String::from("Hates")),
        created: Utc::now(),
        updated: Utc::now(),
    };
    let respects = UserRelation {
        weight: 1000,
        text: Some(String::from("Respects")),
        created: Utc::now(),
        updated: Utc::now(),
    };
    let mut g = Graph::<User, UserRelation>::new(GraphKind::Directed);
    let chris = g.insert_node(chris);
    let keewa = g.insert_node(keewa);
    let jazi = g.insert_node(jazi);
    let brian = g.insert_node(brian);
    let fred = g.insert_node(fred);
    let chrisloveskeewa = g.insert_edge(loves.clone(), chris, keewa);
    let keewalovesqchris = g.insert_edge(lovesq.clone(), keewa, chris);
    let keewalovesjazi = g.insert_edge(loves.clone(), keewa, jazi);
    let chrisrespectsbrian = g.insert_edge(respects.clone(), chris, brian);
    let brianhatesfred = g.insert_edge(respects.clone(), brian, fred);
    let fredlovesqjazi = g.insert_edge(lovesq.clone(), fred, jazi);

    println!("chris and keewa: {:?}", g.get_edge(chris, keewa));
    println!("keewa and chris: {:?}", g.get_edge(keewa, chris));
    println!("keewa and jazi: {:?}", g.get_edge(keewa, jazi));
    println!("keewa and fred: {:?}", g.get_edge(keewa, fred));
    g

}
