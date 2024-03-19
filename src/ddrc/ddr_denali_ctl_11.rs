#[doc = "Register `DDR_DENALI_CTL_11` reader"]
pub type R = crate::R<DdrDenaliCtl11Spec>;
#[doc = "Register `DDR_DENALI_CTL_11` writer"]
pub type W = crate::W<DdrDenaliCtl11Spec>;
#[doc = "Field `TINIT4_F1` reader - DRAM TINIT4 value for frequency copy 1 in cycles."]
pub type Tinit4F1R = crate::FieldReader<u32>;
#[doc = "Field `TINIT4_F1` writer - DRAM TINIT4 value for frequency copy 1 in cycles."]
pub type Tinit4F1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - DRAM TINIT4 value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tinit4_f1(&self) -> Tinit4F1R {
        Tinit4F1R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - DRAM TINIT4 value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tinit4_f1(&mut self) -> Tinit4F1W<DdrDenaliCtl11Spec> {
        Tinit4F1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl11Spec;
impl crate::RegisterSpec for DdrDenaliCtl11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_11::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl11Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_11::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_11 to value 0"]
impl crate::Resettable for DdrDenaliCtl11Spec {
    const RESET_VALUE: u32 = 0;
}
