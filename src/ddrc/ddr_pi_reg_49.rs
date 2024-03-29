#[doc = "Register `DDR_PI_REG_49` reader"]
pub type R = crate::R<DdrPiReg49Spec>;
#[doc = "Register `DDR_PI_REG_49` writer"]
pub type W = crate::W<DdrPiReg49Spec>;
#[doc = "Field `PI_TREF_INTERVAL` reader - Defines the cycles between refreshes to different chip selects."]
pub type PiTrefIntervalR = crate::FieldReader<u16>;
#[doc = "Field `PI_TREF_INTERVAL` writer - Defines the cycles between refreshes to different chip selects."]
pub type PiTrefIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PI_SWLVL_LOAD` writer - Indicates user request to load delays and execute software\n\nleveling. Set to 1 to trigger."]
pub type PiSwlvlLoadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:23 - Defines the cycles between refreshes to different chip selects."]
    #[inline(always)]
    pub fn pi_tref_interval(&self) -> PiTrefIntervalR {
        PiTrefIntervalR::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:23 - Defines the cycles between refreshes to different chip selects."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tref_interval(&mut self) -> PiTrefIntervalW<DdrPiReg49Spec> {
        PiTrefIntervalW::new(self, 8)
    }
    #[doc = "Bit 24 - Indicates user request to load delays and execute software\n\nleveling. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_load(&mut self) -> PiSwlvlLoadW<DdrPiReg49Spec> {
        PiSwlvlLoadW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 49\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_49::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_49::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg49Spec;
impl crate::RegisterSpec for DdrPiReg49Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_49::R`](R) reader structure"]
impl crate::Readable for DdrPiReg49Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_49::W`](W) writer structure"]
impl crate::Writable for DdrPiReg49Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_49 to value 0"]
impl crate::Resettable for DdrPiReg49Spec {
    const RESET_VALUE: u32 = 0;
}
