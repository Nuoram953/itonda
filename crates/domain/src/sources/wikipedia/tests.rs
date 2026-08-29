use crate::sources::wikipedia::parser::parse_gameplay_wikitext;

#[test]
fn test_parse_gears_of_war_3_gameplay() {
    let raw = r#"==Gameplay==
Like its predecessors, ''Gears of War 3'' is a [[third-person shooter]] that emphasizes the use of cover and squad tactics in order to survive combat situations. The player's character can carry four weapons: one pistol, one set of grenades, and two primary weapons. Firearms can be swapped with other weapons dropped by fallen foes or at stockpiles throughout the game, along with ammunition. When the player reloads a weapons ammunition, they have an opportunity for an "active reload", shown by a small cursor moving over a line with a marked section on the player's [[heads-up display]] (HUD). If the player hits a control button when the cursor is in the marked section, they will reload faster with the resulting reload being slightly more powerful than normal bullets, causing more damage to opponents as well as allowing for more "knockback" meaning that enemies running directly at you will be slowed on shot.<ref>{{Cite web|url=test.com}}</ref>

When in combat, the player can take some damage from enemy fire, filling a blood-colored "crimson omen" on the HUD as a measure of the player's health, unlike the traditional health bar in other shooters. By staying out of the line of fire, this will dissipate over time, but by taking too much damage, the player will become downed or killed, and must be "revived" by an ally within a short "bleed-out" period, or else the player will die. Within ''Gears of War 3'', some cover can be destroyed after taking some amount of damage.

New to ''Gears of War 3'' is the ability to tag enemy opponents; computer-controlled allies will then concentrate fire on these marked enemies, while human allies will be alerted to their location on their HUD. Players can now also swap weapons and ammunition (and money during Horde Mode) with other allies in the course of battle.

The player maintains an [[experience point|experience level]] that persists across all game modes. The player earns experience through kills, performing special types of kills and executions, reviving and aiding teammates, and through general process of the campaign or competitive modes. Players can also gain experience points by winning medals and ribbons.
"#;

    let pillars = parse_gameplay_wikitext(raw);
    assert_eq!(pillars.len(), 4);

    assert_eq!(pillars[0].icon, "combat");
    assert_eq!(pillars[0].title, "Active Reload");
    assert!(pillars[0].description.contains("active reload") || pillars[0].description.contains("cover"));

    assert_eq!(pillars[1].icon, "survival");
    assert_eq!(pillars[1].title, "Health & Recovery");
    assert!(pillars[1].description.contains("damage") || pillars[1].description.contains("fire"));

    assert_eq!(pillars[2].icon, "coop");
    assert_eq!(pillars[2].title, "Squad Tagging & Coordination");
    assert!(pillars[2].description.contains("Tag enemy opponents") || pillars[2].description.contains("allies"));

    assert_eq!(pillars[3].icon, "progression");
    assert_eq!(pillars[3].title, "Experience & Medals");
    assert!(pillars[3].description.contains("experience") || pillars[3].description.contains("medals"));
}



#[test]
fn test_parse_kingdom_come_deliverance_gameplay() {
    let raw = r#"== Gameplay ==
[[File:Kingdom Come gameplay screenshot.jpg|thumb|Henry riding]]
''Kingdom Come: Deliverance'' is an [[action role-playing video game]] set in an [[open-world]] environment and played from a [[first-person perspective]]. It utilises a classless role-playing system, allowing the player to customise their skills. Abilities and stats grow depending on what the player does and says through branched dialogue trees. Reputation is based on player choices and therefore can bring consequences.<ref>test</ref>

The clothing system features 16 item slots and items on many areas of the body that can be layered. The player is able to use a variety of weapons, including swords, knives, axes, hammers, and bows. Horses are featured heavily in the game, and are designed to act with their own AI.

''Kingdom Come: Deliverance'' also features a needs system which requires the player to sleep and eat in order to stay healthy. Equipment and clothing also degrade and require repair. The game uses skill/stat-based mini-games for weapon and armor repair, as well as for distilling alcohol, or creating medicines.

[[Quests]] are intended to be [[Nonlinear gameplay|nonlinear]], with multiple ways to complete objectives to allow multiple character types to be viable. Every NPC has a daily routine, and every routine can be affected by the player.
"#;

    let pillars = parse_gameplay_wikitext(raw);
    assert_eq!(pillars.len(), 4);

    assert_eq!(pillars[0].icon, "choices");
    assert_eq!(pillars[0].title, "Choices & Consequences");
    assert_eq!(pillars[0].description, "Reputation is based on player choices and therefore can bring consequences.");

    assert_eq!(pillars[0].image_file, Some("Kingdom Come gameplay screenshot.jpg".to_string()));


    assert_eq!(pillars[1].icon, "combat");
    assert_eq!(pillars[1].title, "Weapons & Combat");
    assert!(pillars[1].description.contains("weapons"));


    assert_eq!(pillars[2].icon, "survival");
    assert_eq!(pillars[2].title, "Needs System");
    assert!(pillars[2].description.contains("needs system"));

    assert_eq!(pillars[3].icon, "choices");
    assert_eq!(pillars[3].title, "Nonlinear Quests");
    assert!(pillars[3].description.contains("Quests are intended to be nonlinear"));
}




