#[doc = "Register `DDR_DENALI_CTL_297` reader"]
pub type R = crate::R<DdrDenaliCtl297Spec>;
#[doc = "Register `DDR_DENALI_CTL_297` writer"]
pub type W = crate::W<DdrDenaliCtl297Spec>;
#[doc = "Field `TDFI_CTRLUPD_INTERVAL_F2` reader - Defines the DFI tCTRLUPD_INTERVAL timing parameter (in DFI clocks), the maximum cycles between dfi_ctrlupd_req assertions. If programmed to a non-zero, a timing violation will cause an interrupt and bit (0) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiCtrlupdIntervalF2R = crate::FieldReader<u32>;
#[doc = "Field `TDFI_CTRLUPD_INTERVAL_F2` writer - Defines the DFI tCTRLUPD_INTERVAL timing parameter (in DFI clocks), the maximum cycles between dfi_ctrlupd_req assertions. If programmed to a non-zero, a timing violation will cause an interrupt and bit (0) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiCtrlupdIntervalF2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tCTRLUPD_INTERVAL timing parameter (in DFI clocks), the maximum cycles between dfi_ctrlupd_req assertions. If programmed to a non-zero, a timing violation will cause an interrupt and bit (0) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn tdfi_ctrlupd_interval_f2(&self) -> TdfiCtrlupdIntervalF2R {
        TdfiCtrlupdIntervalF2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tCTRLUPD_INTERVAL timing parameter (in DFI clocks), the maximum cycles between dfi_ctrlupd_req assertions. If programmed to a non-zero, a timing violation will cause an interrupt and bit (0) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_ctrlupd_interval_f2(&mut self) -> TdfiCtrlupdIntervalF2W<DdrDenaliCtl297Spec> {
        TdfiCtrlupdIntervalF2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_297::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_297::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl297Spec;
impl crate::RegisterSpec for DdrDenaliCtl297Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_297::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl297Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_297::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl297Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_297 to value 0"]
impl crate::Resettable for DdrDenaliCtl297Spec {
    const RESET_VALUE: u32 = 0;
}
