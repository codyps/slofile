
use scraper::{Html, Selector};

#[derive(Debug)]
struct Team {
    name: String,
    slug: String,
    members: u64,
    channels: u64,
    bots: u64,
}

#[tokio::main]
async fn main() {
    let teams = slofile("https://slofile.com/").await;

    for team in teams {
        println!("{:?}", team);
    }
}

async fn slofile(url: &str) -> Vec<Team> {
    let resp = reqwest::get(url).await.unwrap();
    assert!(resp.status().is_success());
    let body = resp.text().await.unwrap();

    let fragment = Html::parse_document(&body);

    // div class="team"
    // ... div class="name"
    // ... div class="count" (3 times)
    let teams = Selector::parse("div.team").unwrap();
    let name = Selector::parse("div.name>a").unwrap();
    let counts = Selector::parse("div.count").unwrap();

    let mut team_res = Vec::new();

    for team in fragment.select(&teams) {
        let name = team.select(&name).next().unwrap();
        let name_v = name.value();
        let slug = name_v.attr("href").unwrap();
        let name = name.inner_html();

        let mut counts = team.select(&counts);
        let members: u64 = counts.next().unwrap().text().next().unwrap().parse().unwrap();
        let channels: u64 = counts.next().unwrap().text().next().unwrap().parse().unwrap();
        let bots: u64 = counts.next().unwrap().text().next().unwrap().parse().unwrap();

        team_res.push(Team {
            name,
            slug: slug.to_owned(),
            members,
            channels,
            bots,
        })
    }

    team_res
}
