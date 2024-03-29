#[doc = "Register `ACTIVE_PIXEL_STA_L` reader"]
pub type R = crate::R<ActivePixelStaLSpec>;
#[doc = "Register `ACTIVE_PIXEL_STA_L` writer"]
pub type W = crate::W<ActivePixelStaLSpec>;
#[doc = "Field `ACTIVE_PIXEL_STA_L` reader - ACTIVE_PIXEL \\[7:0\\]
which is detected by \n\nvideo capture module. \n\nThis bit field is valid only when STRM_VALID \n\nis high. And STRM_VALID becomes high when \n\ntwo successive frames are determined as \n\nstable."]
pub type ActivePixelStaLR = crate::FieldReader;
#[doc = "Field `ACTIVE_PIXEL_STA_L` writer - ACTIVE_PIXEL \\[7:0\\]
which is detected by \n\nvideo capture module. \n\nThis bit field is valid only when STRM_VALID \n\nis high. And STRM_VALID becomes high when \n\ntwo successive frames are determined as \n\nstable."]
pub type ActivePixelStaLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ACTIVE_PIXEL \\[7:0\\]
which is detected by \n\nvideo capture module. \n\nThis bit field is valid only when STRM_VALID \n\nis high. And STRM_VALID becomes high when \n\ntwo successive frames are determined as \n\nstable."]
    #[inline(always)]
    pub fn active_pixel_sta_l(&self) -> ActivePixelStaLR {
        ActivePixelStaLR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ACTIVE_PIXEL \\[7:0\\]
which is detected by \n\nvideo capture module. \n\nThis bit field is valid only when STRM_VALID \n\nis high. And STRM_VALID becomes high when \n\ntwo successive frames are determined as \n\nstable."]
    #[inline(always)]
    #[must_use]
    pub fn active_pixel_sta_l(&mut self) -> ActivePixelStaLW<ActivePixelStaLSpec> {
        ActivePixelStaLW::new(self, 0)
    }
}
#[doc = "Active Pixel Status Low Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`active_pixel_sta_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`active_pixel_sta_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActivePixelStaLSpec;
impl crate::RegisterSpec for ActivePixelStaLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`active_pixel_sta_l::R`](R) reader structure"]
impl crate::Readable for ActivePixelStaLSpec {}
#[doc = "`write(|w| ..)` method takes [`active_pixel_sta_l::W`](W) writer structure"]
impl crate::Writable for ActivePixelStaLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets ACTIVE_PIXEL_STA_L to value 0"]
impl crate::Resettable for ActivePixelStaLSpec {
    const RESET_VALUE: u32 = 0;
}
