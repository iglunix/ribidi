//type FriBidiChar = u32;
//type FriBidiStrIndex = usize;

use unicode_bidi::{bidi_class, BidiClass, BidiInfo};

#[no_mangle]
pub extern "C" fn fribidi_get_bidi_type(c: u32) -> usize {
    let bc = bidi_class(std::char::from_u32(c).unwrap());
    (match bc {
        // BidiClass::LTR => 'L',
        // BidiClass::RTL => 'R',
        BidiClass::AL => 'A',
        BidiClass::EN => '1',
        BidiClass::AN => '9',
        BidiClass::ES => 'w',
        BidiClass::ET => 'w',
        BidiClass::CS => 'w',
        BidiClass::NSM => '`',
        BidiClass::BN => 'b',
        // BidiClass::BS => 'B',
        // BidiClass::SS => 'S',
        BidiClass::WS => '_',
        BidiClass::ON => 'n',
        BidiClass::LRE => '+',
        BidiClass::RLE => '+',
        BidiClass::LRO => '+',
        BidiClass::RLO => '+',
        BidiClass::PDF => '-',
        BidiClass::LRI => '+',
        BidiClass::RLI => '+',
        BidiClass::FSI => '+',
        BidiClass::PDI => '-',
        // BidiClass::U => '|',
        BidiClass::R => 'R',
        // BidiClass::D => '+',
        // BidiClass::C => '-',
        // BidiClass::T => '^',
        BidiClass::L => '>',
        // BidiClass::G => '~',
        BidiClass::B => 'B',
        BidiClass::S => 'S',
    }) as usize
}

#[no_mangle]
pub extern "C" fn fribidi_log2vis(
    string: *const u32,
    len: usize,
    pbase_dir: *const usize,
    visual_str: *mut u32,
    position_L_to_V_list: *mut usize,
    position_V_to_L_list: *mut usize,
    embedding_level_list: *mut i8,
) -> bool {
    let u32_str = unsafe { std::slice::from_raw_parts(string, len) };
    let y: String = u32_str
        .iter()
        .map(|x| std::char::from_u32(*x).unwrap())
        .collect();
    let bidi_info = BidiInfo::new(&y, None);
    let para = &bidi_info.paragraphs[0];
    let line = para.range.clone();
    let display_str = bidi_info.reorder_line(para, line);
    true
}
