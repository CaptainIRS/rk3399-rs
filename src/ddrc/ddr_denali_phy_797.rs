#[doc = "Register `DDR_DENALI_PHY_797` reader"]
pub type R = crate::R<DdrDenaliPhy797Spec>;
#[doc = "Register `DDR_DENALI_PHY_797` writer"]
pub type W = crate::W<DdrDenaliPhy797Spec>;
#[doc = "Field `PHY_ADR_ADDR_SEL_2` reader - Mux select to map in LPDDR4 addressing for address slice 2."]
pub type PhyAdrAddrSel2R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_ADDR_SEL_2` writer - Mux select to map in LPDDR4 addressing for address slice 2."]
pub type PhyAdrAddrSel2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Mux select to map in LPDDR4 addressing for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_addr_sel_2(&self) -> PhyAdrAddrSel2R {
        PhyAdrAddrSel2R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Mux select to map in LPDDR4 addressing for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_addr_sel_2(&mut self) -> PhyAdrAddrSel2W<DdrDenaliPhy797Spec> {
        PhyAdrAddrSel2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_797::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_797::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy797Spec;
impl crate::RegisterSpec for DdrDenaliPhy797Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_797::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy797Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_797::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy797Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_797 to value 0"]
impl crate::Resettable for DdrDenaliPhy797Spec {
    const RESET_VALUE: u32 = 0;
}
