pub struct CircularBuffer<T> {
    data: Vec<Option<T>>,
    capacity: usize,
    occupied: usize,
    position: usize,
    oldest: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut data = vec![];
        for _ in 0..capacity {
            data.push(None);
        }
        CircularBuffer {
            data,
            capacity,
            occupied: 0,
            position: 0,
            oldest: 0,
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.occupied == self.capacity {
            Err(Error::FullBuffer)
        } else {
            self.data[self.position] = Some(_element);
            self.occupied += 1;
            self.position += 1;
            self.position %= self.capacity;

            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.occupied == 0 {
            Err(Error::EmptyBuffer)
        } else {
            let ret = self.data[self.oldest].take();
            self.occupied -= 1;
            self.oldest += 1;
            self.oldest %= self.capacity;

            Ok(ret.unwrap())
        }
    }

    pub fn clear(&mut self) {
        *self = Self::new(self.capacity);
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.occupied == self.capacity {
            self.data[self.oldest] = Some(_element);
            self.oldest += 1;
            self.oldest %= self.capacity;
        } else {
            self.write(_element);
        }
    }
}
