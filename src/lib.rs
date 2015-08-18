#![feature(fnbox)]

use std::collections::VecDeque;
use std::cell::UnsafeCell;

pub struct DequeCell<T>(UnsafeCell<VecDeque<T>>);

impl<T> DequeCell<T> {
	pub fn new() -> Self {
		DequeCell(UnsafeCell::new(VecDeque::new()))
	}
	fn inner(&self) -> &mut VecDeque<T> {
		unsafe { &mut *self.0.get() }
	}
	pub fn push_back(&self, t: T) {
		self.inner().push_back(t);
	}
	pub fn push_front(&self, t: T) {
		self.inner().push_front(t);
	}
	pub fn pop_back(&self) -> Option<T> {
		self.inner().pop_back()
	}
	pub fn pop_front(&self) -> Option<T> {
		self.inner().pop_front()
	}
	pub fn len(&self) -> usize {
		self.inner().len()
	}
	pub fn capacity(&self) -> usize {
		self.inner().capacity()
	}
	pub fn reserve(&self, additional: usize) {
		self.inner().reserve(additional);
	}
	pub fn reserve_exact(&self, additional: usize) {
		self.inner().reserve_exact(additional);
	}
	pub fn is_empty(&self) -> bool {
		self.inner().is_empty()
	}
	pub fn clear(&self) {
		self.inner().clear();
	}
	pub fn drain(&self) -> Drain<T> {
		Drain { inner: self }
	}
}

pub struct Drain<'a, T: 'a> {
    inner: &'a DequeCell<T>,
}

impl<'a, T: 'a> Iterator for Drain<'a, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.inner.pop_front()
    }
}


#[cfg(test)]
mod tests {
	use super::DequeCell;
	use std::boxed::FnBox;
	
	struct Context<T> {
		actions: DequeCell<Box<FnBox(&mut Context<T>)>>,
		data: T,
	}
	
	impl<T> Context<T> {
		fn drain(&mut self) {
			while let Some(action) = self.actions.pop_front() {
				action.call_box((self,));
			}
		}
		fn new(data: T) -> Self {
			Context { actions: DequeCell::new(), data: data }
		}
	}
	
	#[test]
	fn interleaved_access() {
		let mut c = Context::new(0);
		
		
		c.actions.push_back(Box::new(|c: &mut Context<i32>| {
			assert!(c.data == 0);
			c.data += 1;
			c.actions.push_back(Box::new(|c: &mut Context<i32>| {
				assert!(c.data == 2);
				c.data += 1;
			}));
		}));
		c.actions.push_back(Box::new(|c: &mut Context<i32>| {
			assert!(c.data == 1);
			c.data += 1;
		}));
			
		c.drain();
		assert!(c.data == 3);
	}
}
