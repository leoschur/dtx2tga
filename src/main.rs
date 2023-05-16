use ::std::{
    env, fmt,
    fs::File,
    mem::{size_of, transmute},
    slice,
};
use std::{default, os::unix::prelude::FileExt};

#[derive(Debug)]
#[repr(u32)]
enum Flags {
    /// This DTX has fullbrite colors.
    DtxFullbrite = (1 << 0),
    /// Use 16-bit, even if in 32-bit mode.
    DtxPrefer16bit = (1 << 1),
    /// Used to make some of the tools stuff easier..
    /// This means each TextureMipData has its texture data allocated.
    DtxMipsalloced = (1 << 2),
    /// The sections count was screwed up originally.
    /// This flag is set in all the textures from now on when the count is fixed.
    DtxSectionsfixed = (1 << 3),
    /// Not saved: used internally.. tells it to not put the texture in the texture cache list.
    DtxNosyscache = (1 << 6),
    /// If in 16-bit mode, use a 4444 texture for this.
    DtxPrefer4444 = (1 << 7),
    /// Use 5551 if 16-bit.
    DtxPrefer5551 = (1 << 8),
    /// If there is a sys copy - don't convert it to device specific format (keep it 32 bit).
    Dtx32bitsyscopy = (1 << 9),
    /// Cube environment map. +x is stored in the normal data area
    /// -x,+y,-y,+z,-z are stored in their own sections
    DtxCubemap = (1 << 10),
    /// Bump mapped texture, this has 8 bit U and V components for the bump normal
    DtxBumpmap = (1 << 11),
    /// Bump mapped texture with luminance, this has 8 bits for luminance, U and V
    DtxLumbumpmap = (1 << 12),
}

// #define CURRENT_DTX_VERSION -5  // m_Version in the DTX header.

/**
 *  Extra data.  Here's how it's layed out:
 *  m_Extra[0]      = Texture group.
 *  m_Extra[1]      = Number of mipmaps to use (there are always 4 in the file,
 *                    but this says how many to use at runtime).
 *  m_Extra[2]      = BPPIdent telling what format the texture is in.
 *  m_Extra[3]      = Mipmap offset if the card doesn't support S3TC compression.
 *  m_Extra[4]      = Mipmap offset applied to texture coords (so a 512 could be
 *                    treated like a 256 or 128 texture in the editor).
 *  m_Extra[5]      = Texture priority (default 0).
 *  m_Extra[6-9]    = Detail texture scale (float value).
 *  m_Extra[10-11]  = Detail texture angle (integer degrees)s
 */
#[repr(C)]
union ExtraData {
    extra: [u8; 12],      // 12,
    extra_long: [u32; 3], // 3
}

impl fmt::Debug for ExtraData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(unsafe { self.extra_long.iter() })
            .finish()
    }
}

const COMMAND_STRING_SIZE: usize = 128;

#[repr(C)]
#[derive(Debug)]
struct Header {
    m_res_type: u32,
    m_base_width: u16,
    m_base_height: u16,
    m_version: i32, // CURRENT_DTX_VERSION (-5 is expected)
    m_n_mipmaps: u16,
    m_n_sections: u16,

    m_iflags: i32,     // Combination of DTX_ flags.
    m_user_flags: i32, // Flags that go on surfaces.

    extra: ExtraData,

    m_command_string: [u8; 128],
}

impl Header {
    fn new() -> Header {
        Header {
            m_res_type: 0,
            m_version: 0,
            m_base_width: 0,
            m_base_height: 0,
            m_n_mipmaps: 0,
            m_n_sections: 0,
            m_iflags: Default::default(),
            m_user_flags: 0,
            extra: ExtraData { extra_long: [0; 3] },
            m_command_string: [0; 128],
        }
    }
    fn parse(f: File) -> Header {
        let mut buf = [0u8; size_of::<Header>()];
        f.read_exact_at(&mut buf[..], 0).unwrap();
        unsafe { transmute(buf) }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    // let output = &args[2];

    let mut file = match File::open(input) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open file {}: {:?}", input, e),
    };

    let header = Header::parse(file);
    dbg!(header);
}
