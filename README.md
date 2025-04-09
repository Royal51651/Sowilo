# Sowilo - ᛊᛟᚹᛁᛚᛟ
Sowilo is an app that generates color palettes based on several different color models. Named after the nordic rune for light (ᛊ)

# How does it work?
Sowilo uses several different elements of the HSL and HSV color models, in addition to some custom algorithms, to qualify and sort colors. By default, it groups colors by their hue, taking slices at a user specified length. Then, it does the same with saturation, slicing each of these seperated hues into chunks. Finally, the data is sorted and ranked using multiple different properties, such as luminosity, vibrancy, and colorfulness

# How do I get it?
You should be able to download the latest release from the release tab found here:  https://github.com/Royal51651/Sowilo/releases

Tauri is still a fairly new framework, so it doesn't yet have cross-compilation. Because of this, certain architectures and operating systems may be unavailable. To solve this, compile from source.

How to compile from source:

Make sure you have git, and follow the prerequisites from the official Tauri site: https://v2.tauri.app/start/prerequisites/
Once you're done with that, clone the directory.

```git clone https://github.com/Royal51651/Sowilo```

Then cd into the folder

```cd Sowilo```

Install the dependencies

```npm install```

And finally run the project

```npm run tauri build```

From there, you should be presented with instructions to install based on your operating system.
