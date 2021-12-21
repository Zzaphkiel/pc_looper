// All the code in this mod referenced the code below:
// https://github.com/naari3/pc_assist/blob/master/src/ppt.rs
// A thousand thanks to naari3 too.

// The map from u32 in PPT to board::Shape is defined in method 'from_ppt' of enum board::Shape.

use process_memory::{DataMember, Memory, ProcessHandle};
extern crate process_memory;

pub fn get_pid(process_name: &str) -> process_memory::Pid {
    /// A helper function to turn a c_char array to a String
    fn utf8_to_string(bytes: &[i8]) -> String {
        use std::ffi::CStr;
        unsafe {
            CStr::from_ptr(bytes.as_ptr())
                .to_string_lossy()
                .into_owned()
        }
    }
    let mut entry = winapi::um::tlhelp32::PROCESSENTRY32 {
        dwSize: std::mem::size_of::<winapi::um::tlhelp32::PROCESSENTRY32>() as u32,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [0; winapi::shared::minwindef::MAX_PATH],
    };
    let snapshot: process_memory::ProcessHandle;
    unsafe {
        snapshot = winapi::um::tlhelp32::CreateToolhelp32Snapshot(
            winapi::um::tlhelp32::TH32CS_SNAPPROCESS,
            0,
        );
        if winapi::um::tlhelp32::Process32First(snapshot, &mut entry)
            == winapi::shared::minwindef::TRUE
        {
            while winapi::um::tlhelp32::Process32Next(snapshot, &mut entry)
                == winapi::shared::minwindef::TRUE
            {
                if utf8_to_string(&entry.szExeFile) == process_name {
                    return entry.th32ProcessID;
                }
            }
        }
    }
    0
}

pub struct Ppt {
    pub process_handle: ProcessHandle,
}

impl Ppt {
    pub fn still_active(&self) -> std::io::Result<bool> {
        let mut exit_code: winapi::shared::minwindef::DWORD = 0;
        if unsafe {
            winapi::um::processthreadsapi::GetExitCodeProcess(self.process_handle, &mut exit_code)
        } == winapi::shared::minwindef::FALSE
        {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(exit_code == winapi::um::minwinbase::STILL_ACTIVE)
        }
    }

    pub fn get_current_piece(&self) -> Option<u32> {
        let offsets = vec![0x140461B20, 0x378, 0x40, 0x140, 0x110];
        let current_piece_address = DataMember::<i32>::new_offset(self.process_handle, offsets);
        let current_piece = current_piece_address.read().ok().and_then(|i| {
            if i == -1 {
                return None;
            } else {
                return Some(i as u32);
            }
        });
        return current_piece;
    }

    pub fn get_next_pieces(&self) -> std::io::Result<Vec<u32>> {
        let offsets = vec![0x140461B20, 0x378, 0xB8, 0x15C];
        let next_pieces_address = DataMember::<[u32; 5]>::new_offset(self.process_handle, offsets);
        let next_pieces = next_pieces_address.read()?.to_vec();

        return Ok(next_pieces);
    }

    pub fn get_pieces_from_bags(&self) -> std::io::Result<Vec<u32>> {
        let offsets = vec![0x140461B20, 0x378, 0x40, 0x30, 0x320];

        let next_pieces_address = DataMember::<[u32; 14]>::new_offset(self.process_handle, offsets);
        let next_pieces = next_pieces_address.read()?.to_vec();

        return Ok(next_pieces);
    }

    pub fn get_piece_count(&self) -> std::io::Result<u32> {
        let offsets = vec![0x140461B20, 0x378, 0x40, 0x30, 0x3D8];

        let hold_address = DataMember::<u32>::new_offset(self.process_handle, offsets);
        let hold = hold_address.read();

        return hold;
    }

    pub fn get_hold(&self) -> std::io::Result<u32> {
        let offsets = vec![0x140598A20, 0x38, 0x3D0, 0x8];

        let hold_address = DataMember::<u32>::new_offset(self.process_handle, offsets);
        let hold = hold_address.read();

        return hold;
    }

    pub fn get_columns(&self) -> std::io::Result<Vec<Vec<i32>>> {
        let offsets = vec![0x140461B20, 0x378, 0xC0, 0x10, 0x3C0, 0x18];

        let board_address = DataMember::<u32>::new_offset(self.process_handle, offsets);
        let mut columns_addresses = DataMember::<[u64; 10]>::new(self.process_handle);
        columns_addresses.set_offset(vec![board_address.read()? as usize]);
        let column_addrs = columns_addresses.read()?;

        let mut columns: Vec<Vec<i32>> = Vec::new();
        for column_addr in column_addrs.iter() {
            let mut pieces = DataMember::<[i32; 40]>::new(self.process_handle);
            pieces.set_offset(vec![*column_addr as usize]);
            columns.push(pieces.read()?.to_vec());
        }

        return Ok(columns);
    }
}
