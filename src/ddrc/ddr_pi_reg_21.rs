#[doc = "Register `DDR_PI_REG_21` reader"]
pub type R = crate::R<DdrPiReg21Spec>;
#[doc = "Register `DDR_PI_REG_21` writer"]
pub type W = crate::W<DdrPiReg21Spec>;
#[doc = "Field `PI_TDFI_PHYUPD_TYPE3_F2` reader - Defines the DFI tPHYUPD_TYPE3 timing parameter (in DFI clocks),\n\nthe maximum cycles that dfi_phyupd_req can assert after\n\ndfi_phyupd_ack for dfi_phyupd_type 3. If programmed to a\n\nnon-zero, a timing violation causes an interrupt and bit7 set in the\n\nPI_REG_193.pi_update_error_status parameter and bit7 set in the\n\nPI_REG_22.pi_control_error_status parameter. The suffix f2 of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhyupdType3F2R = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_PHYUPD_TYPE3_F2` writer - Defines the DFI tPHYUPD_TYPE3 timing parameter (in DFI clocks),\n\nthe maximum cycles that dfi_phyupd_req can assert after\n\ndfi_phyupd_ack for dfi_phyupd_type 3. If programmed to a\n\nnon-zero, a timing violation causes an interrupt and bit7 set in the\n\nPI_REG_193.pi_update_error_status parameter and bit7 set in the\n\nPI_REG_22.pi_control_error_status parameter. The suffix f2 of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhyupdType3F2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE3 timing parameter (in DFI clocks),\n\nthe maximum cycles that dfi_phyupd_req can assert after\n\ndfi_phyupd_ack for dfi_phyupd_type 3. If programmed to a\n\nnon-zero, a timing violation causes an interrupt and bit7 set in the\n\nPI_REG_193.pi_update_error_status parameter and bit7 set in the\n\nPI_REG_22.pi_control_error_status parameter. The suffix f2 of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_phyupd_type3_f2(&self) -> PiTdfiPhyupdType3F2R {
        PiTdfiPhyupdType3F2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE3 timing parameter (in DFI clocks),\n\nthe maximum cycles that dfi_phyupd_req can assert after\n\ndfi_phyupd_ack for dfi_phyupd_type 3. If programmed to a\n\nnon-zero, a timing violation causes an interrupt and bit7 set in the\n\nPI_REG_193.pi_update_error_status parameter and bit7 set in the\n\nPI_REG_22.pi_control_error_status parameter. The suffix f2 of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_phyupd_type3_f2(&mut self) -> PiTdfiPhyupdType3F2W<DdrPiReg21Spec> {
        PiTdfiPhyupdType3F2W::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg21Spec;
impl crate::RegisterSpec for DdrPiReg21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_21::R`](R) reader structure"]
impl crate::Readable for DdrPiReg21Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_21::W`](W) writer structure"]
impl crate::Writable for DdrPiReg21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_21 to value 0"]
impl crate::Resettable for DdrPiReg21Spec {
    const RESET_VALUE: u32 = 0;
}
