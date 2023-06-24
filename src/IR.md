# GBC Infrared Communication

{{#include partials/dandocs_notice.md}}

The Game Boy Color came with an infrared port on the very top of the handheld. Previously, where IR communications had to be done with special cartridges (like the HuC-1 variants), the Game Boy itself now had the hardware built-in. Unfortunately, the feature was never popular outside of a few games and accessories. The IR port essentially sends out signals and is also capable of receiving them, allowing for fast, wireless, line-of-sight transmission.

- GBC comes with one IR port. Capable of sending and receiving an IR signal (two separate diodes).
- Turning on the IR light does drain battery, hence not recommended leaving it on when not in use
- IR port can communicate with non-GBC devices, e.g. anything that sends an IR signal (TV remotes, Wiimotes, household lamps, etc)

## Communication Types

While a number of games may use similar formats for their IR communications, there is no "standard" protocol that all games use. IR communication is entirely determined by the game's code, hence it can vary wildly depending on needs. However, all communications fall into one of several general categories as described below:

- 1-Player Init: These only require one GBC to initiate IR transfers. Both GBCs typically wait for an infrared signal. When one player presses a button, the GBC starts sending pulses. This setup is not unlike how 2-Player Serial I/O is handled (with master and slave Game Boys). Examples include Super Mario Bros. DX score exchange and the GBC-to-GBC Mystery Gifts in Pokemon Gold/Silver/Crystal. Most IR compatible games fall into this group.
- 2-Player Init: Transfers require both GBCs to initiate at roughly the same time. Examples include Pokemon Pinball score exchange, Pokemon TCG's "Card Pop", and trading/fighting Charaboms in the Bomberman games.
- Active Object Init: Transfers require the GBC to interact with another non-GBC device capable of both sending and receiving infrared signals. These objects are designed to work specifically with GBCs and send pulses in much the same manner as a GBC would. Examples include Mystery Gifts via the Pokemon Pikachu 2 and trading Points via the Pocket Sakura.
- Inactive Object Init: Transfers require the GBC to interact with another non-GBC device capable of sending infrared signals but not necessarily receiving them. These objects may not be designed to work specifically with GBCs (notable exception is the Full Changer). Communication is input-only for these cases. Examples include Zok Zok Heroes, Chee Chai Alien, the Bomberman Max games' special stages, and Mission Impossible's TV remote feature.

## Communication Protocol

Again, there is no set or established infrared protocol that games must follow. Many games vary in their approach. For example, the 2nd Generation Pokemon games use the GBC's hardware timers, while others have hardcoded values that count cycles to check timing. The simplest form is a bare-bones communication protocol, i.e. something like a binary Morse code where a "0" is a long ON-OFF pulse and "1" is a short ON-OFF pulse or vice versa. Properly done, data could have been short, compact, and easily converted into bytes in RAM. Sakura Taisen GB seems to follow this model in its communications with the Pocket Sakura. Not all games do this, however, and appear to be doing who knows that, opting instead for customized and specialized communications unique to each title. To illustrate this idea, it would be possible to use a table of given lengths of IR ON-OFF pulses so that individual bytes could be sent all at once instead of in a binary, bit-by-bit manner. A number of games try to send a few pulses when pressing input like the A button and wait for another GBC to echo that in response, but after the handshake, most of the IR pulses are impossible to understand without disassembling the code.

One thing to note is that 4 games in particular do share somewhat similar IR protocols, at least regarding the initial handshake between 2 GBCs. They are Pokemon TCG 1 & 2 and Bomberman Max Red & Blue, all from the "2-Player Init" category above. Typically, IR capable GBC games will continually wait for an IR signal on both sides, i.e. the "1-Player Init" category. When one player presses certain input, that GBC takes the initiative and sends out a few IR pulses. That is to say, for most IR games, it only takes *just one* player to start the entire process.

The handshake for the 4 games previously mentioned, however, requires *both* players to input at almost the same time. One has to be slightly faster or slower than the other. Each side continually sends a few IR pulses, then reads the sensor to see if anything was received. If so, the GBCs begin to sync. The idea is that one side should be sending while the other is checking, and then the handshake completes. This is why one side needs to be faster or slower to input; if they are sending IR signals at the same time, they don't see anything when reading the sensor. As a result, both GBCs cannot input at exactly the same time. Practically speaking, this is unlikely to happen under normal circumstances, since most humans can't synchronize their actions down to a handful of microseconds, so the handshake will normally succeed.

## RP Register

The following is just theory. This handshake is possibly an artifact of the HuC-1. Consider that the Japanese version of Pokemon TCG 1 used the HuC-1 for its IR communications, and the developers may have borrowed the "best practices" used by other HuC-1/"GB KISS" games. When bringing Pokemon TCG 1 overseas, the IR handling code was likely minimally adapted to use the GBC's IR port, with the underlying protocol remaining unchanged in most regards. Pokemon TCG 2 ditched the HuC-1 in favor of the GBC IR port, so the IR code from non-Japanese versions of Pokemon TCG 1 was copy+pasted. The Bomberman games were made by Hudson Soft, literally the same people who created the HuC-1 in the first place. They too probably used the same protocol that had worked forever in their "GB KISS" games, so they used the same handshake method as before, just on the GBC IR port now. More research into the HuC-1 itself and the games needs to be done to confirm any of this.

On the GBC, the MMIO register located at \$FF56 controls infrared communication. Simply known as "RP" (Radiation Port? Reception Port? Red Port???), it is responsible for sending and receiving IR signals. Below is a diagram of the 8-bit register:

| Bit(s) | Effect                                        | Access |
|--------|-----------------------------------------------|--------|
| 0      | Turn IR light ON (1) or OFF (0)               | R/W    |
| 1      | Bit 1 = 1                                     | R      |
| 2-5    | Unused                                        |        |
| 6-7    | Signal Read Enable (0 = Disable) (3 = Enable) | R/W    |

Turning on the IR light is as simple as writing to Bit 0 of RP. Reading is a bit more complicated. Bits 6 and 7 must both be set (\$C0), to read Bit 1, otherwise Bit 1 returns 1, acting as if no signal is detected, except in edge cases detailed below in "Obscure Behavior". With signal reading enabled, Bit 1 will determine the status of any incoming IR signals. Like other Game Boy MMIO registers, unused bits read high (set to 1).

## Signal Fade

The IR sensor in the GBC adapts to the current level of IR light. That is to say, if the GBC receives a sustained IR signal beyond a certain amount of time, eventually the sensor treats this as a new "normal" level of IR light, and Bit 1 of RP goes back to 1. This is called the signal "fade" because it may appear as if the signal disappears.

Signal fade time is dependent on length and has an inverse relationship with the distance between a GBC and the IR light. The closer a GBC is to the IR source, the longer the fade time. The farther away a GBC is to the IR source, the shorter the fade time. One possible explanation for everything is that the IR signal is weaker on the receiving end, so the signal is prone to get "lost" to surrounding noise. The GBC IR sensor is probably good at sending IR signals (evidenced by the Mission Impossible cheat to turn a GBC into a TV remote) but not so good at picking up signals (evidenced by Chee Chai Aliens plastic add-on to enhance IR reception).

At about 3.0 to 3.5 inches (7.62 to 8.89 cm) signal fade time appears to be around 3ms. Optimal distance seems to be 2.5 to 4.0 inches (6.35 to 10.16 cm) to maintain a fade time close to 3ms and avoid potential miscommunication. One oddity of note is that putting two GBCs very close together (physically touching) produced unusually short fade times, far shorter than 3ms. There may be some sort of interference at that range.

## Obscure Behavior

The RP register has one very strange quirk. Disabling Bits 6 and 7 and then subsequently re-enabling them causes Bit 1 to go to zero under certain conditions. In other words, the IR sensor will act as if it is detecting a signal if reading the signal is disabled then enabled. It seems this behavior happens in the presence of any light; covering up the sensor during the read signal disable/enable causes the sensor to act normally. It's possible that the sensor resets itself (to its lowest level of detection???) and immediately detects any infrared sources, even from ambient/environmental light. The presence of any noise may temporarily trick the sensor into "seeing" IR light. By abusing this behavior, the GBC has some rudimentary ability to gauge the type of nearby lighting:

| Result of 1st RP Write (\$00) | Result of 2nd RP Write (\$C0) | Type of Lighting |
|-------------------------------|-------------------------------|------------------|
| Bit 1 = 1                     | Bit 1 = 1                     | Dark             |
| Bit 1 = 0                     | Bit 1 = 1                     | Ambient          |
| Bit 1 = 0 (sometimes 1)       | Bit 1 = 0                     | Bright           |

Writing \$00 to RP, followed by \$C0 will trigger these results listed above. One very important thing to note is that when enabling Bits 6 and 7 (writing \$C0), it does take some time for the sensor to register legitimate IR light coming into the sensor. I.e. if you want to use this method to detect what kind of light a GBC is looking at, the software needs to loop for a bit until Bit 1 of RP changes. Generally a few hundred cycles in double-speed mode will suffice. If Bit 1 of RP remains 1 after the loop, it's safe to assume the lighting is either ambient or dark. This delay doesn't seem to happen when Bits 6 and 7 are never disabled (which is what most official GBC software does). Games typically write either \$C0 or \$C1 to RP, with a small handful setting it to \$00 initially when setting up other MMIO registers (Pokemon G/S/C does this).

The downside to this method is that when detecting a bright IR source, the sensor quickly adjusts to this new level, and the next attempt at writing \$00 followed by \$C0 to RP will result in readings of dark or ambient (typically dark though). Essentially the bright result only appears briefly when transitioning from lower levels of light, then it "disappears" thanks to the short time it takes for IR signal fade. Designing a game mechanic (darkness and light) around this quirk is still possible, although it would require careful thought and planning to properly work around the observed limitations.

One suggested method: once the Bright setting is detected, switch to writing only \$C0 to RP so that the IR sensor works normally. If IR light stops being detected, switch to alternating \$00 and \$C0 writes as described above to determine Dark or Ambient settings. Whether it's practical or not to do this in a game remains theoretical at this point.
