use pac::SNVS_HP;

pub fn init() {
    /// HP Real-Time Counter Enable
    const SNVS_HP_CR_RTC_EN: u32 = 1;

    SNVS_HP::borrow_unchecked(|snvs| {
        snvs.CR.write(SNVS_HP_CR_RTC_EN);
    });
}

pub fn now() -> u32 {
    SNVS_HP::borrow_unchecked(|snvs| snvs.RTCLR.read())
}
