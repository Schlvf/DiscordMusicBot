# Detailed installation guide

## Create a Discord app

If you already have a Discord app created you can skip to the next section, otherwise follow these steps:

1. Go to the [Discord developer site](https://discord.com/developers/applications) and log into your account
2. Click on `New application`, give it any name, accept the terms of service and lastly click on `Create`
3. On the left side click on the `Bot` tab
4. Under the Token section click on `Reset Token` and confirm the prompt you'll see. You may even need to complete a 2-Factor authentication process if you have 2FA active on your Discord account
5. Now copy and save the token that you just got as we'll need it later. This is the token we will use to log in as the bot
6. This step is optional but I'd recommend to uncheck the `Public bot` option so only you can add it to Discord servers. This can be found under `Authorization flow`
7. Under `Privileged Gateway Intents` check Message content intent

Additionally you can change the bot's name or photo, just fiddle around and move to the next section once you're done

## Invite the bot to your Discord server

Assuming you're still on the Discord developer site, just follow these steps:

1. Select your application in case you haven't done so already and on the left side click on the `Oauth2` tab
2. Under the `OAuth2 URL Generator` select the `bot` checkbox
3. Scroll down to the `Generated url` section and copy the url that you see there
4. Paste it in the browser of your preference, select the server you desire to add the bot to, and finally click on `Authorize`
5. Now locate the bot in your server and make sure that it has permissions to read messages, send messages, connect to voice channels and talk in voice channels. You can do this by giving it a Discord role that has those permissions

Please note that you don't need to check any permissions from the url generator page as long as the bot is given the correct role permissions inside the server

## Install ffmpeg and yt-dlp

### If you're on windows

The simplest way to install these 2 tools on windows is using winget since it will do everything for you. To do so, just copy and paste the following commands on a powershell window:

`winget install ffmpeg`

`winget install yt-dlp`

### Alternatively

If you're not on windows or you want to install everything manually you can find the installers in the [official site of ffmpeg](https://www.ffmpeg.org/download.html) and the [official site of yt-dlp](https://github.com/yt-dlp/yt-dlp/wiki/Installation)

The process may vary depending on the operative system you're working with, so make sure to follow the instructions accordingly

## Download the executable

Go to [releases](https://github.com/Schlvf/DiscordBot/releases) and download the corresponding pre-compiled executable accordingly to your operative system

windows users -> `DiscordBot.exe`\
linux users -> `DiscordBot`

Alternatively you can compile your own executable of the tool by cloning the repository and setting up a [Rust](https://www.rust-lang.org/) compilation environment. This process goes beyond the scope of this guide so please refer to the [official Cargo documentation](https://doc.rust-lang.org/stable/cargo/) where you can learn how to install Cargo and the commands needed to compile/build the project for your desired operative system

## Prepare the token

When the tool is executed it needs to locate the Discord app token we saved earlier so it can login as your bot, this is why we must add the token to the environmental variables or to a `.env` file

The simplest way is the file method so navigate to the folder where you left the executable and create a file called `.env` and inside this file you must add the following:

```conf
DISCORD_TOKEN = "the_token_you_saved_earlier"
```

Just make sure to replace the token right as it was given to you inside the quotes

## Final steps

If you've done everything correctly you should now be able to run the executable to start using the bot with the slash commands, however, keep in mind that the tool  has no user interface

When you simply run the executable by itself it opens your default terminal. **In windows it will normally open a powershell window, however, if something fails the execution will be interrupted and the window will close right away**

In order to be able to see what failed with the bot in case something breaks or doesn't work, you will need to execute it from a cmd, bash or powershell window since it will not close once the executable exits and will let you read the logs

If you're on windows and you have no idea how to do this just follow these steps:

1. Open the folder where you put the `DiscordBot.exe` and the `.env` file
2. Hold `shift` and right click in an empty space of the folder
3. You should now be able to see an additional option in the context menu called `Open PowerShell window here` so go ahead and click it. *(If you don't see it repeat step 2)*
4. Once you're in powershell type `.\DiscordBot.exe` or whatever name you gave the executable and press enter and it will run it as a sub-process of the powershell window and will let you read the logs even after the program exits

If you're not on windows I assume you know how to use a terminal, so go ahead and run the executable from there

## Solving other common issues that you may experience

> I don't see the slash commands on my server

The very first time that you run the bot successfully the bot will trigger an event that registers the commands globally on the server and this can take a moment to reflect on Discord

If you don't see any errors in the logs just wait a minute and restart Discord by pressing `Ctrl` + `R` on desktop and the slash commands should be available once your Discord has fully restarted

> When I run the tool I see `Warning: Could not load .env file: path not found`

This simply means that the program is not finding a `.env` file to load environmental variables, however, as long as the `DISCORD_TOKEN` variable is configured in your system the bot will work normally

> When I run the tool I see `missing DISCORD_TOKEN: NotPresent`

This means that the `DISCORD_TOKEN` variable was not correctly configured in the system or the `.env` file. To fix this simply go back **Prepare the token** section and follow the instructions there

> When I run the tool I see `401 Unauthorized error`

If you see in the logs something like:

`Http(UnsuccessfulRequest(ErrorResponse { status_code: 401, url: "https://discord.com/api/v10/users/@me", method: GET, error: DiscordJsonError { code: 0, message: "401: Unauthorized", errors: [] } }))`

This most likely means that your `DISCORD_TOKEN` is no longer valid. The token can become invalid for multiple reasons, but to fix it you can go back to the Discord developer site and reset your application token to generate a new one

> When I request a Youtube video the bot replies `failed to create audio: yt-dlp failed with non-zero status code: WARNING: [youtube] kaEGKHZvU: Signature extraction failed: Some formats may be missing`

The Youtube API is constantly changing and those changes break `yt-dlp`. To fix these errors simply make sure that your `yt-dlp` is updated. To manually update it, open a powershell window and run the following command

`yt-dlp -U`.

You may need to do this from time to time but it will fix the problem 99% of the time, however, if for whatever reason the problem persists just give it some time and wait until a new version of `yt-dlp` is available

> When I request a Youtube video the bot replies `failed to create audio: yt-dlp failed with non-zero status code: ERROR: [youtube] h4dCXkgwM: Sign in to confirm your age. This video may be inappropriate for some users.`

You will see this error message when you request an **Age restricted Youtube video**. To fix this please follow the instructions on [how to pass cookies to yt-dlp](https://github.com/yt-dlp/yt-dlp/wiki/FAQ#how-do-i-pass-cookies-to-yt-dlp) and [how to export cookies from your browser]( https://github.com/yt-dlp/yt-dlp/wiki/Extractors#exporting-youtube-cookies)
