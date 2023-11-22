use serenity::{
    framework::standard::{
        macros::command,
        CommandResult,
    },
    model::channel::Message,
    prelude::*,
};
use std::collections::HashMap;
use plotters::prelude::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;

// Define a synchronous function to create and save the graph
fn create_and_save_graph(message_counts: &HashMap<String, i32>, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut best_users: Vec<_> = message_counts.iter().collect();
    best_users.sort_by(|a, b| b.1.cmp(&a.1));

    let max_count = *best_users.iter().map(|(_, count)| *count).max().unwrap_or(&0);
    let count_length = best_users.len();

    let root = BitMapBackend::new(filename, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("User Message Count", ("sans-serif", 50))
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0..count_length, 0..max_count)?;

    chart.configure_mesh()
        .x_labels(count_length)
        .x_label_formatter(&|x| {
            if *x < best_users.len() {
                best_users[*x as usize].0.clone()
            } else {
                String::new() // Return an empty string if out of bounds
            }
        })
        .x_desc("Username")
        .y_desc("Message Count")
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLUE.filled())
            .data(best_users.iter().enumerate().map(|(idx, (_, count))| (idx, **count))),
    )?;

    root.present()?;
    Ok(())
}

#[command]
async fn stats(ctx: &Context, msg: &Message) -> CommandResult {
    let channel_id = msg.channel_id;
    channel_id.broadcast_typing(&ctx.http).await?;

    let requested_count: u64 = match msg.content.replacen("-stats ", "", 1).parse() {
        Ok(num) => num,
        Err(_) => {
            msg.reply(ctx, "Please enter a valid number").await?;
            return Ok(());
        }
    };

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

        if messages.is_empty() {
            break;
        }

        last_message_id = messages.last().map(|message| message.id);
        total_fetched += messages.len() as u64;
        all_messages.extend(messages);
    }

    let mut message_counts: HashMap<String, i32> = HashMap::new();
    for message in all_messages {
        *message_counts.entry(message.author.name).or_insert(0) += 1;
    }

    // Create the graph synchronously
    create_and_save_graph(&message_counts, "output.png").expect("Failed to create graph");

    // Send the image to the Discord channel
    let path = Path::new("output.png");
    let mut file = File::open(&path).expect("Unable to open the file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Unable to read the file");

    channel_id.send_files(&ctx.http, vec![(&buffer as &[u8], "output.png")], |m| {
        m.content("Here is the statistics graph:")
    }).await.expect("Unable to send message");

    Ok(())
}
