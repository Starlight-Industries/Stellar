pub mod arch;

pub trait Bootstrap
where
    Self: Sized,
{
    type Error;
    fn bootstrap();
    fn add_package() -> Result<(), Self::Error>;
}
