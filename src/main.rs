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

    #[structopt(short, long)]
    organization: String,

    #[structopt(short, long)]
    repository: String,
}


async fn create_pull_request_list() -> octocrab::Result<HashMap<String, Vec<String>>, octocrab::Error> {
    let opt: Opt = Opt::from_args();
    let octocrab = octocrab::instance();
    let page = octocrab
        .pulls(opt.organization, opt.repository)
        .list()
        .state(params::State::Closed)
        .direction(Direction::Descending)
        .sort(pulls::Sort::Created)
        .page(1 as u8)
        .per_page(100 as u8)
        .send()
        .await?;

    let mut items_map = HashMap::<String, Vec<String>>::new();
    items_map.contains_key(&opt.milestone);

    let mut page = Some(page);
    while let Some(current_page) = page {
        for pull in current_page.items {
            match pull.milestone {
                Some(milestone) => {
                    if milestone.title != opt.milestone {
                        continue
                    }
                },
                _ => {
                    println!("!!no milestone found for #{}!!", pull.number)
                },
            }

            println!("medium pr #{}, #{}", pull.number, opt.last_pull_request);
            match pull.labels {
                Some(labels) => {
                    if labels.len() > 1 {
                        println!("!!one more labels are assigned for #{}!!", pull.number);
                    }

                    for label in labels {
                        if items_map.contains_key(&label.name) {
                            items_map.get_mut(&label.name).unwrap().push(format!("{} (#{})", pull.title, pull.number));
                        }
                        else {
                            items_map.insert(label.name, vec![format!("{} (#{})", pull.title, pull.number)]);
                        }
                    }
                },
                _ => {
                    println!("!!no label found for #{}!!", pull.number)
                },
            }

            // end of pull requests
            if pull.number == opt.last_pull_request {
                return Ok(items_map);
            }
        }

        // next cursor
        page = octocrab.get_page::<models::pulls::PullRequest>(&current_page.next).await?;
    }
    Ok(items_map)
}


#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let items_map = create_pull_request_list().await?;

    println!("\n\n\n");
    for (k, v) in items_map.iter() {
        println!("#{}", k);
        for pull in v.iter() {
            println!("- {}", pull);
        }
        println!();
    }
    Ok(())
}
