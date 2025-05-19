klandestin reads from ~/.klandestinrc where a singular telegram user id is specified and monitors texts from the said user to a telegram bot that is configured via setting TELOXIDE_TOKEN env var. klandestin logs every text to ~/klandestin_log and abridged messages to ~/.klandestin_current, limited to 50 characters.

Initally created to display cute messages from my wife along my other dwmblocks modules. :D

How to use:

1. create a telegram bot for receiving messages from the target user.
2. export the bot token as TELOXIDE_TOKEN
3. save the target user id in ~/.klandestinrc and launch the compiled binary
4. have your target user text the bot by using the status command: "/status here goes my first update..."
5. check that both log files got updated, and use in whatever way you may find productive
