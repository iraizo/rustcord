/*
    usage: threads that keep listening to events like on_message
*/

pub mod listener {
    use serenity::{
        async_trait,
        model::{channel::Message, gateway::Ready},
        prelude::*,
    };

    struct Handler;

    #[async_trait]
    impl EventHandler for Handler {
        async fn message(&self, ctx: Context, msg: Message) {}
        /* add more handlers here */ 

        /* https://docs.rs/serenity/0.9.1/serenity/client/trait.EventHandler.html#method.message */
    }
}
