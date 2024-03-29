#[doc = "Register `DDR_PI_REG_193` reader"]
pub type R = crate::R<DdrPiReg193Spec>;
#[doc = "Field `PI_UPDATE_ERROR_STATUS` reader - Identifies the source of any DFI PI-initiated or PHY-initiated update\n\nerrors. A value of 1 indicates a timing violation of the associated\n\ntiming parameter.\n\nBit 5-0:\n\nphyupd_resp_error,\n\nphyupd_type3_error,\n\nphyupd_type2_error,\n\nphyupd_type1_error,\n\nctrlupd_max_error,\n\nctrlupd_interval_error."]
pub type PiUpdateErrorStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Identifies the source of any DFI PI-initiated or PHY-initiated update\n\nerrors. A value of 1 indicates a timing violation of the associated\n\ntiming parameter.\n\nBit 5-0:\n\nphyupd_resp_error,\n\nphyupd_type3_error,\n\nphyupd_type2_error,\n\nphyupd_type1_error,\n\nctrlupd_max_error,\n\nctrlupd_interval_error."]
    #[inline(always)]
    pub fn pi_update_error_status(&self) -> PiUpdateErrorStatusR {
        PiUpdateErrorStatusR::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "DDR PHY Independent Register 193\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_193::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg193Spec;
impl crate::RegisterSpec for DdrPiReg193Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_193::R`](R) reader structure"]
impl crate::Readable for DdrPiReg193Spec {}
#[doc = "`reset()` method sets DDR_PI_REG_193 to value 0"]
impl crate::Resettable for DdrPiReg193Spec {
    const RESET_VALUE: u32 = 0;
}
