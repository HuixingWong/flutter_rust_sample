


// This is the entry point of your Rust library.
// When adding new code to your project, note that only items used
// here will be transformed to their Dart equivalents.

// A plain enum without any fields. This is similar to Dart- or C-style enums.
// flutter_rust_bridge is capable of generating code for enums with fields
// (@freezed classes in Dart and tagged unions in C).
pub enum Platform {
    Unknown,
    Android,
    Ios,
    Windows,
    Unix,
    MacIntel,
    MacApple,
    Wasm,
}

// A function definition in Rust. Similar to Dart, the return type must always be named
// and is never inferred.
pub fn platform() -> Platform {
    // This is a macro, a special expression that expands into code. In Rust, all macros
    // end with an exclamation mark and can be invoked with all kinds of brackets (parentheses,
    // brackets and curly braces). However, certain conventions exist, for example the
    // vector macro is almost always invoked as vec![..].
    //
    // The cfg!() macro returns a boolean value based on the current compiler configuration.
    // When attached to expressions (#[cfg(..)] form), they show or hide the expression at compile time.
    // Here, however, they evaluate to runtime values, which may or may not be optimized out
    // by the compiler. A variety of configurations are demonstrated here which cover most of
    // the modern oeprating systems. Try running the Flutter application on different machines
    // and see if it matches your expected OS.
    //
    // Furthermore, in Rust, the last expression in a function is the return value and does
    // not have the trailing semicolon. This entire if-else chain forms a single expression.
    if cfg!(windows) {
        Platform::Windows
    } else if cfg!(target_os = "android") {
        Platform::Android
    } else if cfg!(target_os = "ios") {
        Platform::Ios
    } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        Platform::MacApple
    } else if cfg!(target_os = "macos") {
        Platform::MacIntel
    } else if cfg!(target_family = "wasm") {
        Platform::Wasm
    } else if cfg!(unix) {
        Platform::Unix
    } else {
        Platform::Unknown
    }
}

// The convention for Rust identifiers is the snake_case,
// and they are automatically converted to camelCase on the Dart side.
pub fn rust_release_mode() -> bool {
    cfg!(not(debug_assertions))
}

pub fn hello() -> String {
    print!("cao: {}", "fucking");
    "Hello from cao Rust!".to_owned()
}

pub fn draw_tree(tree: Vec<u32>) -> Vec<u32> {
    tree
}

// test quciksort
pub fn quicksort_test() {
    let mut arr = [5, 4, 3, 2, 1];
    quicksort(&mut arr);
    println!("fucking rust quick sort: {:?}", arr);
}

fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);

    let (left, right) = arr.split_at_mut(pivot_index);
    quicksort(left);
    quicksort(&mut right[1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot_index = arr.len() - 1;
    let mut i = 0;

    for j in 0..pivot_index {
        if arr[j] <= arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot_index);
    i
}

pub fn fucking() {
    print!("fucking: {}", "123")
}



// this is avif code

use flutter_rust_bridge::ZeroCopyBuffer;
use std::collections::HashMap;
use std::slice;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::RwLock;
use std::thread;

lazy_static::lazy_static! {
    static ref DECODERS: RwLock<HashMap<String, Decoder>> = {
        RwLock::new(HashMap::new())
    };
}

pub fn init_memory_decoder(key: String, avif_bytes: Vec<u8>) -> AvifInfo {
    {
        let map = DECODERS.read().unwrap();
        if map.contains_key(&key) {
            let decoder = &map[&key];
            return decoder.info;
        }
    }

    let (decoder_request_tx, decoder_request_rx): (
        Sender<DecoderCommand>,
        Receiver<DecoderCommand>,
    ) = mpsc::channel();
    let (decoder_response_tx, decoder_response_rx): (
        Sender<CodecResponse>,
        Receiver<CodecResponse>,
    ) = mpsc::channel();
    let (decoder_info_tx, decoder_info_rx): (Sender<AvifInfo>, Receiver<AvifInfo>) =
        mpsc::channel();

    thread::spawn(move || unsafe {
        let decoder = libavif_sys::avifDecoderCreate();
        let image = libavif_sys::avifImageCreateEmpty();
        let read_memory_result = libavif_sys::avifDecoderReadMemory(
            decoder,
            image,
            avif_bytes.as_ptr(),
            avif_bytes.len(),
        );
        if !(read_memory_result == libavif_sys::AVIF_RESULT_OK
            || read_memory_result == libavif_sys::AVIF_RESULT_BMFF_PARSE_FAILED)
        {
            libavif_sys::avifDecoderDestroy(decoder);
            panic!("Couldn't decode the image. Code: {}", read_memory_result);
        }

        match decoder_info_tx.send(AvifInfo {
            width: 0,
            height: 0,
            duration: (*decoder).duration,
            image_count: (*decoder).imageCount as u32,
        }) {
            Ok(result) => result,
            Err(e) => panic!("Decoder connection lost. {}", e),
        };

        loop {
            let request = decoder_request_rx.recv().unwrap();
            let response = match request {
                DecoderCommand::GetNextFrame => _get_next_frame(decoder),
                DecoderCommand::Reset => _reset_decoder(decoder),
                DecoderCommand::Dispose => _dispose_decoder(decoder),
            };
            match decoder_response_tx.send(response) {
                Ok(result) => result,
                Err(e) => panic!("Decoder connection lost. {}", e),
            };

            match request {
                DecoderCommand::Dispose => break,
                _ => {}
            };
        }
    });

    let avif_info = match decoder_info_rx.recv() {
        Ok(result) => result,
        Err(e) => panic!("Couldn't read avi info. Code: {}", e),
    };

    {
        let mut map = DECODERS.write().unwrap();
        map.insert(
            key,
            Decoder {
                request_tx: decoder_request_tx,
                response_rx: decoder_response_rx,
                info: avif_info,
            },
        );
    }
    return avif_info;
}

pub fn reset_decoder(key: String) -> bool {
    let map = DECODERS.read().unwrap();
    if !map.contains_key(&key) {
        return false;
    }

    let decoder = &map[&key];
    match decoder.request_tx.send(DecoderCommand::Reset) {
        Ok(result) => result,
        Err(e) => panic!("Decoder connection lost. {}", e),
    };
    decoder.response_rx.recv().unwrap();
    return true;
}

pub fn dispose_decoder(key: String) -> bool {
    let mut map = DECODERS.write().unwrap();
    if !map.contains_key(&key) {
        return false;
    }

    let decoder = &map[&key];
    match decoder.request_tx.send(DecoderCommand::Dispose) {
        Ok(result) => result,
        Err(e) => panic!("Decoder connection lost. {}", e),
    };
    decoder.response_rx.recv().unwrap();
    map.remove(&key);
    return true;
}

pub fn get_next_frame(key: String) -> Frame {
    let map = DECODERS.read().unwrap();
    if !map.contains_key(&key) {
        panic!("Decoder not found. {}", key);
    }

    let decoder = &map[&key];
    match decoder.request_tx.send(DecoderCommand::GetNextFrame) {
        Ok(result) => result,
        Err(e) => panic!("Decoder connection lost. {}", e),
    };
    let result = decoder.response_rx.recv().unwrap();
    return result.frame;
}

pub fn encode_avif(
    width: u32,
    height: u32,
    speed: i32,
    max_threads: i32,
    timescale: u64,
    max_quantizer: i32,
    min_quantizer: i32,
    max_quantizer_alpha: i32,
    min_quantizer_alpha: i32,
    image_sequence: Vec<EncodeFrame>,
) -> ZeroCopyBuffer<Vec<u8>> {
    unsafe {
        let encoder = libavif_sys::avifEncoderCreate();
        (*encoder).maxThreads = max_threads;
        (*encoder).speed = speed;
        (*encoder).timescale = timescale;
        (*encoder).minQuantizer = min_quantizer;
        (*encoder).maxQuantizer = max_quantizer;
        (*encoder).minQuantizerAlpha = min_quantizer_alpha;
        (*encoder).maxQuantizerAlpha = max_quantizer_alpha;

        for frame in image_sequence.iter() {
            let image = libavif_sys::avifImageCreate(
                width,
                height,
                8,
                libavif_sys::AVIF_PIXEL_FORMAT_YUV444,
            );
            libavif_sys::avifImageAllocatePlanes(image, libavif_sys::AVIF_PLANES_YUV);

            let mut rgb = libavif_sys::avifRGBImage::default();
            let raw_rgb = &mut rgb as *mut libavif_sys::avifRGBImage;
            libavif_sys::avifRGBImageSetDefaults(raw_rgb, image);
            rgb.format = libavif_sys::AVIF_RGB_FORMAT_RGBA;
            rgb.depth = 8;
            libavif_sys::avifRGBImageAllocatePixels(raw_rgb);
            std::ptr::copy(
                frame.data.as_ptr(),
                rgb.pixels,
                (rgb.rowBytes * (*image).height) as usize,
            );

            let conversion_result = libavif_sys::avifImageRGBToYUV(image, &rgb);
            if conversion_result != libavif_sys::AVIF_RESULT_OK {
                libavif_sys::avifImageDestroy(image);
                libavif_sys::avifEncoderDestroy(encoder);
                libavif_sys::avifRGBImageFreePixels(raw_rgb);
                panic!("yuv_to_rgb error {}", conversion_result);
            }
            let add_result = libavif_sys::avifEncoderAddImage(
                encoder,
                image,
                frame.duration_in_timescale,
                libavif_sys::AVIF_ADD_IMAGE_FLAG_NONE,
            );
            if add_result != libavif_sys::AVIF_RESULT_OK {
                libavif_sys::avifImageDestroy(image);
                libavif_sys::avifEncoderDestroy(encoder);
                libavif_sys::avifRGBImageFreePixels(raw_rgb);
                panic!("add_image error {}", add_result);
            }
            libavif_sys::avifImageDestroy(image);
            libavif_sys::avifRGBImageFreePixels(raw_rgb);
        }

        let mut s = ::std::mem::MaybeUninit::<u8>::uninit();
        let mut avif_output = libavif_sys::avifRWData {
            data: s.as_mut_ptr(),
            size: 0,
        };
        let raw_avif_output = &mut avif_output as *mut libavif_sys::avifRWData;
        let finish_result = libavif_sys::avifEncoderFinish(encoder, raw_avif_output);
        if finish_result != libavif_sys::AVIF_RESULT_OK {
            libavif_sys::avifRWDataFree(raw_avif_output);
            libavif_sys::avifEncoderDestroy(encoder);
            panic!("avif_output error {}", finish_result);
        }
        let output_data = slice::from_raw_parts(avif_output.data, avif_output.size).to_vec();
        libavif_sys::avifRWDataFree(raw_avif_output);
        libavif_sys::avifEncoderDestroy(encoder);
        return ZeroCopyBuffer(output_data);
    }
}

fn _dispose_decoder(decoder: *mut libavif_sys::avifDecoder) -> CodecResponse {
    unsafe {
        libavif_sys::avifDecoderDestroy(decoder);
        return CodecResponse {
            command: DecoderCommand::Dispose,
            frame: Frame {
                data: ZeroCopyBuffer(Vec::new()),
                duration: 0.0,
                width: 0,
                height: 0,
            },
        };
    }
}

fn _reset_decoder(decoder: *mut libavif_sys::avifDecoder) -> CodecResponse {
    unsafe {
        libavif_sys::avifDecoderReset(decoder);
        return CodecResponse {
            command: DecoderCommand::Reset,
            frame: Frame {
                data: ZeroCopyBuffer(Vec::new()),
                duration: 0.0,
                width: 0,
                height: 0,
            },
        };
    }
}

fn _get_next_frame(decoder: *mut libavif_sys::avifDecoder) -> CodecResponse {
    unsafe {
        let mut decode_result = libavif_sys::avifDecoderNextImage(decoder);
        if decode_result == libavif_sys::AVIF_RESULT_NO_IMAGES_REMAINING {
            libavif_sys::avifDecoderReset(decoder);
            decode_result = libavif_sys::avifDecoderNextImage(decoder);
        }

        if decode_result != libavif_sys::AVIF_RESULT_OK {
            panic!("decode error {}", decode_result);
        }

        let mut rgb = libavif_sys::avifRGBImage::default();
        let raw_rgb = &mut rgb as *mut libavif_sys::avifRGBImage;

        libavif_sys::avifRGBImageSetDefaults(raw_rgb, (*decoder).image);
        rgb.format = libavif_sys::AVIF_RGB_FORMAT_RGBA;
        rgb.depth = 8;
        libavif_sys::avifRGBImageAllocatePixels(raw_rgb);
        let conversion_result = libavif_sys::avifImageYUVToRGB((*decoder).image, raw_rgb);
        if conversion_result != libavif_sys::AVIF_RESULT_OK {
            panic!("yuv_to_rgb error {}", conversion_result);
        }

        let size = rgb.rowBytes * (*(*decoder).image).height;
        let data = ZeroCopyBuffer(slice::from_raw_parts(rgb.pixels, size as usize).to_vec());
        libavif_sys::avifRGBImageFreePixels(raw_rgb);
        return CodecResponse {
            command: DecoderCommand::GetNextFrame,
            frame: Frame {
                data: data,
                duration: (*decoder).imageTiming.duration,
                width: (*(*decoder).image).width,
                height: (*(*decoder).image).height,
            },
        };
    }
}

#[derive(Copy, Clone)]
pub struct AvifInfo {
    pub width: u32,
    pub height: u32,
    pub image_count: u32,
    pub duration: f64,
}

pub struct Frame {
    pub data: ZeroCopyBuffer<Vec<u8>>,
    pub duration: f64,
    pub width: u32,
    pub height: u32,
}

pub struct EncodeFrame {
    pub data: Vec<u8>,
    pub duration_in_timescale: u64,
}

struct Decoder {
    request_tx: Sender<DecoderCommand>,
    response_rx: Receiver<CodecResponse>,
    info: AvifInfo,
}

unsafe impl Send for Decoder {}
unsafe impl Sync for Decoder {}

enum DecoderCommand {
    GetNextFrame,
    Reset,
    Dispose,
}

struct CodecResponse {
    pub command: DecoderCommand,
    pub frame: Frame,
}
