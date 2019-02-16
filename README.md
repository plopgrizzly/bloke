# Bloke
A video game avatar system, similar to Nintendo's Mii - but open source and for PC / phone games.

## Files
A Bloke File stores your Bloke (`*.bloke`). Games may render the Bloke according to their own style, and don't have to read all of the fields. For creating your Bloke, their will be a public domain standard bloke renderer, that does read all of the fields and adjust rendering appropriately. The file is a simple format of listing shorts (integers 0-65535) which specify how much of an attribute the Bloke has.

The file format is always 192 bytes uncompressed. It describes a human-like figure by attributes that are measured on 16-bit continuums. It does not store a binary bit for gender. It also does not include clothing or jewelery (video games often have their own in game as items). What is included are other stylistic choices like hair dye and nail polish. Video games may also apply this format to other imaginary life forms in game, which are not human (example: a game where you play as an alien).
