#[doc = "Register `DDR_PI_REG_87` reader"]
pub type R = crate::R<DdrPiReg87Spec>;
#[doc = "Register `DDR_PI_REG_87` writer"]
pub type W = crate::W<DdrPiReg87Spec>;
#[doc = "Field `PI_LPDDR4_RDLVL_PATTERN_10` reader - Indicates user-defined LPDDR4 read data pattern 10."]
pub type PiLpddr4RdlvlPattern10R = crate::FieldReader<u32>;
#[doc = "Field `PI_LPDDR4_RDLVL_PATTERN_10` writer - Indicates user-defined LPDDR4 read data pattern 10."]
pub type PiLpddr4RdlvlPattern10W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Indicates user-defined LPDDR4 read data pattern 10."]
    #[inline(always)]
    pub fn pi_lpddr4_rdlvl_pattern_10(&self) -> PiLpddr4RdlvlPattern10R {
        PiLpddr4RdlvlPattern10R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indicates user-defined LPDDR4 read data pattern 10."]
    #[inline(always)]
    #[must_use]
    pub fn pi_lpddr4_rdlvl_pattern_10(&mut self) -> PiLpddr4RdlvlPattern10W<DdrPiReg87Spec> {
        PiLpddr4RdlvlPattern10W::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 87\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_87::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_87::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg87Spec;
impl crate::RegisterSpec for DdrPiReg87Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_87::R`](R) reader structure"]
impl crate::Readable for DdrPiReg87Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_87::W`](W) writer structure"]
impl crate::Writable for DdrPiReg87Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_87 to value 0"]
impl crate::Resettable for DdrPiReg87Spec {
    const RESET_VALUE: u32 = 0;
}
