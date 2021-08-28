# bonkbot

A small silly bot to "bonk" people in discord.

![Screenshot](files/Screenshot.png)

## Usage

This bot pulls from a list of custom emoji defined at the top of `main.rs`. Change these to refer to emoji that your instance of the bot will know about (i.e. ones from the server you intend to use it on)

- [Create](https://discordpy.readthedocs.io/en/latest/discord.html#creating-a-bot-account) a discord application and bot.
- [Invite](https://discordpy.readthedocs.io/en/latest/discord.html#inviting-your-bot) the bot to your server.
- Run the bot with `cargo run`. To provide the token, you have 3 options:
  - Provide the token directly with `--token <token>`
  - Provide the name of a file containing the token with `--token-filename <filename>`
  - Set the environment variable `DISCORD_TOKEN` to the token before running.
- Send the message `!bonk <content>`
- Enjoy the silly fun

---

Based on [Serenity](https://github.com/serenity-rs/serenity).

Available under the terms of the Mozilla Public Licence, version 2.0
