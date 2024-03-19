#[doc = "Register `DDR_DENALI_PHY_412` reader"]
pub type R = crate::R<DdrDenaliPhy412Spec>;
#[doc = "Register `DDR_DENALI_PHY_412` writer"]
pub type W = crate::W<DdrDenaliPhy412Spec>;
#[doc = "Field `PHY_USER_PATT0_3` reader - User-defined pattern to be used during write data leveling for slice 3. This register holds the bytes 3 to 0 written/read from device."]
pub type PhyUserPatt0_3R = crate::FieldReader<u32>;
#[doc = "Field `PHY_USER_PATT0_3` writer - User-defined pattern to be used during write data leveling for slice 3. This register holds the bytes 3 to 0 written/read from device."]
pub type PhyUserPatt0_3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User-defined pattern to be used during write data leveling for slice 3. This register holds the bytes 3 to 0 written/read from device."]
    #[inline(always)]
    pub fn phy_user_patt0_3(&self) -> PhyUserPatt0_3R {
        PhyUserPatt0_3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User-defined pattern to be used during write data leveling for slice 3. This register holds the bytes 3 to 0 written/read from device."]
    #[inline(always)]
    #[must_use]
    pub fn phy_user_patt0_3(&mut self) -> PhyUserPatt0_3W<DdrDenaliPhy412Spec> {
        PhyUserPatt0_3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_412::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_412::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy412Spec;
impl crate::RegisterSpec for DdrDenaliPhy412Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_412::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy412Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_412::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy412Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_412 to value 0"]
impl crate::Resettable for DdrDenaliPhy412Spec {
    const RESET_VALUE: u32 = 0;
}
