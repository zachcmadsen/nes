mod mapper0;

pub(crate) use mapper0::Mapper0;

pub(crate) trait Map {
    fn map(&mut self, address: u16) -> u16;
}
