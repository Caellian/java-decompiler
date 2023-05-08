use java::JavaGeneratorBuilder;
use std::{io::Cursor, ops::Deref};

use self::writer::Indented;

pub mod java;
pub mod writer;

pub trait GeneratorBackend {
    const NAME: &'static str;

    type LanguageContext;
    type ScopeRequirements: Default + Sized;
}

pub struct GeneratorResult<B: GeneratorBackend + Sized, R, E> {
    pub inner: Result<R, E>,
    pub requirements: B::ScopeRequirements,
}

pub struct GeneratorResultOk<B: GeneratorBackend + Sized, R> {
    pub value: R,
    pub requirements: B::ScopeRequirements,
}

impl<B: GeneratorBackend + Sized, R, E> TryInto<GeneratorResultOk<B, R>>
    for GeneratorResult<B, R, E>
{
    type Error = E;
    fn try_into(self) -> Result<GeneratorResultOk<B, R>, Self::Error> {
        self.inner.map(|it| GeneratorResultOk {
            value: it,
            requirements: self.requirements,
        })
    }
}

impl<B: GeneratorBackend + Sized, R, E> Deref for GeneratorResult<B, R, E> {
    type Target = Result<R, E>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<B: GeneratorBackend, R, E> From<(Result<R, E>, B::ScopeRequirements)>
    for GeneratorResult<B, R, E>
{
    fn from(value: (Result<R, E>, B::ScopeRequirements)) -> Self {
        let (inner, requirements) = value;
        GeneratorResult {
            inner,
            requirements,
        }
    }
}

pub trait GenerateCode<I, C = ()>: GeneratorBackend + Sized {
    fn write_value<W: std::io::Write>(
        &self,
        lang: &Self::LanguageContext,
        ctx: &C,
        input: &I,
        w: &mut W,
    ) -> Result<Self::ScopeRequirements, std::io::Error>;

    fn generate(
        &self,
        lang: &Self::LanguageContext,
        ctx: &C,
        input: &I,
    ) -> Result<(String, Self::ScopeRequirements), std::io::Error> {
        let mut buff = Vec::with_capacity(64);
        let mut w = Cursor::new(&mut buff);
        let req = self.write_value(lang, ctx, input, &mut w)?;

        let string = std::str::from_utf8(w.into_inner().as_slice())
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?
            .to_string();
        Ok((string, req))
    }
}

pub struct GeneratorBuilder;
impl GeneratorBuilder {
    #[inline(always)]
    pub fn java() -> JavaGeneratorBuilder {
        JavaGeneratorBuilder::new()
    }
}
