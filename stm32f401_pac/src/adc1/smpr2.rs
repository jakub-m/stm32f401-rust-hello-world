#[doc = "Register `SMPR2` reader"]
pub type R = crate::R<Smpr2Spec>;
#[doc = "Register `SMPR2` writer"]
pub type W = crate::W<Smpr2Spec>;
#[doc = "Field `SMPx_x` reader - Sample time bits"]
pub type SmpxXR = crate::FieldReader<u32>;
#[doc = "Field `SMPx_x` writer - Sample time bits"]
pub type SmpxXW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Sample time bits"]
    #[inline(always)]
    pub fn smpx_x(&self) -> SmpxXR {
        SmpxXR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Sample time bits"]
    #[inline(always)]
    #[must_use]
    pub fn smpx_x(&mut self) -> SmpxXW<Smpr2Spec> {
        SmpxXW::new(self, 0)
    }
}
#[doc = "sample time register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`smpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Smpr2Spec;
impl crate::RegisterSpec for Smpr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr2::R`](R) reader structure"]
impl crate::Readable for Smpr2Spec {}
#[doc = "`write(|w| ..)` method takes [`smpr2::W`](W) writer structure"]
impl crate::Writable for Smpr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMPR2 to value 0"]
impl crate::Resettable for Smpr2Spec {
    const RESET_VALUE: u32 = 0;
}
