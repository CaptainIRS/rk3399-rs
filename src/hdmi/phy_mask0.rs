#[doc = "Register `PHY_MASK0` reader"]
pub type R = crate::R<PhyMask0Spec>;
#[doc = "Register `PHY_MASK0` writer"]
pub type W = crate::W<PhyMask0Spec>;
#[doc = "Field `TX_PHY_LOCK` reader - Mask bit for PHY_INT0.TX_PHY_LOCK interrupt bit"]
pub type TxPhyLockR = crate::BitReader;
#[doc = "Field `TX_PHY_LOCK` writer - Mask bit for PHY_INT0.TX_PHY_LOCK interrupt bit"]
pub type TxPhyLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPD` reader - Mask bit for PHY_INT0.HPD interrupt bit"]
pub type HpdR = crate::BitReader;
#[doc = "Field `HPD` writer - Mask bit for PHY_INT0.HPD interrupt bit"]
pub type HpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SENSE_0` reader - Mask bit for PHY_INT0.RX_SENSE\\[0\\]
interrupt bit"]
pub type RxSense0R = crate::BitReader;
#[doc = "Field `RX_SENSE_0` writer - Mask bit for PHY_INT0.RX_SENSE\\[0\\]
interrupt bit"]
pub type RxSense0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SENSE_1` reader - Mask bit for PHY_INT0.RX_SENSE\\[1\\]
interrupt bit"]
pub type RxSense1R = crate::BitReader;
#[doc = "Field `RX_SENSE_1` writer - Mask bit for PHY_INT0.RX_SENSE\\[1\\]
interrupt bit"]
pub type RxSense1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SENSE_2` reader - Mask bit for PHY_INT0.RX_SENSE\\[2\\]
interrupt bit"]
pub type RxSense2R = crate::BitReader;
#[doc = "Field `RX_SENSE_2` writer - Mask bit for PHY_INT0.RX_SENSE\\[2\\]
interrupt bit"]
pub type RxSense2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SENSE_3` reader - Mask bit for PHY_INT0.RX_SENSE\\[3\\]
interrupt bit"]
pub type RxSense3R = crate::BitReader;
#[doc = "Field `RX_SENSE_3` writer - Mask bit for PHY_INT0.RX_SENSE\\[3\\]
interrupt bit"]
pub type RxSense3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask bit for PHY_INT0.TX_PHY_LOCK interrupt bit"]
    #[inline(always)]
    pub fn tx_phy_lock(&self) -> TxPhyLockR {
        TxPhyLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for PHY_INT0.HPD interrupt bit"]
    #[inline(always)]
    pub fn hpd(&self) -> HpdR {
        HpdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask bit for PHY_INT0.RX_SENSE\\[0\\]
interrupt bit"]
    #[inline(always)]
    pub fn rx_sense_0(&self) -> RxSense0R {
        RxSense0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask bit for PHY_INT0.RX_SENSE\\[1\\]
interrupt bit"]
    #[inline(always)]
    pub fn rx_sense_1(&self) -> RxSense1R {
        RxSense1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask bit for PHY_INT0.RX_SENSE\\[2\\]
interrupt bit"]
    #[inline(always)]
    pub fn rx_sense_2(&self) -> RxSense2R {
        RxSense2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask bit for PHY_INT0.RX_SENSE\\[3\\]
interrupt bit"]
    #[inline(always)]
    pub fn rx_sense_3(&self) -> RxSense3R {
        RxSense3R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for PHY_INT0.TX_PHY_LOCK interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn tx_phy_lock(&mut self) -> TxPhyLockW<PhyMask0Spec> {
        TxPhyLockW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask bit for PHY_INT0.HPD interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn hpd(&mut self) -> HpdW<PhyMask0Spec> {
        HpdW::new(self, 1)
    }
    #[doc = "Bit 4 - Mask bit for PHY_INT0.RX_SENSE\\[0\\]
interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sense_0(&mut self) -> RxSense0W<PhyMask0Spec> {
        RxSense0W::new(self, 4)
    }
    #[doc = "Bit 5 - Mask bit for PHY_INT0.RX_SENSE\\[1\\]
interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sense_1(&mut self) -> RxSense1W<PhyMask0Spec> {
        RxSense1W::new(self, 5)
    }
    #[doc = "Bit 6 - Mask bit for PHY_INT0.RX_SENSE\\[2\\]
interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sense_2(&mut self) -> RxSense2W<PhyMask0Spec> {
        RxSense2W::new(self, 6)
    }
    #[doc = "Bit 7 - Mask bit for PHY_INT0.RX_SENSE\\[3\\]
interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sense_3(&mut self) -> RxSense3W<PhyMask0Spec> {
        RxSense3W::new(self, 7)
    }
}
#[doc = "PHY RXSENSE, PLL Lock, and HPD Mask Register Mask register for generation\n\nof PHY_INT0 interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_mask0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_mask0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyMask0Spec;
impl crate::RegisterSpec for PhyMask0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_mask0::R`](R) reader structure"]
impl crate::Readable for PhyMask0Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_mask0::W`](W) writer structure"]
impl crate::Writable for PhyMask0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PHY_MASK0 to value 0"]
impl crate::Resettable for PhyMask0Spec {
    const RESET_VALUE: u8 = 0;
}
