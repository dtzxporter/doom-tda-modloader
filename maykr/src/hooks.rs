use std::ffi::c_void;
use std::ffi::CStr;
use std::io::Cursor;
use std::mem::transmute;
use std::sync::OnceLock;

use minhook::MinHook;

use porter_utils::Pattern;
use porter_utils::StructWriteExt;

use crate::utilities::main_module;
use crate::Error;

/// IdFile constructor.
const IDFILE_MEMORY_CTOR: Pattern = Pattern::new("48 89 5C 24 18 55 56 57 48 81 EC ? ? ? ? 48 8B 05 ? ? ? ? 48 33 C4 48 89 84 24 40 01 00 00 48 8D 05");
/// IdFile readonly set.
const IDFILE_MEMORY_SETREADONLY: Pattern =
    Pattern::new("4C 89 81 28 01 00 00 4C 89 81 30 01 00 00");

/// Open container file.
const OPEN_CONTAINER: Pattern = Pattern::new("40 53 55 56 57 41 55 41 57 B8 ? ? ? ? E8 ? ? ? ? 48 2B E0 48 8B 05 ? ? ? ? 48 33 C4 48 89 84 24 60 41 00 00");

/// New method to allocate.
const ID_NEW: Pattern = Pattern::new("48 83 EC ? 4C 8B D1 C7 44 24 28");

type OpenContainer = extern "system" fn(
    id_file_system_local: u64,
    id_resource_manager: &mut IdResourceManager,
    id_resource_index: u32,
) -> i64;

