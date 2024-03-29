#[doc = "Register `DDR_PI_REG_82` reader"]
pub type R = crate::R<DdrPiReg82Spec>;
#[doc = "Register `DDR_PI_REG_82` writer"]
pub type W = crate::W<DdrPiReg82Spec>;
#[doc = "Field `PI_RDLVL_ERROR_STATUS` reader - Holds the error that is associated with the data eye training error or\n\ngate training error interrupt. The uppermost bit set indicates a\n\nPI_REG_79.pi_tdfi_rdlvl_resp parameter violation. The next\n\nuppermost bit set indicates a PI_REG_81.pi_tdfi_rdlvl_max\n\nparameter violation. Lower bits are reserved."]
pub type PiRdlvlErrorStatusR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_INTERVAL` reader - Indicates the number of long count sequences that are counted\n\nbetween automatic data eye training commands."]
pub type PiRdlvlIntervalR = crate::FieldReader<u16>;
#[doc = "Field `PI_RDLVL_INTERVAL` writer - Indicates the number of long count sequences that are counted\n\nbetween automatic data eye training commands."]
pub type PiRdlvlIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - Holds the error that is associated with the data eye training error or\n\ngate training error interrupt. The uppermost bit set indicates a\n\nPI_REG_79.pi_tdfi_rdlvl_resp parameter violation. The next\n\nuppermost bit set indicates a PI_REG_81.pi_tdfi_rdlvl_max\n\nparameter violation. Lower bits are reserved."]
    #[inline(always)]
    pub fn pi_rdlvl_error_status(&self) -> PiRdlvlErrorStatusR {
        PiRdlvlErrorStatusR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:23 - Indicates the number of long count sequences that are counted\n\nbetween automatic data eye training commands."]
    #[inline(always)]
    pub fn pi_rdlvl_interval(&self) -> PiRdlvlIntervalR {
        PiRdlvlIntervalR::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:23 - Indicates the number of long count sequences that are counted\n\nbetween automatic data eye training commands."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_interval(&mut self) -> PiRdlvlIntervalW<DdrPiReg82Spec> {
        PiRdlvlIntervalW::new(self, 8)
    }
}
#[doc = "DDR PHY Independent Register 82\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_82::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_82::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg82Spec;
impl crate::RegisterSpec for DdrPiReg82Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_82::R`](R) reader structure"]
impl crate::Readable for DdrPiReg82Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_82::W`](W) writer structure"]
impl crate::Writable for DdrPiReg82Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_82 to value 0"]
impl crate::Resettable for DdrPiReg82Spec {
    const RESET_VALUE: u32 = 0;
}