#[test]
fn test_parse_elden_ring_gameplay() {
    let raw = r#"== Gameplay ==
''Elden Ring'' is an [[action role-playing game]] presented in a [[third-person perspective]]. Gameplay focuses heavily on combat and exploration; it features elements similar to those found in other games developed by [[FromSoftware]], such as the ''[[Souls]]'' series, ''[[Bloodborne]]'', and ''[[Sekiro: Shadows Die Twice]]''. Throughout the game, players explore an [[open world]] called the Lands Between with a mountable steed named Torrent, navigating six main areas, including catacombs, castles, and fortresses.

Combat features a wide range of weapons, magical spells, and horseback battle mechanics. Players can stealthily navigate through enemy camps and perform critical backstabs from the shadows. Fast travel allows instant relocation between visited Sites of Grace, where players rest, level up attributes using Runes, and memorize sorceries.
"#;

    let pillars = parse_gameplay_wikitext(raw);
    assert!(!pillars.is_empty());
    assert_eq!(pillars[0].icon, "explore");
    assert!(pillars[0].description.contains("Players explore an open world") || pillars[0].description.contains("open world"));
}

#[test]
fn test_parse_big_walk_gameplay() {
    let raw = r#"==Gameplay==

[[Image:Big Walk Gameplay.jpg|thumb|left|Completion of puzzles requires players to use different methods of communication.]]

''Big Walk'' is a [[multiplayer]] co-operative [[adventure video game]] in which a group of players walk through an open-world environment.<ref name="GSpot"/> Players form a group of two to twelve in an online [[Matchmaking (video games)|lobby]], with the size of the group adjusting the content.<ref name="GI"/><ref name="GSpot"/> [[Cross-platform play]] is also supported.<ref name="NLife"/> The lobby host maintains save data, allowing other participants to join or leave.<ref name="GSpot"/> Each player is represented by a [[humanoid]] avatar with a segmented head, torso and legs.<ref name="GSpot"/>

The objective of ''Big Walk'' is to complete [[puzzle video game|puzzles]].<ref name="GSpot"/> Many challenges involve teamwork and using novel methods to communicate with one another,<ref name=GM>{{Cite web |url=test |title=test}}</ref> including miming, singing or transmitting codes.<ref name=PCG>{{Cite web |url=test |title=test}}</ref> Completion of a number of puzzles unlocks features that allow players to further navigate the environment, including a map room to plot travel or a train.<ref name="GSpot"/>

''Big Walk'' has a [[proximity chat]] system.<ref name=IGN/> The volume of players' voices changes depending on their location, becoming inaudible when a short distance away.<ref name="NLife"/> A [[Timekeeping in games|day-night cycle]] and darkness at night requires players to use torches and other light sources to see each other.<ref name="NLife"/> When players are separated, they can communicate through leaving messages, using [[walkie talkie]]s, or launching [[firework]]s to signal their location.<ref name="EuroG"/><ref name="NLife"/>
"#;

    let pillars = parse_gameplay_wikitext(raw);
    assert_eq!(pillars.len(), 3);

    assert_eq!(pillars[0].title, "Multiplayer Lobbies");
    assert!(pillars[0].description.contains("lobby"));

    assert_eq!(pillars[1].title, "Puzzles & Navigation");
    assert!(pillars[1].description.contains("puzzles"));

    assert_eq!(pillars[2].title, "Proximity Chat");
    assert!(pillars[2].description.contains("voices") || pillars[2].description.contains("inaudible"));
}

