use std::collections::HashMap;

use octocrab::{models, params::{self, Direction, pulls}};
use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(name = "auto-release")]
struct Opt {
    #[structopt(short, long, default_value = "75")]
    last_pull_request: u64,

    #[structopt(short, long)]
    milestone: String,
}


#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let opt: Opt = Opt::from_args();

    let octocrab = octocrab::instance();
    let page = octocrab
        .pulls("himkt", "konoha")
        .list()
        .state(params::State::Closed)
        .direction(Direction::Descending)
        .sort(pulls::Sort::Created)
        .page(1 as u8)
        .per_page(50 as u8)
        .send()
        .await?;

    let mut items_map = HashMap::<String, Vec<String>>::new();
    items_map.contains_key(&opt.milestone);

    let mut page = Some(page);
    while let Some(current_page) = page {
        for pull in current_page.items {
            println!("{} (#{})", pull.title, pull.number);

            match pull.labels {
                Some(labels) => {
                    if labels.len() > 1 {
                        println!("!!one more labels are assigned!!");
                    }

                    for label in labels {
                        println!("{}", label.name);
                    }
                },
                _ => println!("!!no label found!!"),
            }

            match pull.milestone {
                Some(milestone) => {
                    println!("{}", milestone.title);
                },
                _ => println!("!!no label found!!"),
            }

            // end of pull requests
            if pull.number == opt.last_pull_request {
                return Ok(());
            }
        }

        // next cursor
        page = octocrab.get_page::<models::pulls::PullRequest>(&current_page.next).await?;
    }

    Ok(())
}
