#[doc = "Register `DDR_DENALI_CTL_06` reader"]
pub type R = crate::R<DdrDenaliCtl06Spec>;
#[doc = "Register `DDR_DENALI_CTL_06` writer"]
pub type W = crate::W<DdrDenaliCtl06Spec>;
#[doc = "Field `TINIT3_F0` reader - DRAM TINIT3 value for frequency copy 0 in cycles."]
pub type Tinit3F0R = crate::FieldReader<u32>;
#[doc = "Field `TINIT3_F0` writer - DRAM TINIT3 value for frequency copy 0 in cycles."]
pub type Tinit3F0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - DRAM TINIT3 value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tinit3_f0(&self) -> Tinit3F0R {
        Tinit3F0R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - DRAM TINIT3 value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tinit3_f0(&mut self) -> Tinit3F0W<DdrDenaliCtl06Spec> {
        Tinit3F0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_06::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_06::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl06Spec;
impl crate::RegisterSpec for DdrDenaliCtl06Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_06::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl06Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_06::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl06Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_06 to value 0"]
impl crate::Resettable for DdrDenaliCtl06Spec {
    const RESET_VALUE: u32 = 0;
}
