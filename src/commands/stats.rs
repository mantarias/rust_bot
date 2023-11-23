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
use plotters::prelude::full_palette::ORANGE;
use rand::Rng; // Import Rng trait


use plotters::coord::Shift;

fn create_and_save_graph(message_counts: &HashMap<String, i32>, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root_area = BitMapBackend::new(&filename, (950, 700)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();
    let title_style = TextStyle::from(("sans-serif", 30).into_font()).color(&(BLACK));
    root_area.titled("Message Statistics", title_style).unwrap();

    let dims = root_area.dim_in_pixel();
    let center = (dims.0 as i32 / 2, dims.1 as i32 / 2);
    let radius = 300.0;

    // Convert HashMap into vectors for labels and sizes
    let (labels, sizes): (Vec<_>, Vec<_>) = message_counts.iter().map(|(name, count)| (format!("{}:{}",name.as_str(), count.to_string()), *count as f64)).unzip();

    // Generate a random color for each label
    let mut rng = rand::thread_rng();
    let colors: Vec<_> = labels.iter().map(|_| {
        RGBColor(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255))
    }).collect();

    let mut pie = Pie::new(&center, &radius, &sizes, &colors, &labels);
    pie.start_angle(66.0);

    pie.percentages((("sans-serif", radius * 0.08).into_font()).color(&BLACK));
    root_area.draw(&pie)?;

    // Draw labels to the rightvscode-file://vscode-app/c:/Users/manta/AppData/Local/Programs/Microsoft%20VS%20Code/resources/app/out/vs/code/electron-sandbox/workbench/workbench.html
    let label_area = root_area.margin(10, 10, 10, 10).shrink((0, 0), (150, dims.1 as i32));
        for (i, label) in labels.iter().enumerate() {
        label_area.draw_text(
            label,
            &TextStyle::from(("sans-serif", 35).into_font()).color(&colors[i]),
            (0, i as i32 * 35),
        )?;
    }

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
    // create_and_save_graph(&message_counts, "output.png").expect("Failed to create graph");
    create_and_save_graph(&message_counts, "output.png");

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
