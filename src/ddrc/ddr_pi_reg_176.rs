#[doc = "Register `DDR_PI_REG_176` reader"]
pub type R = crate::R<DdrPiReg176Spec>;
#[doc = "Register `DDR_PI_REG_176` writer"]
pub type W = crate::W<DdrPiReg176Spec>;
#[doc = "Field `PI_INT_MASK` reader - Indicates mask for PI interrupt signals from the\n\nPI_REG_174.pi_int_status parameter."]
pub type PiIntMaskR = crate::FieldReader<u32>;
#[doc = "Field `PI_INT_MASK` writer - Indicates mask for PI interrupt signals from the\n\nPI_REG_174.pi_int_status parameter."]
pub type PiIntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Indicates mask for PI interrupt signals from the\n\nPI_REG_174.pi_int_status parameter."]
    #[inline(always)]
    pub fn pi_int_mask(&self) -> PiIntMaskR {
        PiIntMaskR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Indicates mask for PI interrupt signals from the\n\nPI_REG_174.pi_int_status parameter."]
    #[inline(always)]
    #[must_use]
    pub fn pi_int_mask(&mut self) -> PiIntMaskW<DdrPiReg176Spec> {
        PiIntMaskW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 176\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_176::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_176::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg176Spec;
impl crate::RegisterSpec for DdrPiReg176Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_176::R`](R) reader structure"]
impl crate::Readable for DdrPiReg176Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_176::W`](W) writer structure"]
impl crate::Writable for DdrPiReg176Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_176 to value 0"]
impl crate::Resettable for DdrPiReg176Spec {
    const RESET_VALUE: u32 = 0;
}
