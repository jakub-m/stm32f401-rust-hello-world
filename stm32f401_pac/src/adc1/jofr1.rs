#[doc = "Register `JOFR1` reader"]
pub type R = crate::R<Jofr1Spec>;
#[doc = "Register `JOFR1` writer"]
pub type W = crate::W<Jofr1Spec>;
#[doc = "Field `JOFFSET1` reader - Data offset for injected channel x"]
pub type Joffset1R = crate::FieldReader<u16>;
#[doc = "Field `JOFFSET1` writer - Data offset for injected channel x"]
pub type Joffset1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn joffset1(&self) -> Joffset1R {
        Joffset1R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    #[must_use]
    pub fn joffset1(&mut self) -> Joffset1W<Jofr1Spec> {
        Joffset1W::new(self, 0)
    }
}
#[doc = "injected channel data offset register x\n\nYou can [`read`](crate::Reg::read) this register and get [`jofr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jofr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jofr1Spec;
impl crate::RegisterSpec for Jofr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jofr1::R`](R) reader structure"]
impl crate::Readable for Jofr1Spec {}
#[doc = "`write(|w| ..)` method takes [`jofr1::W`](W) writer structure"]
impl crate::Writable for Jofr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets JOFR1 to value 0"]
impl crate::Resettable for Jofr1Spec {
    const RESET_VALUE: u32 = 0;
}
