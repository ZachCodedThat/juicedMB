
          use std::env;
          use std::fs;

          use serenity::{
          	async_trait,
          	model::{channel::Message, gateway::Ready},
          	prelude::*,
          };

          

          const TK_MESSAGE: &str = "
          Damn someone fucked up huh?

          This is the TK list for the <#326194923214995457>

          Current TK Leader: Denzel! with 2 

          Type: 
              - !tkl to see the leaderboard in full 
              
          ";

          

          

          const TK_COMMAND: &str = "!tk";

          const TK_LEADERBOARD_COMMAND: &str = "!tkl";

          struct Handler;

          #[async_trait]
          impl EventHandler for Handler {
            async fn message(&self, ctx: Context, msg: Message) {
             
            if msg.content == TK_COMMAND {
              if let Err(why) = msg.channel_id.say(&ctx.http, TK_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
          }
          else if msg.content == TK_LEADERBOARD_COMMAND {
          

            let contents = fs::read_to_string("TkList.txt")
            .expect("Something went wrong reading the file");

    
            if let Err(why) = msg.channel_id.say(&ctx.http, contents).await {
              println!("Error sending message: {:?}", why);
          }
        }
          }

          async fn ready(&self, _: Context, ready: Ready) {
          	println!("{} is connected!", ready.user.name);
          	}
          }
          #[tokio::main]
          async fn main() {
            let token = env::var("DISCORD_TOKEN")
            .expect("Expected a token in the environment");

            let mut client = Client::builder(&token)
            .event_handler(Handler)
            .await
            .expect("Err creating client");

            if let Err(why) = client.start().await {
            	println!("Client error: {:?}", why);
            }
          }


         



          




            
            