#[test]
fn test_parse_dispatch_gameplay() {
    let raw = r#"==Gameplay==
[[File:Dispatch dialogue tree example.jpg|thumb|[[Dialogue tree]]s are used for in-game conversations.]]

''Dispatch'' is an [[adventure game]], where the player's choices affect the story via the use of [[dialogue tree]]s in conversations with other characters. A large form of the gameplay consists of navigating a superhero team across the Superhero Dispatch Network (SDN) map to crimes and events, where the player must strategically decide which hero or heroes best fit the activity based on their stats and character traits, while also managing their [[Glossary of video game terms#cooldown|cooldowns]]. In the hacking [[minigame]], the player must navigate pathways and complete [[quick time event]]s.<ref name=Randall>{{Cite web |title=Dispatch |url=https://test.com}}</ref>
"#;

    let pillars = parse_gameplay_wikitext(raw);
    assert_eq!(pillars.len(), 3);

    assert_eq!(pillars[0].icon, "choices");
    assert_eq!(pillars[0].title, "Player's Choice");
    assert_eq!(pillars[0].image_file, Some("Dispatch dialogue tree example.jpg".to_string()));
    assert!(pillars[0].description.contains("choices affect the story") || pillars[0].description.contains("dialogue tree"));
    assert!(!pillars[0].description.contains("used for in-game conversations"));

    assert_eq!(pillars[1].icon, "coop");
    assert_eq!(pillars[1].title, "Superhero Dispatch");
    assert!(pillars[1].description.contains("superhero team") || pillars[1].description.contains("Superhero Dispatch Network"));

    assert_eq!(pillars[2].icon, "puzzle");
    assert_eq!(pillars[2].title, "Hacking Minigame");
    assert!(pillars[2].description.contains("hacking minigame") || pillars[2].description.contains("quick time event"));
}

#[test]
fn test_parse_stardew_valley_gameplay() {
    let raw = r#"==Gameplay==
[[File:Stardew valley screenshot.png|thumb|left|alt=Gameplay screenshot|''Stardew Valley'' puts players in charge of managing a farm.]]
''Stardew Valley'' is a [[Farm life sim|farm life simulation video game]] set in top-down perspective.<ref>{{Cite web |title=Review}}</ref> The [[Nonlinear gameplay|open-ended]] game starts with the player character leaving their corporate job to manage a farm just outside of Pelican Town, located in the eponymous Stardew Valley.<ref>{{Cite web |title=Review 2018}}</ref> The farm inherited from their deceased grandfather must be restored, and players must assist in the town's revival.<ref name="IGN" />

Players may choose from several different farm types, each with a unique theme and different benefits and drawbacks. Each theme helps players focus on upgrading different types of [[Video game slang|skills]] faster.<ref>{{Cite web |title=Farm types}}</ref> On the farm, players need to cut down trees, break rocks, and use a [[scythe]] to clear weeds to make space for tilling and planting.<ref name="Eurogamer" /> The farming system allows players to plant [[Season|seasonal seeds]] that require daily watering and generally must be harvested before the next season. Players can also build barns and coops to raise animals for products like eggs and milk.<ref name="RPGamer" /> As the game progresses through four 28-day seasons, players can complete certain tasks known as bundles. Players must collect a set amount of goods to complete each bundle and bring them to the Community Center to unlock new areas and activities such as a new island.<ref name="IGN" />

Players may develop skills in farming, [[foraging]], fishing, mining, and combat.<ref name="IGN" /> Interacting with townspeople and giving gifts builds relationships over time.<ref name="IGN" /> Players can marry one of 12 bachelors or bachelorettes regardless of gender, allowing the spouse to help with daily farm chores such as cooking, feeding animals, or watering crops.<ref>{{Cite web |title=Spouses}}</ref> After marriage, the couple may choose to have children. Time in the game is divided into day-length segments. Each morning, players begin with a full energy bar, which decreases as tasks are performed and can be replenished by eating food. A nearby cave system holds ores that can be mined and smelted by using a furnace. The cave also contains monsters, adding a combat element to mining. Players must switch between a pickaxe and a sword while exploring deeper levels, where more valuable treasures can be found. A [[Multiplayer video game|multiplayer]] mode was introduced in a later update, allowing online play with other players.
"#;

    let pillars = parse_gameplay_wikitext(raw);
    assert_eq!(pillars.len(), 4);

    let titles: Vec<&str> = pillars.iter().map(|p| p.title.as_str()).collect();
    assert!(titles.contains(&"Farm Types"));
    assert!(titles.contains(&"Farming System"));
    assert!(titles.contains(&"Raise Animals"));
    assert!(titles.contains(&"Community Center Bundles"));

    // Ensure story intro was filtered out
    assert!(!titles.contains(&"Open-ended"));
    assert!(!titles.contains(&"Open-Ended"));

    let desc = pillars.iter().map(|p| p.description.as_str()).collect::<Vec<_>>().join(" ");
    assert!(desc.contains("farm types"));
    assert!(desc.contains("farming system") || desc.contains("seasonal seeds"));
    assert!(desc.contains("raise animals") || desc.contains("barns and coops"));
    assert!(desc.contains("bundle") || desc.contains("Community Center"));
}






