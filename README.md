Initally created to display cute messages from my wife along my other dwmblocks modules. :D The way it's integrated with my dwm stuff can be found [here](https://github.com/euocheisthai/archfiles/blob/0dab246229fad8ab3c03e15304c0c386d8819571/dwmblocks-async-main/config.h#L21).

klandestin reads from `~/.klandestinrc`, where a singular telegram user id is specified, and monitors texts sent from that user to the telegram bot that is running locally with its `TELOXIDE_TOKEN`. klandestin logs every text to `~/.klandestin_log `and abridged messages to `~/.klandestin_current`, limited to 50 characters.

How to use:

1. create a telegram bot for receiving messages from the target user; `@BotFather` is the way to go.
2. export the bot token as `TELOXIDE_TOKEN`
3. save the target user id in `~/.klandestinrc` and launch the compiled binary. `~/.klandestinrc` example:
```bash
123567890 # obtained from @UserInfoToBot
```
4. have your target user text the bot by using the status command: `/status here goes my first update...`
5. check that both log files got updated, and use in whatever way you may find productive.
