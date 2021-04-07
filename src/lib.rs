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
        BidiClass::L => 'L',
        // BidiClass::G => '~',
        BidiClass::B => 'B',
        BidiClass::S => 'S',
    }) as usize
}

#[no_mangle]
pub extern "C" fn fribidi_get_bidi_types(string: *const u32, len: usize, btypes: *mut usize) {
    for (i, c) in unsafe { std::slice::from_raw_parts(string, len).iter().enumerate() } {
        unsafe {
                *btypes.offset(i as isize) = fribidi_get_bidi_type(*c);
        }
    }
}

#[no_mangle]
pub extern "C" fn fribidi_get_bidi_type_name(btype: usize) -> *const u8 {
    use std::ffi::CString;
    let mut string = String::new();
    string.push(std::char::from_u32(btype as u32).unwrap());
    string.as_ptr()
}

fn string_from_utf32_raw_parts(string: *const u32, len: usize) -> String {
    unsafe {
            std::slice::from_raw_parts(string, len)
    }.iter().map(|x| std::char::from_u32(*x).unwrap()).collect()
}

fn string_to_utf32_raw() {

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
    let y = string_from_utf32_raw_parts(string, len);
    let bidi_info = BidiInfo::new(&y, None);
    let para = &bidi_info.paragraphs[0];
    let line = para.range.clone();
    let display_str = bidi_info.reorder_line(para, line);
    for (i, embedding_level) in bidi_info.levels.iter().enumerate() {
        unsafe {
            *embedding_level_list.offset(i as isize) = embedding_level.number() as i8;
        }
    }
    true
}

#[no_mangle]
pub extern "C" fn fribidi_get_bracket(c: u32) -> u32 {
    /*
         * Servo doesn't implement this yet
         * https://github.com/servo/unicode-bidi/issues/3
         * 
         * just return the c; should be fine for now
         */
        c
}

#[no_mangle]
pub extern "C" fn fribidi_get_bracket_types(string: *const u32, len: usize, _types: *const usize, btypes: *mut u32) {
    for (i, c) in unsafe { std::slice::from_raw_parts(string, len).iter().enumerate() } {
        unsafe {
                *btypes.offset(i as isize) = fribidi_get_bracket(*c);
        }
    }
}

#[no_mangle]
pub extern "C" fn fribidi_get_par_embedding_levels(
    bidi_types: *const u32,
    bracket_types: *const u32,
    len: usize,
    pbase_dir: usize,
    embedding_levels: usize
) -> usize {
    0
}

#[no_mangle]
pub extern "C" fn fribidi_get_joining_type(c: u32) -> usize {
    /* unjoining */ 0
}

#[no_mangle]
pub extern "C" fn fribidi_get_joining_types(
    string: *const u32,
    len: usize,
    jtypes: *mut usize
) {
    for (i, c) in unsafe { std::slice::from_raw_parts(string, len).iter().enumerate() } {
        unsafe {
                *jtypes.offset(i as isize) = fribidi_get_joining_type(*c);
        }
    }
}

#[no_mangle]
pub extern "C" fn fribidi_get_par_embedding_levels_ex(
    types: *const usize,
    btypes: *const u32,
    len: usize,
    base_dir: *mut u32,
    embedding_levels: *mut u32
) -> u32 {
    for (i, c) in unsafe {
        std::slice::from_raw_parts(base_dir, len).iter().enumerate()
    } {
        unsafe {
            *base_dir.offset(i as isize) = 'n' as u32;
            *embedding_levels.offset(i as isize) = 0;
        }
    }
    1
}

#[no_mangle]
pub extern "C" fn fribidi_join_arabic(
    types: *const usize,
    len: usize,
    embedding_levels: *const u32,
    ar_props: *mut usize
) {
    for (i, c) in unsafe {
        std::slice::from_raw_parts(types, len).iter().enumerate()
    } {
        unsafe {
            *ar_props.offset(i as isize) = 0;
        }
    }
}

#[no_mangle]
pub extern "C" fn fribidi_reorder_line(
    flags: u32,
    types: *const u32,
    len: usize,
    off: usize,
    base_dir: *const u32,
    embedding_levels: *mut u32,
    visual_str: *mut u32,
    map: *mut usize
) -> u32 {
    for (i, c) in unsafe {
        std::slice::from_raw_parts(visual_str.offset(off as isize), len).iter().enumerate()
    } {
        unsafe {
            if map != std::ptr::null_mut() {
                *map.offset(i as isize) = i;
            }
        }
    }
    1
}

#[no_mangle]
pub extern "C" fribidi_shape(
    flags: u32,
    levels: *const u32,
    len: usize,
    ar_props: *mut usize,
    string: *mut u32
) {

}
