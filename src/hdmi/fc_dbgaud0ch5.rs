#[doc = "Register `FC_DBGAUD0CH5` reader"]
pub type R = crate::R<FcDbgaud0ch5Spec>;
#[doc = "Register `FC_DBGAUD0CH5` writer"]
pub type W = crate::W<FcDbgaud0ch5Spec>;
#[doc = "Field `FC_DBGAUD0CH5` reader - Frame Composer Audio Data Channel 5 Register 0"]
pub type FcDbgaud0ch5R = crate::FieldReader;
#[doc = "Field `FC_DBGAUD0CH5` writer - Frame Composer Audio Data Channel 5 Register 0"]
pub type FcDbgaud0ch5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer Audio Data Channel 5 Register 0"]
    #[inline(always)]
    pub fn fc_dbgaud0ch5(&self) -> FcDbgaud0ch5R {
        FcDbgaud0ch5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer Audio Data Channel 5 Register 0"]
    #[inline(always)]
    #[must_use]
    pub fn fc_dbgaud0ch5(&mut self) -> FcDbgaud0ch5W<FcDbgaud0ch5Spec> {
        FcDbgaud0ch5W::new(self, 0)
    }
}
#[doc = "Frame Composer Audio Data Channel 5 Register 0\n\nConfigures the audio fixed data to be used in channel 5 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud0ch5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud0ch5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcDbgaud0ch5Spec;
impl crate::RegisterSpec for FcDbgaud0ch5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_dbgaud0ch5::R`](R) reader structure"]
impl crate::Readable for FcDbgaud0ch5Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_dbgaud0ch5::W`](W) writer structure"]
impl crate::Writable for FcDbgaud0ch5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_DBGAUD0CH5 to value 0"]
impl crate::Resettable for FcDbgaud0ch5Spec {
    const RESET_VALUE: u8 = 0;
}
