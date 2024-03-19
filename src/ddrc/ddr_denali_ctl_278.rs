#[doc = "Register `DDR_DENALI_CTL_278` reader"]
pub type R = crate::R<DdrDenaliCtl278Spec>;
#[doc = "Register `DDR_DENALI_CTL_278` writer"]
pub type W = crate::W<DdrDenaliCtl278Spec>;
#[doc = "Field `TDFI_PHYUPD_TYPE0_F0` reader - Defines the DFI tPHYUPD_TYPE0 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit (2) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiPhyupdType0F0R = crate::FieldReader<u32>;
#[doc = "Field `TDFI_PHYUPD_TYPE0_F0` writer - Defines the DFI tPHYUPD_TYPE0 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit (2) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiPhyupdType0F0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE0 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit (2) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn tdfi_phyupd_type0_f0(&self) -> TdfiPhyupdType0F0R {
        TdfiPhyupdType0F0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE0 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit (2) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phyupd_type0_f0(&mut self) -> TdfiPhyupdType0F0W<DdrDenaliCtl278Spec> {
        TdfiPhyupdType0F0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_278::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_278::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl278Spec;
impl crate::RegisterSpec for DdrDenaliCtl278Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_278::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl278Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_278::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl278Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_278 to value 0"]
impl crate::Resettable for DdrDenaliCtl278Spec {
    const RESET_VALUE: u32 = 0;
}
