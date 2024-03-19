#[doc = "Register `DDR_DENALI_CTL_268` reader"]
pub type R = crate::R<DdrDenaliCtl268Spec>;
#[doc = "Register `DDR_DENALI_CTL_268` writer"]
pub type W = crate::W<DdrDenaliCtl268Spec>;
#[doc = "Field `CALVL_DFI_PROMOTE_THRESHOLD_F0` reader - CA training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type CalvlDfiPromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `CALVL_DFI_PROMOTE_THRESHOLD_F0` writer - CA training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type CalvlDfiPromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CALVL_NORM_THRESHOLD_F1` reader - CA training normal threshold number of long counts until the normal priority request is asserted."]
pub type CalvlNormThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `CALVL_NORM_THRESHOLD_F1` writer - CA training normal threshold number of long counts until the normal priority request is asserted."]
pub type CalvlNormThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CA training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    pub fn calvl_dfi_promote_threshold_f0(&self) -> CalvlDfiPromoteThresholdF0R {
        CalvlDfiPromoteThresholdF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CA training normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    pub fn calvl_norm_threshold_f1(&self) -> CalvlNormThresholdF1R {
        CalvlNormThresholdF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CA training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_dfi_promote_threshold_f0(
        &mut self,
    ) -> CalvlDfiPromoteThresholdF0W<DdrDenaliCtl268Spec> {
        CalvlDfiPromoteThresholdF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - CA training normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_norm_threshold_f1(&mut self) -> CalvlNormThresholdF1W<DdrDenaliCtl268Spec> {
        CalvlNormThresholdF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_268::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_268::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl268Spec;
impl crate::RegisterSpec for DdrDenaliCtl268Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_268::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl268Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_268::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl268Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_268 to value 0"]
impl crate::Resettable for DdrDenaliCtl268Spec {
    const RESET_VALUE: u32 = 0;
}
