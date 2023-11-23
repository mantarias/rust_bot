use plotters::prelude::*;
use rand::Rng;
use serenity::model::id::ChannelId;

use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};
use std::collections::HashMap;
use std::fs::File;

use std::io::Read;
use std::path::Path; // Import Rng trait

fn create_and_save_graph(
    message_counts: &Vec<(String, i32)>,
    filename: &str,
    channel_name: &str, // New parameter for the channel name
) -> Result<(), Box<dyn std::error::Error>> {
    let root_area = BitMapBackend::new(&filename, (950, 700)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();
    let title_style = TextStyle::from(("sans-serif", 30).into_font()).color(&(BLACK));
    root_area
        .titled(&format!("Message Statistics for #{}", channel_name), title_style) // Integrating the channel name into the title
        .unwrap();

    let dims = root_area.dim_in_pixel();
    let center = (dims.0 as i32 / 2, dims.1 as i32 / 2);
    let radius = 300.0;

    // Convert HashMap into vectors for labels and sizes
    let (labels, sizes): (Vec<_>, Vec<_>) = message_counts
        .iter()
        .map(|(name, count)| {
            (
                format!("{}:{}", name.as_str(), count.to_string()),
                *count as f64,
            )
        })
        .unzip();

    // Generate a random color for each label
    let mut rng = rand::thread_rng();
    let colors: Vec<_> = labels
        .iter()
        .map(|_| {
            RGBColor(
                rng.gen_range(0..255),
                rng.gen_range(0..255),
                rng.gen_range(0..255),
            )
        })
        .collect();

    let mut pie = Pie::new(&center, &radius, &sizes, &colors, &labels);
    pie.start_angle(66.0);

    pie.percentages((("sans-serif", radius * 0.08).into_font()).color(&BLACK));
    root_area.draw(&pie)?;
    let label_area = root_area
        .margin(10, 10, 10, 10)
        .shrink((0, 0), (150, dims.1 as i32));
    for (i, label) in labels.iter().enumerate() {
        label_area.draw_text(
            label,
            &TextStyle::from(("sans-serif", 35).into_font()).color(&colors[i]),
            (0, i as i32 * 33),
        )?;
    }

    Ok(())
}

use regex::Regex;

#[command]
async fn stats(ctx: &Context, msg: &Message) -> CommandResult {
    let _response = msg.channel_id.say(&ctx.http, "Working on it!...").await?;
    let args: Vec<&str> = msg.content.split_whitespace().collect();

    // Check the arguments' length
    if args.len() <= 1 {
        // Create a friendly and easy-to-understand message.
        let friendly_msg = "Oh, it seems like you forgot to provide enough info.\n\nHere's how to use this command:\n\
                        1. Use `-stats` followed by a number (for example: `-stats 100`).\n\
                        2. You can also include a channel name (for example: `-stats #general 100`).\n\
                        3. The order doesn't matter. You can put the number first (for example: `-stats 100 #general`).";

        // Prompt the user to enter an argument
        msg.channel_id.say(&ctx.http, &friendly_msg).await?;
        return Ok(());
    }
    let mut response = msg.channel_id.say(&ctx.http, "Working on it!...").await?;
    let mut requested_count = 100;
    let mut channel_id = msg.channel_id;
    if args.len() > 1 {
        for arg in &args[1..] {
            if let Ok(count) = arg.parse::<u64>() {
                requested_count = count;
            } else {
                let re = Regex::new(r"<#(\d+)>").unwrap();

                if let Some(captures) = re.captures(arg) {
                    if let Some(id_match) = captures.get(1) {
                        if let Ok(id) = id_match.as_str().parse::<u64>() {
                            channel_id = ChannelId(id);
                        }
                    }
                } else if let Ok(id) = arg.parse::<u64>() {
                    channel_id = ChannelId(id);
                } else {
                    msg.reply(ctx, "Invalid argument format, using current channel and default message count.")
                        .await?;
                }
            }
        }
    }

    let mut last_message_id = None;
    let mut all_messages = Vec::new();
    let mut total_fetched = 0;

    while total_fetched < requested_count {
        let fetch_count = std::cmp::min(requested_count - total_fetched, 100);
        let messages = channel_id
            .messages(&ctx.http, |retriever| {
                retriever.limit(fetch_count);
                if let Some(message_id) = last_message_id {
                    retriever.before(message_id);
                }
                retriever
            })
            .await?;
        response
            .edit(&ctx.http, |m| {
                m.content(format!(
                    "Working on it! step {} of {}",
                    total_fetched, requested_count
                ))
            })
            .await?;
        if messages.is_empty() {

            break;
        }

        last_message_id = messages.last().map(|message| message.id);
        total_fetched += messages.len() as u64;
        all_messages.extend(messages);
    }
    response
    .edit(&ctx.http, |m| {
        m.content(format!("Done collecting messages starting making pie"))
    })
    .await?;
    let mut message_counts: HashMap<String, i32> = HashMap::new();
    for message in all_messages {
        *message_counts.entry(message.author.name).or_insert(0) += 1;
    }

    // sort the hashmap by value
    let mut message_counts: Vec<_> = message_counts.into_iter().collect();
    message_counts.sort_by(|a, b| b.1.cmp(&a.1));

    // Limit the number of slices to 19 and combine the rest into one slice as the 20th slice
    if message_counts.len() > 19 {
        let mut other_count = 0;
        for i in 19..message_counts.len() {
            other_count += message_counts[i].1;
        }
        message_counts.truncate(19); // Keep the first 19 elements
        message_counts.push(("Other".to_string(), other_count));
    }

    // Create the graph synchronously
    // create_and_save_graph(&message_counts, "output.png").expect("Failed to create
    let channel_name = channel_id.to_channel(&ctx.http).await?.guild().unwrap().name;
    let _ = create_and_save_graph(&message_counts, "output.png", &channel_name);

    // Send the image to the Discord channel
    let path = Path::new("output.png");
    let mut file = File::open(&path).expect("Unable to open the file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Unable to read the file");

    msg.channel_id
        .send_files(&ctx.http, vec![(&buffer as &[u8], "output.png")], |m| {
            m.content(format!(
                "Here is the statistics graph for {}",
                channel_id.mention()
            ))
        })
        .await
        .expect("Unable to send message");

    Ok(())
}