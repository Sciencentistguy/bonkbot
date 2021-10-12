# bonkbot

A small silly bot to "bonk" people in discord.

![Screenshot](files/Screenshot.png)

## Usage

- Add custom bonk emojis to `src/bonk_emojis.txt`. The strings present in that file by default will not work, unless your instance of the bot is in my testing server (it isn't).
- [Create](https://discordpy.readthedocs.io/en/latest/discord.html#creating-a-bot-account) a discord application and bot.
- [Invite](https://discordpy.readthedocs.io/en/latest/discord.html#inviting-your-bot) the bot to your server. Make sure to select the `applications.commands` scope if you want to use slash commands.
- Create two files, containing the bot token and application id
- Run the bot, providing the token and application id as command line arguments:
  - Either with cargo:
    - `cargo run -- <token_filename> <application_id_filename>`
  - Or standalone:
    - `bonkbot <token_filename> <application_id_filename>`
- Use the command `/bonk <user>`, or send a message of the form `!bonk <message>`.

---

Written using [Serenity](https://github.com/serenity-rs/serenity).

Available under the terms of the Mozilla Public Licence, version 2.0
