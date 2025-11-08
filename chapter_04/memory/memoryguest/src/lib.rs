const MEMORY_BUFFER_SIZE: usize = 50;
static mut BUFFER: [u8; MEMORY_BUFFER_SIZE] = [0; MEMORY_BUFFER_SIZE];


#[no_mangle]
pub extern fn get_buffer_ptr() -> *const u8 {    
    let pointer: *const u8 = {
        unsafe {
            BUFFER.as_ptr()
        }
    };
    pointer
}

#[no_mangle]
pub extern fn set_name(len: i32) -> i32 {    
    let name = unsafe {
        std::str::from_utf8(&BUFFER[..len as usize]).unwrap()
    };

    let greeting = format!("Hello, {}", name);
    // Rust has a number of ways to do this more efficiently...
    // done "long hand" to illustrate what's happening
    unsafe {
        for (idx , byte) in greeting.as_bytes().iter().enumerate() {
            BUFFER[idx] = *byte;
        }
    }
    
    greeting.len() as i32
}

//for addon lab
//example

// const MEMORY_BUFFER_SIZE: usize = 50;
//const GREETINGS: [&str; 2] = ["Hello", "Ahoy"];
// static mut BUFFER: [u8; MEMORY_BUFFER_SIZE] = [0; MEMORY_BUFFER_SIZE];

// #[no_mangle]
// pub extern fn get_buffer_ptr() -> *const u8 {    
//     let pointer: *const u8 = {
//         unsafe {
//             BUFFER.as_ptr()
//         }
//     };
//     pointer
// }

// #[no_mangle]
// pub extern fn set_name(greeting_idx: i32, len: i32) -> i32 {
//     let greeting = GREETINGS.get(greeting_idx as usize).unwrap_or(&"Hello");

//     let name = unsafe {
//         std::str::from_utf8(&BUFFER[..len as usize]).unwrap()
//     };

//     let greeting = format!("{}, {}", greeting, name);

//     unsafe {
//         for (idx, byte) in greeting.as_bytes().iter().enumerate() {
//             BUFFER[idx] = *byte;
//         }
//     }

//     greeting.len() as i32
// }
