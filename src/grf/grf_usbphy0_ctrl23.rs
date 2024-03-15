#[doc = "Register `GRF_usbphy0_ctrl23` reader"]
pub type R = crate::R<GrfUsbphy0Ctrl23Spec>;
#[doc = "Register `GRF_usbphy0_ctrl23` writer"]
pub type W = crate::W<GrfUsbphy0Ctrl23Spec>;
#[doc = "Field `USBPHY_CTRL23` reader - usbphy_ctrl23 Bit368~383 of usbphy_ctrl register"]
pub type UsbphyCtrl23R = crate::FieldReader<u16>;
#[doc = "Field `USBPHY_CTRL23` writer - usbphy_ctrl23 Bit368~383 of usbphy_ctrl register"]
pub type UsbphyCtrl23W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - usbphy_ctrl23 Bit368~383 of usbphy_ctrl register"]
    #[inline(always)]
    pub fn usbphy_ctrl23(&self) -> UsbphyCtrl23R {
        UsbphyCtrl23R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - usbphy_ctrl23 Bit368~383 of usbphy_ctrl register"]
    #[inline(always)]
    #[must_use]
    pub fn usbphy_ctrl23(&mut self) -> UsbphyCtrl23W<GrfUsbphy0Ctrl23Spec> {
        UsbphyCtrl23W::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfUsbphy0Ctrl23Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "usbphy0_ctrl23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfUsbphy0Ctrl23Spec;
impl crate::RegisterSpec for GrfUsbphy0Ctrl23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_usbphy0_ctrl23::R`](R) reader structure"]
impl crate::Readable for GrfUsbphy0Ctrl23Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_usbphy0_ctrl23::W`](W) writer structure"]
impl crate::Writable for GrfUsbphy0Ctrl23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_usbphy0_ctrl23 to value 0"]
impl crate::Resettable for GrfUsbphy0Ctrl23Spec {
    const RESET_VALUE: u32 = 0;
}