use octocrab::{models, params::{self, Direction, pulls}};


#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let octocrab = octocrab::instance();
    let page = octocrab
        .pulls("himkt", "konoha")
        .list()
        .state(params::State::Closed)
        .direction(Direction::Descending)
        .sort(pulls::Sort::Created)
        .page(1 as u8)
        .per_page(5 as u8)
        .send()
        .await?;

    let mut page = Some(page);
    while let Some(current_page) = page {
        for item in current_page.items {
            for label in item.labels.unwrap() {
                println!("{}", label.name);
            }
            println!("{}", item.number);
            println!("{}", item.title);

            match item.milestone {
                Some(val) => println!("{:?}", val.title),
                _ => println!("!!no label found!!"),
            }
        }

        // next cursor
        page = octocrab.get_page::<models::pulls::PullRequest>(&current_page.next).await?;
    }

    Ok(())
}
