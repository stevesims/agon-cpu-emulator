use ez80::Machine;

// FatFS struct FIL
pub const SIZEOF_MOS_FIL_STRUCT: u32 = 36;
pub const FIL_MEMBER_OBJSIZE: u32 = 11;
pub const FIL_MEMBER_FPTR: u32 = 17;
// FatFS struct FILINFO
pub const SIZEOF_MOS_FILINFO_STRUCT: u32 = 278;
pub const FILINFO_MEMBER_FSIZE_U32: u32 = 0;
//pub const FILINFO_MEMBER_FDATE_U16: u32 = 4;
//pub const FILINFO_MEMBER_FTIME_U16: u32 = 6;
pub const FILINFO_MEMBER_FATTRIB_U8: u32 = 8;
//pub const FILINFO_MEMBER_ALTNAME_13BYTES: u32 = 9;
pub const FILINFO_MEMBER_FNAME_256BYTES: u32 = 22;
// f_open mode (3rd arg)
//pub const FA_READ: u32 = 1;
pub const FA_WRITE: u32 = 2;
pub const FA_CREATE_NEW: u32 = 4;
pub const FA_CREATE_ALWAYS: u32 = 8;

#[derive(Clone, Default)]
pub struct MosMap {
    pub f_chdir: u32,
    pub _f_chdrive: u32,
    pub f_close: u32,
    pub f_closedir: u32,
    pub _f_getcwd: u32,
    pub _f_getfree: u32,
    pub f_getlabel: u32,
    pub f_gets: u32,
    pub f_lseek: u32,
    pub f_mkdir: u32,
    pub f_mount: u32,
    pub f_open: u32,
    pub f_opendir: u32,
    pub _f_printf: u32,
    pub f_putc: u32,
    pub _f_puts: u32,
    pub f_read: u32,
    pub f_readdir: u32,
    pub f_rename: u32,
    pub _f_setlabel: u32,
    pub f_stat: u32,
    pub _f_sync: u32,
    pub _f_truncate: u32,
    pub f_unlink: u32,
    pub f_write: u32,
}

impl MosMap {
    pub fn from_symbol_map(map: std::collections::HashMap<String, u32>) -> Result<MosMap, &'static str> {
        let mut mos_map = MosMap::default();
        let err = "Missing MOS FS symbol in symbol map";

        mos_map.f_chdir = *(map.get("_f_chdir").ok_or(err)?);
        mos_map._f_chdrive = *(map.get("_f_chdrive").ok_or(err)?);
        mos_map.f_close = *(map.get("_f_close").ok_or(err)?);
        mos_map.f_closedir = *(map.get("_f_closedir").ok_or(err)?);
        mos_map._f_getcwd = *(map.get("_f_getcwd").ok_or(err)?);
        mos_map._f_getfree = *(map.get("_f_getfree").ok_or(err)?);
        mos_map.f_getlabel = *(map.get("_f_getlabel").ok_or(err)?);
        mos_map.f_gets = *(map.get("_f_gets").ok_or(err)?);
        mos_map.f_lseek = *(map.get("_f_lseek").ok_or(err)?);
        mos_map.f_mkdir = *(map.get("_f_mkdir").ok_or(err)?);
        mos_map.f_mount = *(map.get("_f_mount").ok_or(err)?);
        mos_map.f_open = *(map.get("_f_open").ok_or(err)?);
        mos_map.f_opendir = *(map.get("_f_opendir").ok_or(err)?);
        mos_map._f_printf = *(map.get("_f_printf").ok_or(err)?);
        mos_map.f_putc = *(map.get("_f_putc").ok_or(err)?);
        mos_map._f_puts = *(map.get("_f_puts").ok_or(err)?);
        mos_map.f_read = *(map.get("_f_read").ok_or(err)?);
        mos_map.f_readdir = *(map.get("_f_readdir").ok_or(err)?);
        mos_map.f_rename = *(map.get("_f_rename").ok_or(err)?);
        mos_map._f_setlabel = *(map.get("_f_setlabel").ok_or(err)?);
        mos_map.f_stat = *(map.get("_f_stat").ok_or(err)?);
        mos_map._f_sync = *(map.get("_f_sync").ok_or(err)?);
        mos_map._f_truncate = *(map.get("_f_truncate").ok_or(err)?);
        mos_map.f_unlink = *(map.get("_f_unlink").ok_or(err)?);
        mos_map.f_write = *(map.get("_f_write").ok_or(err)?);

        Ok(mos_map)
    }
}

pub static MOS_103_MAP: MosMap = MosMap {
    f_chdir    : 0x82B2,
    _f_chdrive : 0x827A,
    f_close    : 0x822B,
    f_closedir : 0x8B5B,
    _f_getcwd  : 0x8371,
    _f_getfree : 0x8CE8,
    f_getlabel : 0x9816,
    f_gets     : 0x9C91,
    f_lseek    : 0x8610,
    f_mkdir    : 0x92F6,
    f_mount    : 0x72F7,
    f_open     : 0x738C,
    f_opendir  : 0x8A52,
    _f_printf  : 0x9F11,
    f_putc     : 0x9E8E,
    _f_puts    : 0x9EC4,
    f_read     : 0x785E,
    f_readdir  : 0x8B92,
    f_rename   : 0x9561,
    _f_setlabel: 0x99DB,
    f_stat     : 0x8C55,
    _f_sync    : 0x8115,
    _f_truncate: 0x8F78,
    f_unlink   : 0x911A,
    f_write    : 0x7C10,
};

/**
 * Like z80_mem_tools::get_cstring, except \r and \n are accepted as
 * string terminators as well as \0
 */
pub fn get_mos_path_string<M: Machine>(machine: &M, address: u32) -> Vec<u8> {
    let mut s: Vec<u8> = vec![];
    let mut ptr = address;

    loop {
        match machine.peek(ptr) {
            0 | 10 | 13 => break,
            b => s.push(b)
        }
        ptr += 1;
    }
    s
}
