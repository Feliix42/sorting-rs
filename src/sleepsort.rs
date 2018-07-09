use std::iter::ExactSizeIterator;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::num::Wrapping;

/// This trait provides  the `sleepsort` functionality
pub trait Sleepsort: IntoIterator + Sized
    where
        Self::Item: SleepsortItem + PartialOrd + Send + 'static
{
    /// **Sleepsort** is an unstable time-based variation of **Countingsort** that
    /// works by assigning every value to a thread and putting that thread to sleep
    /// for a time that is determined by that value:
    ///
    /// ```text
    /// for i in values:
    ///     spawn a thread that sleeps i seconds
    ///     output i
    /// ```
    ///
    /// Unlike many other implementations, this one does not just print the values in
    /// order, but actually collects them, and is able to sort negative signed
    /// integers. Using it for other types is easily possible by simply implementing
    /// the [`SleepsortItem`] trait.
    fn sleepsort(self) -> SleepsortIter<Self::Item> {
        self.sleepsort_with_speed(SleepsortSpeed::Default)
    }

    /// [`sleepsort`](Sleepsort::sleepsort) with a custom speed. See
    /// [`SleepsortSpeed`] for more information.
    fn sleepsort_with_speed(self, speed: SleepsortSpeed)
        -> SleepsortIter<Self::Item>
    {
        let buffer: Vec<_> = self.into_iter().collect();
        let len = buffer.len();
        let (tx, rx) = mpsc::sync_channel(len);

        for item in buffer {
            let tx = tx.clone();
            thread::spawn(move || {
                thread::sleep(speed.adjust_duration(item.key()));
                tx.send(item).unwrap();
            });
        }

        SleepsortIter { rx: rx.into_iter(), len }
    }
}

/// Marks a trait that can be [`sleepsort`](Sleepsort::sleepsort)ed.
pub trait SleepsortItem: PartialOrd + Send + 'static {
    /// Determine how long the thread should sleep for this particular value.
    fn key(&self) -> Duration;
}

/// An iterator over a [`sleepsort`](Sleepsort::sleepsort)ed collection.
#[derive(Debug)]
pub struct SleepsortIter<I> {
    rx: mpsc::IntoIter<I>,
    len: usize
}

/// Adjusts how long [`sleepsort`](Sleepsort::sleepsort) threads sleep.
#[derive(Copy, Clone, Debug)]
pub enum SleepsortSpeed {
    /// Do not change sleep duration.
    Default,

    /// Shorten sleep duration by dividing it by the given value. Too short
    /// durations will cause incorrect results.
    Faster(u32),

    /// Lengthen sleep duration by multiplying it with the given value. Use this if
    /// incorrect results are produced.
    Slower(u32)
}


impl<T> Sleepsort for T
    where
        T: IntoIterator + Sized,
        T::Item: SleepsortItem + PartialOrd + Send + 'static
{}

impl SleepsortItem for u8 {
    fn key(&self) -> Duration {
        Duration::from_secs(u64::from(*self))
    }
}

impl SleepsortItem for u16 {
    fn key(&self) -> Duration {
        Duration::from_secs(u64::from(*self))
    }
}

impl SleepsortItem for u32 {
    fn key(&self) -> Duration {
        Duration::from_secs(u64::from(*self))
    }
}

impl SleepsortItem for u64 {
    fn key(&self) -> Duration {
        Duration::from_secs(*self)
    }
}

impl SleepsortItem for i8 {
    fn key(&self) -> Duration {
        let Wrapping(seconds) =
            Wrapping(*self as u8) + Wrapping(i8::max_value() as u8 + 1);
        Duration::from_secs(u64::from(seconds))
    }
}

impl SleepsortItem for i16 {
    fn key(&self) -> Duration {
        let Wrapping(seconds) =
            Wrapping(*self as u16) + Wrapping(i16::max_value() as u16 + 1);
        Duration::from_secs(u64::from(seconds))
    }
}

impl SleepsortItem for i32 {
    fn key(&self) -> Duration {
        let Wrapping(seconds) =
            Wrapping(*self as u32) + Wrapping(i32::max_value() as u32 + 1);
        Duration::from_secs(u64::from(seconds))
    }
}

impl SleepsortItem for i64 {
    fn key(&self) -> Duration {
        let Wrapping(seconds) =
            Wrapping(*self as u64) + Wrapping(i64::max_value() as u64 + 1);
        Duration::from_secs(seconds)
    }
}

impl<I> Iterator for SleepsortIter<I>
    where
        I: SleepsortItem
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        self.rx.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<I> ExactSizeIterator for SleepsortIter<I>
    where
    I: SleepsortItem
{
    fn len(&self) -> usize {
        self.len
    }
}


impl SleepsortSpeed {
    fn adjust_duration(self, dur: Duration) -> Duration {
        use SleepsortSpeed::*;

        match self {
            Default => dur,
            Faster(div) => dur / div,
            Slower(mul) => dur * mul
        }
    }
}
