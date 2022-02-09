use windows::Win32::Storage::FileSystem::*;

fn print_file_info(info: &FILE_NOTIFY_INFORMATION) {
    match info.Action {
        FILE_ACTION_ADDED => println!("File added"),
        FILE_ACTION_REMOVED => println!("File removed"),
        FILE_ACTION_MODIFIED => println!("File modified"),
        FILE_ACTION_RENAMED_NEW_NAME => println!("File renamed to"),
        FILE_ACTION_RENAMED_OLD_NAME => println!("File renamed from"),
        _ => panic!("Unknown action: {:?}", info.Action),
    }

    let str = core::ptr::slice_from_raw_parts(
        core::ptr::addr_of!(info.FileName) as *const u16,
        info.FileNameLength as usize / 2,
    );
    let name = unsafe { String::from_utf16_lossy(&*str) };
    println!("Name: {}", name);
}

unsafe fn read_dir_changes(dir: &str) {
    let handle = CreateFileW(
        dir,
        FILE_LIST_DIRECTORY,
        FILE_SHARE_READ | FILE_SHARE_WRITE,
        std::ptr::null(),
        OPEN_EXISTING,
        FILE_FLAG_BACKUP_SEMANTICS,
        None,
    );
    let mut buffer = [0u8; 1024];
    let mut bytes_returned = 0u32;

    while ReadDirectoryChangesW(
        handle,
        buffer.as_mut_ptr() as _,
        1024,
        false,
        FILE_NOTIFY_CHANGE_CREATION | FILE_NOTIFY_CHANGE_DIR_NAME | FILE_NOTIFY_CHANGE_FILE_NAME,
        &mut bytes_returned,
        std::ptr::null_mut(),
        None,
    )
    .as_bool()
    {
        let mut info: *const FILE_NOTIFY_INFORMATION = std::mem::transmute(&buffer);
        print_file_info(&*info);
        while (*info).NextEntryOffset != 0 {
            let next = (*info).NextEntryOffset as usize;
            info = std::mem::transmute(std::mem::transmute::<_, *const u8>(info).add(next));
            print_file_info(&*info);
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Usage: {} <directory>", args[0]);
        return;
    }
    unsafe { read_dir_changes(&args[1][..]) }
}