type IdNew = extern "system" fn(size: i64) -> i64;

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct IdResourceManager {
    unknown: [u8; 0x198],
    resources: u64,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct IdResourceFile {
    file_type_index: *const i8,
    file_name_index: *const i8,

    unknown_10: u64,
    dependency_index: u64,

    file_meta_index: u64,
    unknown_28: u64,

    unknown_30: u64,
    data_offset: u64,

    data_size: u64,
    data_size_uncompressed: u64,

    data_checksum: u64,
    file_timestamp: u64,

    streamed_resource_hash: u64,
    version: u32,
    unknown_6c: u32,

    compression_type: u16,
    unknown_72: u16,
    unknown_74: u32,
    unknown_78: u32,
    unknown_7c: u32,

    unknown_80: u32,
    num_dependencies: u16,
    unknown_86: u16,
    unknown_88: u64,
}

type IdFileMemoryCtor = extern "system" fn(memory: i64, file_path: *const i8) -> i64;

type IdFileSetReadOnly = extern "system" fn(idfile: i64, data: *const u8, size: i64) -> ();

/// The original open file read method.
static OPEN_CONTAINER_ORIG: OnceLock<OpenContainer> = OnceLock::new();

static IDNEW_ORIG: OnceLock<IdNew> = OnceLock::new();
static IDFILE_MEMORY_CTOR_ORIG: OnceLock<IdFileMemoryCtor> = OnceLock::new();
static IDFILE_SET_READONLY_ORIG: OnceLock<IdFileSetReadOnly> = OnceLock::new();

extern "system" fn open_container_hook(
    id_file_system_local: u64,
    id_resource_manager: &mut IdResourceManager,
    id_resource_index: u32,
) -> i64 {
    let asset = id_resource_manager.resources
        + (size_of::<IdResourceFile>() * id_resource_index as usize) as u64;
    let asset = asset as *const IdResourceFile;
    let asset = unsafe { &*asset };

    let asset_name = unsafe { CStr::from_ptr(asset.file_name_index) }
        .to_string_lossy()
        .to_string();
    let asset_type = unsafe { CStr::from_ptr(asset.file_type_index) }
        .to_string_lossy()
        .to_string();

    // TODO: Just check local Fs, or use some json document to describe streams/assets to mod.
    if asset_name.contains("some_asset_to_mod") {
        let path = r"path_to_asset_on_disk";
        let data = std::fs::read(path).unwrap();
        let data = data.leak(); // oof, todo: hook dtor and free memory...

        // We need to patch the file using an idFileMemory.
        let file = IDNEW_ORIG.get().unwrap()(0x160);
        let file_ptr = IDFILE_MEMORY_CTOR_ORIG.get().unwrap()(file, asset.file_name_index);

        // We got a new file. Set the data from a buffer, we own it.
        IDFILE_SET_READONLY_ORIG.get().unwrap()(file_ptr, data.as_ptr(), data.len() as i64);

        let file_data = unsafe { std::slice::from_raw_parts_mut(file_ptr as *mut u8, 0x160) };
        let mut file_writer = Cursor::new(file_data);

        // TODO: Abuse page alignment and add a dtor hook that checks for a magic value at 0x160
        // if the magic value is present, we'll put the leaked pointer so it can be reconstructed and
        // dropped so that we don't actually leak any data.
        file_writer.set_position(0x140);
        file_writer.write_struct(0xFFFFFFFFu32).unwrap(); // timestamp.

        file_writer.set_position(0x158);
        file_writer.write_struct(0x0u8).unwrap(); // ownsData.

        println!("Container modded: {:?} {:?}", asset_type, asset_name);

        return file_ptr;
    }

    OPEN_CONTAINER_ORIG
        .get()
        .expect("Failed to call original open container")(
        id_file_system_local,
        id_resource_manager,
        id_resource_index,
    )
}

/// Intalls the hooks for maykr.
pub fn install_hooks() -> Result<(), Error> {
    #[cfg(debug_assertions)]
    println!("Installing hooks...");

    let main_module = main_module();
    let main_module_ptr: *mut c_void = main_module.as_mut_ptr() as _;

    #[cfg(debug_assertions)]
    println!(
        "Main module: {:p} -> {:p} ({:#02x?})",
        main_module_ptr,
        unsafe { main_module_ptr.add(main_module.len()) },
        main_module.len()
    );

    let open_container_offset = OPEN_CONTAINER
        .scan(&main_module)
        .ok_or(Error::PatternNotFound)?;

    let idfile_memory_ctor_offset = IDFILE_MEMORY_CTOR
        .scan(&main_module)
        .ok_or(Error::PatternNotFound)?;
    let idfile_set_readonly_offset = IDFILE_MEMORY_SETREADONLY
        .scan(&main_module)
        .ok_or(Error::PatternNotFound)?;
    let id_new_offset = ID_NEW.scan(&main_module).ok_or(Error::PatternNotFound)?;

    let open_container = unsafe { main_module_ptr.add(open_container_offset) };

    let idfile_memory_ctor = unsafe { main_module_ptr.add(idfile_memory_ctor_offset) };
    let idfile_set_readonly = unsafe { main_module_ptr.add(idfile_set_readonly_offset) };
    let id_new = unsafe { main_module_ptr.add(id_new_offset) };

    #[cfg(debug_assertions)]
    println!(
        "Found open container at: {:#02x?} ({:p})",
        open_container_offset, open_container
    );

    #[cfg(debug_assertions)]
    println!(
        "Found id file memory ctor at: {:#02x?} ({:p})",
        idfile_memory_ctor_offset, idfile_memory_ctor
    );
    #[cfg(debug_assertions)]
    println!(
        "Found id file set readonly at: {:#02x?} ({:p})",
        idfile_set_readonly_offset, idfile_set_readonly
    );
    #[cfg(debug_assertions)]
    println!("Found id new at: {:#02x?} ({:p})", id_new_offset, id_new);

    let open_container_orig =
        unsafe { MinHook::create_hook(open_container, open_container_hook as _)? };
    let open_container_orig =
        unsafe { transmute::<*mut c_void, OpenContainer>(open_container_orig) };

    let idfile_memory_ctor_orig =
        unsafe { transmute::<*mut c_void, IdFileMemoryCtor>(idfile_memory_ctor) };
    let idfile_set_readonly_orig =
        unsafe { transmute::<*mut c_void, IdFileSetReadOnly>(idfile_set_readonly) };
    let idnew_orig = unsafe { transmute::<*mut c_void, IdNew>(id_new) };

    OPEN_CONTAINER_ORIG
        .set(open_container_orig)
        .map_err(|_| Error::UnhandledError)?;

    IDFILE_MEMORY_CTOR_ORIG
        .set(idfile_memory_ctor_orig)
        .map_err(|_| Error::UnhandledError)?;

    IDFILE_SET_READONLY_ORIG
        .set(idfile_set_readonly_orig)
        .map_err(|_| Error::UnhandledError)?;

    IDNEW_ORIG
        .set(idnew_orig)
        .map_err(|_| Error::UnhandledError)?;

    unsafe { MinHook::enable_all_hooks()? };

    Ok(())
}
