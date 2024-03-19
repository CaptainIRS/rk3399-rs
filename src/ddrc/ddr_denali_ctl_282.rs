#[doc = "Register `DDR_DENALI_CTL_282` reader"]
pub type R = crate::R<DdrDenaliCtl282Spec>;
#[doc = "Register `DDR_DENALI_CTL_282` writer"]
pub type W = crate::W<DdrDenaliCtl282Spec>;
#[doc = "Field `TDFI_PHYUPD_RESP_F0` reader - Defines the DFI tPHYUPD_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phyupd_req assertion and a dfi_phyupd_ack assertion. If programmed to a non-zero, a timing violation will cause an interrupt and bit (6) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiPhyupdRespF0R = crate::FieldReader<u16>;
#[doc = "Field `TDFI_PHYUPD_RESP_F0` writer - Defines the DFI tPHYUPD_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phyupd_req assertion and a dfi_phyupd_ack assertion. If programmed to a non-zero, a timing violation will cause an interrupt and bit (6) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiPhyupdRespF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Defines the DFI tPHYUPD_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phyupd_req assertion and a dfi_phyupd_ack assertion. If programmed to a non-zero, a timing violation will cause an interrupt and bit (6) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn tdfi_phyupd_resp_f0(&self) -> TdfiPhyupdRespF0R {
        TdfiPhyupdRespF0R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the DFI tPHYUPD_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phyupd_req assertion and a dfi_phyupd_ack assertion. If programmed to a non-zero, a timing violation will cause an interrupt and bit (6) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phyupd_resp_f0(&mut self) -> TdfiPhyupdRespF0W<DdrDenaliCtl282Spec> {
        TdfiPhyupdRespF0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_282::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_282::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl282Spec;
impl crate::RegisterSpec for DdrDenaliCtl282Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_282::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl282Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_282::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl282Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_282 to value 0"]
impl crate::Resettable for DdrDenaliCtl282Spec {
    const RESET_VALUE: u32 = 0;
}
