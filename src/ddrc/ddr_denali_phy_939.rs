#[doc = "Register `DDR_DENALI_PHY_939` reader"]
pub type R = crate::R<DdrDenaliPhy939Spec>;
#[doc = "Register `DDR_DENALI_PHY_939` writer"]
pub type W = crate::W<DdrDenaliPhy939Spec>;
#[doc = "Field `PHY_PAD_CS_DRIVE` reader - 0x0-0x1fffffff Controls drive settings for cs pads."]
pub type PhyPadCsDriveR = crate::FieldReader<u32>;
#[doc = "Field `PHY_PAD_CS_DRIVE` writer - 0x0-0x1fffffff Controls drive settings for cs pads."]
pub type PhyPadCsDriveW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - 0x0-0x1fffffff Controls drive settings for cs pads."]
    #[inline(always)]
    pub fn phy_pad_cs_drive(&self) -> PhyPadCsDriveR {
        PhyPadCsDriveR::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - 0x0-0x1fffffff Controls drive settings for cs pads."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_cs_drive(&mut self) -> PhyPadCsDriveW<DdrDenaliPhy939Spec> {
        PhyPadCsDriveW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_939::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_939::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy939Spec;
impl crate::RegisterSpec for DdrDenaliPhy939Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_939::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy939Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_939::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy939Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_939 to value 0x0f"]
impl crate::Resettable for DdrDenaliPhy939Spec {
    const RESET_VALUE: u32 = 0x0f;
}
