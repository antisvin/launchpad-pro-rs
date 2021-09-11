use super::diamond::Tone;

pub const TONES: [[Tone; 8]; 8] = [
    [Tone::new(1, 0, 0x0), Tone::new(9, 2, 0xa02756f), Tone::new(5, 3, 0xdcf68e36), Tone::new(11, 5, 0x835fba09), Tone::new(3, 7, 0x5013ab7), Tone::new(13, 8, 0x67c0355a), Tone::new(7, 9, 0xb031befe), Tone::new(15, 10, 0xe1f7c8ee)],
    [Tone::new(9, 9, 0xf5fd8a90), Tone::new(1, 0, 0x0), Tone::new(9, 1, 0xd2f418c6), Tone::new(11, 3, 0x795d4499), Tone::new(3, 4, 0xfafec548), Tone::new(13, 6, 0x5dbdbfeb), Tone::new(9, 7, 0xa62f498e), Tone::new(5, 8, 0xd7f5537e)],
    [Tone::new(5, 8, 0x230971c9), Tone::new(9, 10, 0x2d0be739), Tone::new(1, 0, 0x0), Tone::new(11, 1, 0xa6692bd3), Tone::new(5, 3, 0x280aac81), Tone::new(13, 4, 0x8ac9a724), Tone::new(7, 5, 0xd33b30c7), Tone::new(3, 7, 0x5013ab7)],
    [Tone::new(11, 6, 0x7ca045f6), Tone::new(11, 8, 0x86a2bb66), Tone::new(11, 10, 0x5996d42c), Tone::new(1, 0, 0x0), Tone::new(11, 1, 0x81a180ae), Tone::new(13, 2, 0xe4607b51), Tone::new(11, 4, 0x2cd204f4), Tone::new(15, 5, 0x5e980ee4)],
    [Tone::new(3, 4, 0xfafec548), Tone::new(3, 7, 0x5013ab7), Tone::new(5, 8, 0xd7f5537e), Tone::new(11, 10, 0x7e5e7f51), Tone::new(1, 0, 0x0), Tone::new(13, 1, 0x62befaa3), Tone::new(7, 2, 0xab308446), Tone::new(5, 3, 0xdcf68e36)],
    [Tone::new(13, 3, 0x983fcaa5), Tone::new(13, 5, 0xa2424014), Tone::new(13, 7, 0x753658db), Tone::new(13, 9, 0x1b9f84ae), Tone::new(13, 10, 0x9d41055c), Tone::new(1, 0, 0x0), Tone::new(13, 1, 0x487189a3), Tone::new(15, 2, 0x7a379393)],
    [Tone::new(7, 2, 0x4fce4101), Tone::new(9, 4, 0x59d0b671), Tone::new(7, 6, 0x2cc4cf38), Tone::new(11, 7, 0xd32dfb0b), Tone::new(7, 9, 0x54cf7bb9), Tone::new(13, 10, 0xb78e765c), Tone::new(1, 0, 0x0), Tone::new(15, 1, 0x31c609f0)],
    [Tone::new(15, 1, 0x1e083711), Tone::new(5, 3, 0x280aac81), Tone::new(3, 4, 0xfafec548), Tone::new(15, 6, 0xa167f11b), Tone::new(5, 8, 0x230971c9), Tone::new(15, 9, 0x85c86c6c), Tone::new(15, 10, 0xce39f60f), Tone::new(1, 0, 0x0)]
];
