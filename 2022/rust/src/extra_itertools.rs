pub struct MapOkRes<I, TI, TO, E, F>
where
    I: Iterator<Item = Result<TI, E>>,
    F: FnMut(TI) -> Result<TO, E>,
{
    iter: I,
    f: F,
}

impl<I, TI, TO, E, F> Iterator for MapOkRes<I, TI, TO, E, F>
where
    I: Iterator<Item = Result<TI, E>>,
    F: FnMut(TI) -> Result<TO, E>,
{
    type Item = Result<TO, E>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(v)) => Some((self.f)(v)),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }
}

pub struct FilterMapOkRes<I, TI, TO, E, F>
where
    I: Iterator<Item = Result<TI, E>>,
    F: FnMut(TI) -> Result<Option<TO>, E>,
{
    iter: I,
    f: F,
}

impl<I, TI, TO, E, F> Iterator for FilterMapOkRes<I, TI, TO, E, F>
where
    I: Iterator<Item = Result<TI, E>>,
    F: FnMut(TI) -> Result<Option<TO>, E>,
{
    type Item = Result<TO, E>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            return match self.iter.next() {
                Some(Ok(v)) => match (self.f)(v) {
                    Ok(Some(v)) => Some(Ok(v)),
                    Ok(None) => continue,
                    Err(e) => Some(Err(e)),
                },
                Some(Err(e)) => Some(Err(e)),
                None => None,
            };
        }
    }
}

pub trait ExtraItertools: Iterator {
    #[inline]
    fn map_ok_res<F, TI, TO, E>(self, f: F) -> MapOkRes<Self, TI, TO, E, F>
    where
        F: FnMut(TI) -> Result<TO, E>,
        Self: Iterator<Item = Result<TI, E>> + Sized,
    {
        MapOkRes { iter: self, f }
    }

    #[inline]
    fn filter_map_ok_res<F, TI, TO, E>(self, f: F) -> FilterMapOkRes<Self, TI, TO, E, F>
    where
        F: FnMut(TI) -> Result<Option<TO>, E>,
        Self: Iterator<Item = Result<TI, E>> + Sized,
    {
        FilterMapOkRes { iter: self, f }
    }
}

impl<I: Iterator + ?Sized> ExtraItertools for I {}
