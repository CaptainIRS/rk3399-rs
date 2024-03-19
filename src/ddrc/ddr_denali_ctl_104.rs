#[doc = "Register `DDR_DENALI_CTL_104` reader"]
pub type R = crate::R<DdrDenaliCtl104Spec>;
#[doc = "Register `DDR_DENALI_CTL_104` writer"]
pub type W = crate::W<DdrDenaliCtl104Spec>;
#[doc = "Field `HW_PROMOTE_THRESHOLD_F1` reader - HW interface promotion number of long counts until the high priority request is asserted for frequency copy 1."]
pub type HwPromoteThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `HW_PROMOTE_THRESHOLD_F1` writer - HW interface promotion number of long counts until the high priority request is asserted for frequency copy 1."]
pub type HwPromoteThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HW_PROMOTE_THRESHOLD_F2` reader - HW interface promotion number of long counts until the high priority request is asserted for frequency copy 2."]
pub type HwPromoteThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `HW_PROMOTE_THRESHOLD_F2` writer - HW interface promotion number of long counts until the high priority request is asserted for frequency copy 2."]
pub type HwPromoteThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - HW interface promotion number of long counts until the high priority request is asserted for frequency copy 1."]
    #[inline(always)]
    pub fn hw_promote_threshold_f1(&self) -> HwPromoteThresholdF1R {
        HwPromoteThresholdF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - HW interface promotion number of long counts until the high priority request is asserted for frequency copy 2."]
    #[inline(always)]
    pub fn hw_promote_threshold_f2(&self) -> HwPromoteThresholdF2R {
        HwPromoteThresholdF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - HW interface promotion number of long counts until the high priority request is asserted for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn hw_promote_threshold_f1(&mut self) -> HwPromoteThresholdF1W<DdrDenaliCtl104Spec> {
        HwPromoteThresholdF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - HW interface promotion number of long counts until the high priority request is asserted for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn hw_promote_threshold_f2(&mut self) -> HwPromoteThresholdF2W<DdrDenaliCtl104Spec> {
        HwPromoteThresholdF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_104::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_104::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl104Spec;
impl crate::RegisterSpec for DdrDenaliCtl104Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_104::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl104Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_104::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl104Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_104 to value 0"]
impl crate::Resettable for DdrDenaliCtl104Spec {
    const RESET_VALUE: u32 = 0;
}
