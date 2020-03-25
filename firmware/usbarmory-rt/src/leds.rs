use pac::GPIO4;

const BLUE: u32 = 1 << 22;
const WHITE: u32 = 1 << 21;

pub fn init() {
    GPIO4::borrow_unchecked(|gpio| {
        // set them as outputs
        let old = gpio.GDIR.read();
        gpio.GDIR.write(old | BLUE | WHITE);
    });
}

pub fn set(blue: bool, white: bool) {
    GPIO4::borrow_unchecked(|gpio| {
        // turn the white LED on and the blue LED off to indicate we are alive
        let mut dr = gpio.DR.read();

        if blue {
            dr &= !BLUE;
        } else {
            dr |= BLUE;
        }

        if white {
            dr &= !WHITE;
        } else {
            dr |= WHITE;
        }

        gpio.DR.write(dr);
    });
}
