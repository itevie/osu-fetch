use rosu_v2 as Rosu;
use tokio;

static TITLE_COLOR: &'static str = "\x1b[1;32m";
static RESET: &'static str = "\x1b[0m";

static OSU_LOGO: &'static str = include_str!("./logos/osu.txt");
static OSU_LOGO_WIDTH: usize = 40;

#[tokio::main]
async fn main() {
    let client_id: u64 = 0;
    let client_secret: String = String::from("<token here>");
    let osu: Rosu::Osu = match Rosu::Osu::new(client_id, client_secret).await {
        Ok(client) => client,
        Err(why) => panic!(
            "Failed to create client or make initial osu!api interaction: {}",
            why
        ),
    };

    let user = osu
        .user(14561519)
        .await
        .unwrap_or_else(|why| panic!("Failed to get user: {}", why));
    let statistics = user
        .statistics
        .unwrap_or_else(|| panic!("Failed to get statistics"));

    let parts = vec![
        format!(
            "{}{}{}@{}{}{}",
            TITLE_COLOR, user.username, RESET, TITLE_COLOR, user.user_id, RESET
        ),
        format!(
            "{}",
            "-".repeat(format!("{}@{}", user.username, user.user_id).len())
        ),
        format!("{}Accuracy: {}{}%", TITLE_COLOR, RESET, statistics.accuracy),
        format!("{}Performance: {}{}pp", TITLE_COLOR, RESET, statistics.pp),
        format!(
            "{}Leval: {}{}",
            TITLE_COLOR, RESET, statistics.level.current
        ),
        format!(
            "{}Global Rank: {}{}",
            TITLE_COLOR,
            RESET,
            statistics.global_rank.unwrap_or(0)
        ),
        format!(
            "{}Country Rank: {}{}",
            TITLE_COLOR,
            RESET,
            statistics.country_rank.unwrap_or(0)
        ),
        format!("{}Playtime: {}{}", TITLE_COLOR, RESET, statistics.playtime),
        format!(
            "{}Playcount: {}{}",
            TITLE_COLOR, RESET, statistics.playcount
        ),
        format!(
            "{}Max Combo: {}{}",
            TITLE_COLOR, RESET, statistics.max_combo
        ),
    ];

    let osu_logo_lines = OSU_LOGO.split("\n").collect::<Vec<&str>>();
    let start_at = ((osu_logo_lines.len() - parts.len()) / 2) - 1;
    let mut idx = 0;

    for line in osu_logo_lines {
        print!("{: <40}", line);

        if idx > start_at && parts.len() > idx - start_at - 1 {
            print!("{}", parts[idx - start_at - 1]);
        }
        idx += 1;

        print!("\n");
    }
}
