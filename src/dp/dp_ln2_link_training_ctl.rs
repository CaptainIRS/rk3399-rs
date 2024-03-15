#[doc = "Register `DP_LN2_LINK_TRAINING_CTL` reader"]
pub type R = crate::R<DpLn2LinkTrainingCtlSpec>;
#[doc = "Register `DP_LN2_LINK_TRAINING_CTL` writer"]
pub type W = crate::W<DpLn2LinkTrainingCtlSpec>;
#[doc = "Lane 2 output amplitude setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DriveCurrentSet2 {
    #[doc = "3: 400 mV. This bit's type is R/W."]
    B11 = 3,
    #[doc = "2: 400 mV. This bit's type is R/W."]
    B10 = 2,
    #[doc = "1: 400 mV. This bit's type is R/W."]
    B01 = 1,
    #[doc = "0: 400 mV. This bit's type is R/W."]
    B00 = 0,
}
impl From<DriveCurrentSet2> for u8 {
    #[inline(always)]
    fn from(variant: DriveCurrentSet2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DriveCurrentSet2 {
    type Ux = u8;
}
#[doc = "Field `DRIVE_CURRENT_SET_2` reader - Lane 2 output amplitude setting"]
pub type DriveCurrentSet2R = crate::FieldReader<DriveCurrentSet2>;
impl DriveCurrentSet2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DriveCurrentSet2 {
        match self.bits {
            3 => DriveCurrentSet2::B11,
            2 => DriveCurrentSet2::B10,
            1 => DriveCurrentSet2::B01,
            0 => DriveCurrentSet2::B00,
            _ => unreachable!(),
        }
    }
    #[doc = "400 mV. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == DriveCurrentSet2::B11
    }
    #[doc = "400 mV. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == DriveCurrentSet2::B10
    }
    #[doc = "400 mV. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == DriveCurrentSet2::B01
    }
    #[doc = "400 mV. This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == DriveCurrentSet2::B00
    }
}
#[doc = "Field `DRIVE_CURRENT_SET_2` writer - Lane 2 output amplitude setting"]
pub type DriveCurrentSet2W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DriveCurrentSet2>;
impl<'a, REG> DriveCurrentSet2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "400 mV. This bit's type is R/W."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(DriveCurrentSet2::B11)
    }
    #[doc = "400 mV. This bit's type is R/W."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(DriveCurrentSet2::B10)
    }
    #[doc = "400 mV. This bit's type is R/W."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(DriveCurrentSet2::B01)
    }
    #[doc = "400 mV. This bit's type is R/W."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(DriveCurrentSet2::B00)
    }
}
#[doc = "Field `MAX_DRIVE_REACH_2` reader - This bit field is set to 1 automatically when max driving current level of DP Tx is reached. For more information, refer to MAX_PRE_REACH_2. For test purpose only. This bit's type is RO."]
pub type MaxDriveReach2R = crate::BitReader;
#[doc = "Lane 2 pre-emphasis level setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PreEmphasisSet2 {
    #[doc = "3: 0 dB (No pre-emphasis). This bit's type is R/W."]
    B11 = 3,
    #[doc = "2: 0 dB (No pre-emphasis). This bit's type is R/W."]
    B10 = 2,
    #[doc = "1: 0 dB (No pre-emphasis). This bit's type is R/W."]
    B01 = 1,
    #[doc = "0: 0 dB (No pre-emphasis). This bit's type is R/W."]
    B00 = 0,
}
impl From<PreEmphasisSet2> for u8 {
    #[inline(always)]
    fn from(variant: PreEmphasisSet2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PreEmphasisSet2 {
    type Ux = u8;
}
#[doc = "Field `PRE_EMPHASIS_SET_2` reader - Lane 2 pre-emphasis level setting"]
pub type PreEmphasisSet2R = crate::FieldReader<PreEmphasisSet2>;
impl PreEmphasisSet2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PreEmphasisSet2 {
        match self.bits {
            3 => PreEmphasisSet2::B11,
            2 => PreEmphasisSet2::B10,
            1 => PreEmphasisSet2::B01,
            0 => PreEmphasisSet2::B00,
            _ => unreachable!(),
        }
    }
    #[doc = "0 dB (No pre-emphasis). This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == PreEmphasisSet2::B11
    }
    #[doc = "0 dB (No pre-emphasis). This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == PreEmphasisSet2::B10
    }
    #[doc = "0 dB (No pre-emphasis). This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == PreEmphasisSet2::B01
    }
    #[doc = "0 dB (No pre-emphasis). This bit's type is R/W."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == PreEmphasisSet2::B00
    }
}
#[doc = "Field `PRE_EMPHASIS_SET_2` writer - Lane 2 pre-emphasis level setting"]
pub type PreEmphasisSet2W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PreEmphasisSet2>;
impl<'a, REG> PreEmphasisSet2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 dB (No pre-emphasis). This bit's type is R/W."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(PreEmphasisSet2::B11)
    }
    #[doc = "0 dB (No pre-emphasis). This bit's type is R/W."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(PreEmphasisSet2::B10)
    }
    #[doc = "0 dB (No pre-emphasis). This bit's type is R/W."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(PreEmphasisSet2::B01)
    }
    #[doc = "0 dB (No pre-emphasis). This bit's type is R/W."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(PreEmphasisSet2::B00)
    }
}
#[doc = "Field `MAX_PRE_REACH_2` reader - This bit field is set to 1 automatically when max pre-emphasis level of DP Tx is reached. Note that the MAX_PRE_REACH_2 and MAX_DRIVE_REACH_2 have the same value like the following table. Both of MAX_PRE_REACH_1 and MAX_DRIVE_REACH_2 are for test purpose only. This bit's type is RO."]
pub type MaxPreReach2R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Lane 2 output amplitude setting"]
    #[inline(always)]
    pub fn drive_current_set_2(&self) -> DriveCurrentSet2R {
        DriveCurrentSet2R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - This bit field is set to 1 automatically when max driving current level of DP Tx is reached. For more information, refer to MAX_PRE_REACH_2. For test purpose only. This bit's type is RO."]
    #[inline(always)]
    pub fn max_drive_reach_2(&self) -> MaxDriveReach2R {
        MaxDriveReach2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Lane 2 pre-emphasis level setting"]
    #[inline(always)]
    pub fn pre_emphasis_set_2(&self) -> PreEmphasisSet2R {
        PreEmphasisSet2R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - This bit field is set to 1 automatically when max pre-emphasis level of DP Tx is reached. Note that the MAX_PRE_REACH_2 and MAX_DRIVE_REACH_2 have the same value like the following table. Both of MAX_PRE_REACH_1 and MAX_DRIVE_REACH_2 are for test purpose only. This bit's type is RO."]
    #[inline(always)]
    pub fn max_pre_reach_2(&self) -> MaxPreReach2R {
        MaxPreReach2R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Lane 2 output amplitude setting"]
    #[inline(always)]
    #[must_use]
    pub fn drive_current_set_2(&mut self) -> DriveCurrentSet2W<DpLn2LinkTrainingCtlSpec> {
        DriveCurrentSet2W::new(self, 0)
    }
    #[doc = "Bits 3:4 - Lane 2 pre-emphasis level setting"]
    #[inline(always)]
    #[must_use]
    pub fn pre_emphasis_set_2(&mut self) -> PreEmphasisSet2W<DpLn2LinkTrainingCtlSpec> {
        PreEmphasisSet2W::new(self, 3)
    }
}
#[doc = "DP Lane 2 Link Training Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_ln2_link_training_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_ln2_link_training_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpLn2LinkTrainingCtlSpec;
impl crate::RegisterSpec for DpLn2LinkTrainingCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_ln2_link_training_ctl::R`](R) reader structure"]
impl crate::Readable for DpLn2LinkTrainingCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_ln2_link_training_ctl::W`](W) writer structure"]
impl crate::Writable for DpLn2LinkTrainingCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
#[doc = "`reset()` method sets DP_LN2_LINK_TRAINING_CTL to value 0"]
impl crate::Resettable for DpLn2LinkTrainingCtlSpec {
    const RESET_VALUE: u32 = 0;
}