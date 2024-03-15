#[doc = "Register `SPDIF_USRDR_SHDn` reader"]
pub type R = crate::R<SpdifUsrdrShdnSpec>;
#[doc = "Field `USR_SUB_0` reader - User Data Subframe 0 User Data Bit for Subframe 0"]
pub type UsrSub0R = crate::FieldReader<u16>;
#[doc = "Field `USR_SUB_1` reader - User Data Subframe 1 User Data Bit for Subframe 1"]
pub type UsrSub1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - User Data Subframe 0 User Data Bit for Subframe 0"]
    #[inline(always)]
    pub fn usr_sub_0(&self) -> UsrSub0R {
        UsrSub0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - User Data Subframe 1 User Data Bit for Subframe 1"]
    #[inline(always)]
    pub fn usr_sub_1(&self) -> UsrSub1R {
        UsrSub1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Shadow User Data Register n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_usrdr_shdn::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdifUsrdrShdnSpec;
impl crate::RegisterSpec for SpdifUsrdrShdnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdif_usrdr_shdn::R`](R) reader structure"]
impl crate::Readable for SpdifUsrdrShdnSpec {}
#[doc = "`reset()` method sets SPDIF_USRDR_SHDn to value 0"]
impl crate::Resettable for SpdifUsrdrShdnSpec {
    const RESET_VALUE: u32 = 0;
}