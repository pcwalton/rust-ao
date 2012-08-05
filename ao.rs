enum ByteFormat {
    Little = 1,
    Big    = 2,
    Native = 4
}

enum ao_device {}

struct ao_option {
    key: *u8;
    value: *u8;
    next: *ao_option;
}

struct SampleFormat {
    bits: i32;
    rate: i32;
    channels: i32;
    byte_format: i32;
    matrix: *u8;
}

fn SampleFormat(bits: i32, rate: i32, channels: i32, byte_format: ByteFormat)
             -> SampleFormat {
    SampleFormat {
        bits: bits,
        rate: rate,
        channels: channels,
        byte_format: byte_format as i32,
        matrix: ptr::null()
    }
}

struct AO {
    // FIXME: This is silly.
    x: int;

    drop {
        bindings::ao_shutdown();
    }
}

fn AO() -> AO {
    bindings::ao_initialize();
    return AO { x: 0 };
}

impl AO {
    fn open_live(driver_id: i32, sample_format: &SampleFormat) -> Device {
        let ao_device = bindings::ao_open_live(driver_id,
                                               ptr::addr_of(*sample_format),
                                               ptr::null());
        Device { ao_device: ao_device }
    }

    fn default_driver_id() -> i32 {
        bindings::ao_default_driver_id()
    }
}

struct Device {
    ao_device: *ao_device;

    drop {
        bindings::ao_close(self.ao_device);
    }
}

impl Device {
    // FIXME: This isn't particularly type-safe.
    fn play<T>(output_samples: &[const T]) -> i32 unsafe {
        bindings::ao_play(self.ao_device,
                          unsafe::reinterpret_cast
                                (vec::unsafe::to_ptr_slice(output_samples)),
                          (sys::size_of::<T>() * output_samples.len()) as u32)
    }
}

#[link_name="ao"]
extern mod bindings {
    fn ao_initialize();
    fn ao_shutdown();

    fn ao_open_live(driver_id: i32, format: *SampleFormat, options: *ao_option)
                 -> *ao_device;
    fn ao_play(device: *ao_device, output_samples: *u8, num_bytes: u32) -> i32;
    fn ao_close(device: *ao_device) -> i32;

    fn ao_default_driver_id() -> i32;
}

