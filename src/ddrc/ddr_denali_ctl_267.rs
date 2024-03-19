#[doc = "Register `DDR_DENALI_CTL_267` reader"]
pub type R = crate::R<DdrDenaliCtl267Spec>;
#[doc = "Register `DDR_DENALI_CTL_267` writer"]
pub type W = crate::W<DdrDenaliCtl267Spec>;
#[doc = "Field `CALVL_TIMEOUT_F0` reader - CA training timeout number of long counts until the timeout is asserted."]
pub type CalvlTimeoutF0R = crate::FieldReader<u16>;
#[doc = "Field `CALVL_TIMEOUT_F0` writer - CA training timeout number of long counts until the timeout is asserted."]
pub type CalvlTimeoutF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CALVL_SW_PROMOTE_THRESHOLD_F0` reader - CA training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type CalvlSwPromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `CALVL_SW_PROMOTE_THRESHOLD_F0` writer - CA training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type CalvlSwPromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CA training timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    pub fn calvl_timeout_f0(&self) -> CalvlTimeoutF0R {
        CalvlTimeoutF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CA training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    pub fn calvl_sw_promote_threshold_f0(&self) -> CalvlSwPromoteThresholdF0R {
        CalvlSwPromoteThresholdF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CA training timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_timeout_f0(&mut self) -> CalvlTimeoutF0W<DdrDenaliCtl267Spec> {
        CalvlTimeoutF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - CA training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_sw_promote_threshold_f0(
        &mut self,
    ) -> CalvlSwPromoteThresholdF0W<DdrDenaliCtl267Spec> {
        CalvlSwPromoteThresholdF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_267::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_267::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl267Spec;
impl crate::RegisterSpec for DdrDenaliCtl267Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_267::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl267Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_267::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl267Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_267 to value 0"]
impl crate::Resettable for DdrDenaliCtl267Spec {
    const RESET_VALUE: u32 = 0;
}
