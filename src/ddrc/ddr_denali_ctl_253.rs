#[doc = "Register `DDR_DENALI_CTL_253` reader"]
pub type R = crate::R<DdrDenaliCtl253Spec>;
#[doc = "Register `DDR_DENALI_CTL_253` writer"]
pub type W = crate::W<DdrDenaliCtl253Spec>;
#[doc = "Field `RDLVL_GATE_NORM_THRESHOLD_F2` reader - Gate training normal threshold number of long counts until the normal priority request is asserted."]
pub type RdlvlGateNormThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_GATE_NORM_THRESHOLD_F2` writer - Gate training normal threshold number of long counts until the normal priority request is asserted."]
pub type RdlvlGateNormThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RDLVL_GATE_HIGH_THRESHOLD_F2` reader - Gate training high threshold number of long counts until the high priority request is asserted."]
pub type RdlvlGateHighThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_GATE_HIGH_THRESHOLD_F2` writer - Gate training high threshold number of long counts until the high priority request is asserted."]
pub type RdlvlGateHighThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Gate training normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    pub fn rdlvl_gate_norm_threshold_f2(&self) -> RdlvlGateNormThresholdF2R {
        RdlvlGateNormThresholdF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Gate training high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    pub fn rdlvl_gate_high_threshold_f2(&self) -> RdlvlGateHighThresholdF2R {
        RdlvlGateHighThresholdF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Gate training normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_norm_threshold_f2(
        &mut self,
    ) -> RdlvlGateNormThresholdF2W<DdrDenaliCtl253Spec> {
        RdlvlGateNormThresholdF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Gate training high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_high_threshold_f2(
        &mut self,
    ) -> RdlvlGateHighThresholdF2W<DdrDenaliCtl253Spec> {
        RdlvlGateHighThresholdF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_253::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_253::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl253Spec;
impl crate::RegisterSpec for DdrDenaliCtl253Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_253::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl253Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_253::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl253Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_253 to value 0"]
impl crate::Resettable for DdrDenaliCtl253Spec {
    const RESET_VALUE: u32 = 0;
}
