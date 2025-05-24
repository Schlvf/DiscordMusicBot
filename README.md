# Discord Music Bot

This is a basic Discord music bot implementation that you can run locally on your own computer or server

<details>
<summary><b>Click here if you care to know more about this project</b></summary>

Given that all the popular Discord music bots kept breaking or getting banned I wanted to make a basic one that I could self host for my own private server

This project is built entirely in Rust and it started as a learning exercise given that I wanted to get familiar with the language, therefore, the code may contain bugs, and there’s plenty of room for improvement and optimization. Any feedback or suggestions are welcome, especially if you spot areas where things could be done more idiomatically or efficiently in Rust
</details>

## How to setup the tool

[**Please click here to visit the installation guide**](docs/installation_guide.md)

## How does it work

Once you've completed all the steps in the installation guide and you invited your bot to your server, all you need to do is run the executable to start the music bot and the slash commands will be available in the server

When you request something with the `/play`command the bot will use `yt-dlp` and `ffmpeg` to get the audio track and turn it into something playable in a Discord voice channel

Here's a brief description of all the commands available:

|Command|Description|
|-|-|
|`/ping`|The bot will reply `pong` in the text channel. It is useful to check if the bot is working|
|`/join`|The bot will join your current voice channel if you're in one|
|`/leave`|The bot will leave the voice channel if he's in any and will clear the queue|
|`/play <url>`|The bot will attempt to get the audio track from the url and add it to the queue. Keep in mind that the bot must the in a voice channel for this command to work|
|`/pause`|The bot will pause the current track if there's any in the queue|
|`/resume`|The bot will resume playing the current track if there's any in the queue|
|`/skip`|The bot will skip the current track if there's any in the queue|
|`/stop`|The bot will stop whatever is playing and clear the queue|

## Things to consider

1. This tool won't add an already existing bot to your server, you must create your own Discord application in the Discord developer portal so this tool can bring it to life as a music bot. The process is fairly simple and it is covered in the installation guide

2. Given that the project was built using Songbird, it requires you to setup `yt-dlp` and `ffmpeg` as a third party dependency in order to be able to play audio

3. Since the tool uses `yt-dlp` you could in theory obtain audio tracks from any of [their supported websites](https://github.com/yt-dlp/yt-dlp/blob/master/supportedsites.md) by giving the bot the corresponding url, however, it was only made having Youtube in mind, so feel free to explore on your own

4. If you don't feel safe running the pre-compiled executable, the entire source code is public so you can review it and build your own executable of the tool at any time

5. **You can host the bot for multiple Discord servers at the same time but this doesn't mean that you should**. Please note that if your bot receives a large amount of request in a short span of time Youtube will rate-limit and will prevent it from working as intended for a certain period of time

## For developers

As the creator of the project I have no plans of implementing new features given that the main goal was already achieved but I will do my best to maintain it functional. Nonetheless, feel free to fork, modify, and build upon it as long as it is done under the same license (GPL-3.0) and with proper attribution
