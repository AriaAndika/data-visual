# Data Visual with Web Assembly and Canvas API


# Dev Docs

to compile use `wasm-pack build`

## Wasm Basic

> wasm can send a data in `number type` from js to wasm to js that work out of the box
> 
> wasm can have a linear memory that can be accessed in js

`lib.rs`
```rs
// export a function
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
	a + b
}

// create linear memory
const MEM_SIZE: usize = 100;
static mut MEM: [u8;MEM_SIZE] = [1;MEM_SIZE];

// get the memory pointer
#[wasm_bindgen]
pub fn get_ptr() -> *const u8 {
    let ptr;
    unsafe { ptr = MEM.as_ptr(); }
    ptr
}
```

`dist.js`
```js
// Initiate wasm
const wasm = await WebAssembly.instantiateStreaming(fetch("./data_visual_bg.wasm"))

export const { 
	// linear memory buffer
	memory: { buffer },	
	// extract the exported rust function
	get_ptr, add 
} = wasm.instance.exports

// initiate the linear memory
export const ptr = get_ptr()
export const wasmMemory = new Uint8Array(buffer)

// then use it as regular array with the pointer index
let memoryValue = wasmMemory[ptr]
wasmMemory[ptr + 1] = 8
```

The memory doesn`t have to be u8:
| rust  | javascript |
| ------------- | ------------- |
| u8  | Uint8Array  |
| u16  | Uint16Array  |
| i32  | Int32Array  |

