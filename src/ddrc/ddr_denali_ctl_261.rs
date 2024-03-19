#[doc = "Register `DDR_DENALI_CTL_261` reader"]
pub type R = crate::R<DdrDenaliCtl261Spec>;
#[doc = "Register `DDR_DENALI_CTL_261` writer"]
pub type W = crate::W<DdrDenaliCtl261Spec>;
#[doc = "Field `CALVL_BG_PAT_2` reader - CA Training pattern 2 driven on the CA bus before and after a calibration command."]
pub type CalvlBgPat2R = crate::FieldReader<u32>;
#[doc = "Field `CALVL_BG_PAT_2` writer - CA Training pattern 2 driven on the CA bus before and after a calibration command."]
pub type CalvlBgPat2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - CA Training pattern 2 driven on the CA bus before and after a calibration command."]
    #[inline(always)]
    pub fn calvl_bg_pat_2(&self) -> CalvlBgPat2R {
        CalvlBgPat2R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - CA Training pattern 2 driven on the CA bus before and after a calibration command."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_bg_pat_2(&mut self) -> CalvlBgPat2W<DdrDenaliCtl261Spec> {
        CalvlBgPat2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_261::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_261::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl261Spec;
impl crate::RegisterSpec for DdrDenaliCtl261Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_261::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl261Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_261::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl261Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_261 to value 0"]
impl crate::Resettable for DdrDenaliCtl261Spec {
    const RESET_VALUE: u32 = 0;
}