#[test]
fn test_wikipedia_fetcher_traits() {
    use crate::metadata::traits::MetadataFetcher;
    use crate::media::types::MediaType;
    use crate::sources::wikipedia::WikipediaSource;

    let source = WikipediaSource::new();
    assert_eq!(source.id(), crate::metadata::models::MetadataProviderId::Wikipedia);
    assert_eq!(source.name(), "Wikipedia");
    assert!(source.supports_media_type(MediaType::Game));
    assert!(!source.supports_media_type(MediaType::Movie));
}

#[tokio::test]
async fn test_wikipedia_client_rate_limiter() {
    use crate::http::RateLimiter;
    use crate::sources::wikipedia::client::WikipediaClient;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, query_param},
    };

    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(query_param("action", "parse"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "parse": {
                    "title": "Gears of War",
                    "sections": [
                        {
                            "toclevel": 1,
                            "level": "2",
                            "line": "Gameplay",
                            "number": "1",
                            "index": "1"
                        }
                    ]
                }
            })),
        )
        .mount(&server)
        .await;

    let client = WikipediaClient::with_rate_limiter(RateLimiter::new(4, 2.0))
        .with_base_url(server.uri());

    let section = client.find_gameplay_section("Gears of War").await.unwrap();
    assert_eq!(section, Some("1".to_string()));
}

#[test]
fn test_deserialize_wikipedia_imageinfo_response() {
    use crate::sources::wikipedia::models::WikipediaImageInfoResponse;

    let json = r#"{
        "query": {
            "pages": {
                "12345": {
                    "imageinfo": [
                        {
                            "url": "https://upload.wikimedia.org/wikipedia/commons/test.jpg"
                        }
                    ]
                }
            }
        }
    }"#;

    let res: WikipediaImageInfoResponse = serde_json::from_str(json).unwrap();
    let pages = res.query.unwrap().pages.unwrap();
    let page = pages.get("12345").unwrap();
    let url = page.imageinfo.as_ref().unwrap()[0].url.as_ref().unwrap();
    assert_eq!(url, "https://upload.wikimedia.org/wikipedia/commons/test.jpg");
}

#[tokio::test]
async fn test_wikipedia_client_fetch_image_url() {
    use crate::sources::wikipedia::client::WikipediaClient;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, query_param},
    };

    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(query_param("action", "query"))
        .and(query_param("titles", "File:Kingdom Come gameplay screenshot.jpg"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "query": {
                "pages": {
                    "100": {
                        "imageinfo": [
                            { "url": "https://upload.wikimedia.org/test.png" }
                        ]
                    }
                }
            }
        })))
        .mount(&server)
        .await;

    let client = WikipediaClient::new().with_base_url(server.uri());
    let url = client
        .fetch_image_url("Kingdom Come gameplay screenshot.jpg")
        .await
        .unwrap();

    assert_eq!(url, Some("https://upload.wikimedia.org/test.png".into()));
}

#[test]
fn test_parse_star_wars_zero_company_gameplay() {
    let raw = r#"==Gameplay==
[[File:Zero Company gameplay screenshot.jpg|thumb|Combat screenshot]]
''Star Wars: Zero Company'' is a [[tactical role-playing game|turn-based tactical]] video game where players command a squad of four mercenaries during the [[Clone Wars]]. Combat is turn-based, utilizing an action point system for movement, attacks, and special abilities. Squad members can sustain lasting injuries or even suffer permanent death, though the narrative continues regardless of who is lost. Players can customize each operative's class, skills, and gear, as well as deploy astromech droids for battlefield support. Outside of combat, players can explore the world from a third-person perspective, converse with NPCs, and explore procedurally generated dungeons.

Between missions, players manage a base of operations known as the Den to conduct research, upgrade their gear, and gather intel on enemy movements. The game introduces a "cycles" system representing the passage of time, where players choose which operations to prioritize before enemy factions advance their own plans. In these missions, players must make narrative decisions that may lead to consequences several cycles later. Squad members also develop bonds and tactical synergies based on shared combat experiences. Features a progression system where players can choose which permanent upgrades to deny their enemies through sabotage.
"#;

    let pillars = parse_gameplay_wikitext(raw);
    assert_eq!(pillars.len(), 4);

    let titles: Vec<&str> = pillars.iter().map(|p| p.title.as_str()).collect();
    assert!(titles.contains(&"Turn-Based Tactical Combat"));
    assert!(titles.contains(&"Permanent Death"));
    assert!(titles.contains(&"Base Management & Operations"));
    assert!(titles.contains(&"Progression & Enemy Sabotage"));

    // Ensure image is associated with the first pillar
    assert_eq!(pillars[0].image_file, Some("Zero Company gameplay screenshot.jpg".to_string()));

    // Ensure bogus titles like "Nonlinear Quests" are not present
    assert!(!titles.contains(&"Nonlinear Quests"));
}


