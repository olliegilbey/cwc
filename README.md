# Coffee with CUDOS Text Generator CLI Tool

Generates event messages for social media for CUDOS online events.
It uses the system clock to determine the dates and times of events and creates the necessary posts for those events.

USAGE:
    cwc [OPTIONS] --location <LOCATION>

OPTIONS:
    -h, --help                   Print help information
    -l, --location <LOCATION>    Sets the location for the event (Twitter or Discord)
    -t, --topics <TOPICS>        Sets the topics for the event
    -u, --url <URL>              Sets the URL location for the event
    -V, --version                Print version information

EXAMPLE:
```
cwc -l d -t "Cats" -u "discord.gg/cudos"
```

OUTPUT:
```
------EVENT MESSAGE-------

Join us for ☕Coffee with CUDOS #144 here on Discord in the `coffee-with-cudos` voice channel.

Bring your favourite drink and come have a chat during our office-hours!
Feel free to post questions into the channel chat or unmute your microphone and ask them in the voice channel.
Please note, nothing covered in these sessions constitutes financial advice. 🚀

------ANNOUNCEMENT--------

Hey @In-The-Know!

☕`coffee-with-cudos` #144 office hours will start <t:1684839600:R> at <t:1684839600:t>!
Swing by with a hot drink and join us for a chat on Discord.

We will be chatting about Developers, Cats, Validators, and anything else that comes up! You can find the event here:

discord.gg/cudos

Keen to see you there!! 🚀

------TWEET---------------

Join us for ☕️ Coffee with #CUDOS #144 on Discord in the `coffee-with-cudos` voice channel.
We will be chatting about Developers, Cats, and Validators.
Swing by with a hot drink! ☕️
discord.gg/cudos

------CAFFEINE------------

Hey @Developers!

CUDOS Caffeine #3 is happening <t:1684846800:R> at <t:1684846800:t> in the #vibe-while-you-code channel with a member or two from the CUDOS team and available @Developer-Rangers!

Bring your technical questions and we can help you get set up!
```
