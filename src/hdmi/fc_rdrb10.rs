#[doc = "Register `FC_RDRB10` reader"]
pub type R = crate::R<FcRdrb10Spec>;
#[doc = "Register `FC_RDRB10` writer"]
pub type W = crate::W<FcRdrb10Spec>;
#[doc = "Field `NVBIFRAMEINTERPOLATION` reader - NTSC VBI frame interpolation"]
pub type NvbiframeinterpolationR = crate::FieldReader;
#[doc = "Field `NVBIFRAMEINTERPOLATION` writer - NTSC VBI frame interpolation"]
pub type NvbiframeinterpolationW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - NTSC VBI frame interpolation"]
    #[inline(always)]
    pub fn nvbiframeinterpolation(&self) -> NvbiframeinterpolationR {
        NvbiframeinterpolationR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - NTSC VBI frame interpolation"]
    #[inline(always)]
    #[must_use]
    pub fn nvbiframeinterpolation(&mut self) -> NvbiframeinterpolationW<FcRdrb10Spec> {
        NvbiframeinterpolationW::new(self, 0)
    }
}
#[doc = "Frame Composer Round Robin NTSC VBI Packet Insertion Register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcRdrb10Spec;
impl crate::RegisterSpec for FcRdrb10Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_rdrb10::R`](R) reader structure"]
impl crate::Readable for FcRdrb10Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_rdrb10::W`](W) writer structure"]
impl crate::Writable for FcRdrb10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_RDRB10 to value 0"]
impl crate::Resettable for FcRdrb10Spec {
    const RESET_VALUE: u8 = 0;
}
