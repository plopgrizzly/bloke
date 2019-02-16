/// A Bloke (from a `*.bloke` file).
///
/// A Bloke File stores your Bloke (`*.bloke`).  Games may render the Bloke according to their own
/// style, and don't have to read all of the fields.  For creating your Bloke, their will be a
/// public domain standard bloke renderer, that does read all of the fields and adjust rendering
/// appropriately.  The file is a simple format of listing shorts (integers 0-65535) which specify
/// how much of an attribute the Bloke has.
///
/// The file format is always 192 bytes uncompressed.  It describes a human-like figure by
/// attributes that are measured on 16-bit continuums.  It does not store a binary bit for gender.
/// It also does not include clothing or jewelery (video games often have their own in game as
/// items).  What is included are other stylistic choices like hair dye and nail polish.  Video
/// games may also apply this format to other imaginary life forms in game, which are not human
/// (example: a game where you play as an alien).
pub struct Bloke {
    /// Version must be 1
    pub version: u16,
    /// Default: 32768 (65535 = tall arch, 0 = flat feet)
    pub arch_height: u16,
    /// Default: 32768 (65535 = tall legs, 0 = short legs)
    pub legs_height: u16,
    /// Default: 32768 (65535 = tall body, 0 = short body)
    pub body_height: u16,
    /// Default: 32768 (65535 = long neck, 0 = short neck)
    pub neck_height: u16,
    /// Default: 32768 (65535 = tall head, 0 = short head)
    pub head_height: u16,
    /// Default: 32768 (65535 = tall/thick hair, 0 = bald)
    pub hair_height: u16,
    /// Default: 32768 (65535 = big feet, 0 = small feet)
    pub foot_size: u16,
    /// Default: 32768 (65535 = long toes, 0 = short toes)
    pub toes_size: u16,
    /// Default: 32768 (65535 = fat leg, 0 = no-fat leg)
    pub legs_size: u16,
    /// Default: 32768 (65535 = big knee, 0 = no visible knee)
    pub knee_size: u16,
    /// Default: 32768 (65535 = muscular lower leg, 0 = no-muscles lower leg)
    pub calf_size: u16,
    /// Default: 32768 (65535 = muscular upper leg, 0 = no-muscles upper leg)
    pub hams_size: u16,
    /// Default: 32768 (65535 = big butt, 0 = small butt)
    pub butt_size: u16,
    /// Default: 32768 (65535 = wide body, 0 = thin body)
    pub body_size: u16,
    /// Default: 32768 (65535 = beer gut, 0 = flat stomach)
    pub belly_size: u16,
    /// Default: 32768 (65535 = has chest, 0 = flat chest)
    pub chest_size: u16,
    /// Default: 32768 (65535 = big arch spine, 0 = straight spine)
    pub spine_size: u16,
    /// Default: 32768 (65535 = hunched over spine, 0 = straight spine)
    pub hunch_size: u16,
    /// Default: 32768 (65535 = sloped shoulder, 0 = flat shoulder)
    pub shoulder_size: u16,
    /// Default: 32768 (65535 = muscular arms, 0 = no-muscle arms)
    pub arm_muscle_size: u16,
    /// Default: 32768 (65535 = fat arms, 0 = no-fat arms)
    pub arm_fat_size: u16,
    /// Default: 32768 (65535 = big elbow, 0 = no visible elbow)
    pub elbow_size: u16,
    /// Default: 32768 (65535 = big hands, 0 = small hands)
    pub hand_size: u16,
    /// Default: 32768 (65535 = long fingers, 0 = short fingers)
    pub finger_size: u16,
    /// Default: 32768 (65535 = big head, 0 = small head)
    pub head_size: u16,
    /// Default: 32768 (65535 = big nose, 0 = small nose)
    pub nose_size: u16,
    /// Default: 32768 (65535 = pointy nose, 0 = flat nose)
    pub nosepoint_size: u16,
    /// Default: 32768 (65535 = big nostrils, 0 = small nostrils)
    pub nostril_size: u16,
    /// Default: 32768 (65535 = wide mouth, 0 = small mouth)
    pub mouth_size: u16,
    /// Default: 32768 (65535 = big lips, 0 = no lips)
    pub lips_size: u16,
    /// Default: 32768 (65535 = big eyes, 0 = small eyes)
    pub eyes_size: u16,
    /// Default: 32768 (65535 = eyes rest mostly closed, 0 = eyes rest mostly open)
    pub eyelid_size: u16,
    /// Default: 32768 (65535 = concave/deep around eyes, 0 = eyes as far out as forehead)
    pub eyearea_size: u16,
    /// Default: 32768u16 (65535 = long eyelashes, 0 = short eyelashes)
    pub eyelash_size: u16,
    /// Default: 32768u16 (65535 = curved upwards, 0 = curved downwards)
    pub eyelashcurve_size: u16,
    /// Default: 32768u16 (65535 = thick eyebrows, 0 = thin eyebrows)
    pub eyebrow_thickness: u16,
    /// Default: 32768u16 (65535 = unibrow, 0 = no eyebrows)
    pub eyebrow_width: u16,
    /// Default: 32768u16 (65535 = ears stick out, 0 = flat ears)
    pub ear_size: u16,
    /// Default: 32768u16 (65535 = double chin, 0 = mouth very close to chin)
    pub chin_size: u16,
    /// Default: 32768u16 (65535 = spaced out face, 0 = compact face)
    pub face_size: u16,
    /// Default: 32768u16 (65535 = tall forehead, 0 = short forehead)
    pub forehead_size: u16,
    /// Default: 32768u16 (65535 = convex/chipmunk cheeks, 0 = concave/bony cheeks)
    pub cheek_size: u16,
    /// Default: 32768u16 (65535 = wrinkly skin, 0 = shiny skin)
    pub wrinkle_size: u16,
    /// Default: 0u16 (65535 = long mustache (extends off face), 0 = no mustache)
    pub mustache_size: u16,
    /// Default: 0u16 (65535 = long beard (down to knees), 0 = no beard)
    pub beard_size: u16,
    /// Default: 32768u16 (65535 = thick/long hair on sides, 0 = shaved on sides hair)
    pub sidehair_size: u16,
    /// Default: 32768u16 (65535 = long hair, 0 = shaved)
    pub bodyhair_size: u16,
    /// Default: 32768u16 (65535 = protruding bottom of chin, 0 = rounded bottom of chin)
    pub chinextend_size: u16,
    /// Default: 32768u16 (65535 = back of head extends, 0 = back of head is flat with neck)
    pub backhead_size: u16,
    /// Default: 32768u16 (65535 = hair down to ankles, 0 = hair to top of ears)
    pub longhair_size: u16,
    /// Default: 32768u16 (65535 = wide hips, 0 = small hips)
    pub hips_size: u16,
    /// Default: 32768u16 (65535 = wide waist, 0 = small waist)
    pub waist_size: u16,
    /// Default: 32768u16 (65535 = wide ankles, 0 = small ankles)
    pub ankle_size: u16,
    /// Default: 32768u16 (65535 = long foot hair, 0 = no foot hair)
    pub foothair_size: u16,
    /// Default: 32768u16 (65535 = long armpit hair, 0 = no armpit hair)
    pub armpithair_size: u16,
    /// Default: 32768u16 (65535 = long nose hair, 0 = no nose hair)
    pub nosehair_size: u16,
    /// Default: 32768 (65535 = round body, 0 = straight body)
    pub roundness_body: u16,
    /// Default: 32768u16 (65535 = finger rounded edges, 0 = finger right angles)
    pub roundness_finger: u16,
    /// Default: 32768u16 (65535 = round head, 0 = almost-cubelike-head)
    pub roundness_head: u16,
    /// Default: 32768u16 (65535 = round ear, 0 = pointy ear)
    pub roundness_ear: u16,
    /// Default: 32768u16 (65535 = more brown-red color, 0 = more brown-blue color)
    pub color_skinhue: u16,
    /// Default: 32768u16 (65535 = dark (black) skin, 0 = light (white) skin)
    pub color_skinshade: u16,
    /// Default: 32768u16 (65535 = light red, 0 = light blue)
    pub color_lip: u16,
    /// Default: 0 (65535 = yellow, 0 = white)
    pub color_teeth: u16,
    /// Default: 0b1111_1111_1111_1111 (4 bit = 4 teeth, top left, right, bottom left, right)
    pub havewhich_teeth: u16,
    /// Default: 32768u16 (65535 = long fingernails, 0 = short fingernails)
    pub len_nail: u16,
    /// Default: 32768u16 (65535 = long, 0 = short)
    pub len_arm: u16,
    /// Default: 32768u16 (65535 = eyes far apart, 0 = eyes close to each other)
    pub dist_eye: u16,
    /// Default: 32768 (65535 = face & limb freckles, 32768 = limb freckles, 0 = no freckles)
    pub freckles: u16,
    /// Default: (32 bit RGBA)
    pub color_headhair1: u32,
    /// Default: (32 bit RGBA)
    pub color_headhair2: u32,
    /// Default: (32 bit RGBA)
    pub color_headhair3: u32,
    /// Default: (32 bit RGBA)
    pub color_bodyhair: u32,
    /// Default: (32 bit RGBA)
    pub color_mustache: u32,
    /// Default: (32 bit RGBA)
    pub color_beard: u32,
    /// Default: (32 bit RGBA)
    pub color_foothair: u32,
    /// Default: (32 bit RGBA)
    pub color_armpithair: u32,
    /// Default: (32 bit RGBA)
    pub color_eyelash: u32,
    /// Default: (32 bit RGBA)
    pub color_eyebrow: u32,
    /// Default: (32 bit RGBA)
    pub color_eye: u32,
    /// Default: (32 bit RGBA)
    pub color_fingertoenail: u32,
    /// Default: 32768u16 (65535 = high pitched, 0 = low pitched)
    pub voice_pitch: u16,
    /// Default: 32768u16 (TODO)
    pub voice_waveform: u16,
}

#[cfg(test)]
mod tests {
    #[test]
    fn filesize() {
        assert_eq!(std::mem::size_of::<crate::Bloke>(), 192)
    }
}
