#[doc = "Register `PI_REG_11` reader"]
pub type R = crate::R<PiReg11Spec>;
#[doc = "Register `PI_REG_11` writer"]
pub type W = crate::W<PiReg11Spec>;
#[doc = "Field `PI_TDFI_PHYUPD_TYPE3_F0` reader - Defines the DFI tPHYUPD_TYPE3 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 3. If programmed to a non-zero, a timing violation causes an interrupt and bit7 set in the PI_REG_193.pi_update_error_status parameter and bit7 set in the PI_REG_22.pi_control_error_status parameter. The suffix f0 of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhyupdType3F0R = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_PHYUPD_TYPE3_F0` writer - Defines the DFI tPHYUPD_TYPE3 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 3. If programmed to a non-zero, a timing violation causes an interrupt and bit7 set in the PI_REG_193.pi_update_error_status parameter and bit7 set in the PI_REG_22.pi_control_error_status parameter. The suffix f0 of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhyupdType3F0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE3 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 3. If programmed to a non-zero, a timing violation causes an interrupt and bit7 set in the PI_REG_193.pi_update_error_status parameter and bit7 set in the PI_REG_22.pi_control_error_status parameter. The suffix f0 of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_phyupd_type3_f0(&self) -> PiTdfiPhyupdType3F0R {
        PiTdfiPhyupdType3F0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE3 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 3. If programmed to a non-zero, a timing violation causes an interrupt and bit7 set in the PI_REG_193.pi_update_error_status parameter and bit7 set in the PI_REG_22.pi_control_error_status parameter. The suffix f0 of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_phyupd_type3_f0(&mut self) -> PiTdfiPhyupdType3F0W<PiReg11Spec> {
        PiTdfiPhyupdType3F0W::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg11Spec;
impl crate::RegisterSpec for PiReg11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_11::R`](R) reader structure"]
impl crate::Readable for PiReg11Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_11::W`](W) writer structure"]
impl crate::Writable for PiReg11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_11 to value 0"]
impl crate::Resettable for PiReg11Spec {
    const RESET_VALUE: u32 = 0;
}