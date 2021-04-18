# Shining_Force_II_Blacksmith
Simple utility to edit a Shining Force II savestate so I don't have to visit the blacksmith and reset a bunch. This version was made because I decided to buy an Mega Everdrive Pro and wanted an excuse to learn rust.

This guide was used as a reference.
https://gamefaqs.gamespot.com/genesis/563341-shining-force-ii/faqs/11359

# Building and running
To build run the following commands.
```
 cd blacksmith
 cargo build
 ./target/debug/blacksmith -f $INPUT_SAVE_FILE -o $OUTPUT
```

Please make backups of your saves. I am not responsible for corrupt data.
