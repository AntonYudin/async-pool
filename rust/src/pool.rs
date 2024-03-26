
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::fmt;


#[derive(Debug)]
pub enum PoolError {
    AtCapacity(AtCapacityError),
    Creator(CreatorError)
}

impl std::error::Error for PoolError {}

impl fmt::Display for PoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PoolError::AtCapacity(error) => write!(f, "{}", error),
            PoolError::Creator(error) => write!(f, "{}", error)
        }
    }
}

impl From<Box<dyn std::error::Error>> for PoolError {
    fn from(error: Box<dyn std::error::Error>) -> PoolError {
        PoolError::Creator(CreatorError { wrapped: error.to_string() })
    }
}



#[derive(Debug, Clone)]
pub struct AtCapacityError {
    capacity: usize
}

impl fmt::Display for AtCapacityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pool at capacity: {}", self.capacity)
    }
}


#[derive(Debug, Clone)]
pub struct CreatorError {
    wrapped: String
}

impl fmt::Display for CreatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "creator error: {}", self.wrapped)
    }
}

impl From<Box<dyn std::error::Error>> for CreatorError {
    fn from(error: Box<dyn std::error::Error>) -> CreatorError {
        CreatorError { wrapped: error.to_string() }
    }
}


type OnDrop = Arc<dyn Fn() ->() + Send + Sync>;


pub struct Resource<T> {
    value: T,
    on_drop: OnDrop
}

impl <T> Drop for Resource<T> {
    fn drop(&mut self) {
        println!("Resource.drop()");
        (self.on_drop)();
    }
}

impl <T> Resource<T> {
    pub fn get(&self) -> &T {
        &self.value
    }
    pub fn get_as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}


pub struct Pool<T, F>
where
    F: Fn() -> Result<T, Box<dyn std::error::Error>>
{
    resources: Arc<Mutex<Vec<Resource<T>>>>,
    creator: Arc<F>,
    capacity: usize,
    size: Arc<AtomicUsize>
}

impl <T, F> Clone for Pool<T, F>
where
    F: Fn() -> Result<T, Box<dyn std::error::Error>>
{
    fn clone(&self) -> Self {
        Pool::<T, F> {
            resources: self.resources.clone(),
            creator: self.creator.clone(),
            size: self.size.clone(),
            capacity: self.capacity.clone()
        }
    }
}


impl <T, F> Pool<T, F>
where
    F: Fn() -> Result<T, Box<dyn std::error::Error>>
{
    pub fn new(capacity: usize, creator: F) -> Pool<T, F> {
        let resources: Vec<Resource<T>> = Vec::with_capacity(capacity);
        Pool {
            resources: Arc::new(Mutex::new(resources)),
            creator: Arc::new(creator),
            capacity: capacity,
            size: Arc::new(AtomicUsize::new(0))
        }
    }

    pub fn process<R, FF: FnMut(&Resource<T>) -> R>(&mut self, mut callback: FF) -> Result<R, PoolError> {
        self.process_as_mut(|resource| callback(resource))
    }

    pub fn process_as_mut<R, FF: FnMut(&mut Resource<T>) -> R>(&mut self, mut callback: FF) -> Result<R, PoolError> {

        let mut resource: Resource<T>;

        {
            let mut resources = self.resources.lock().unwrap();
            if resources.len() > 0 {
                    resource = resources.pop().unwrap();
                    println!("popped resource");
            } else if self.capacity > self.size.load(Ordering::Relaxed) {
                    resource = Resource {
                        value: (self.creator)()?,
                        on_drop: Arc::new(|| {
                               println!("resource custom on_drop");
                        })
                    };
                    self.size.fetch_add(1, Ordering::Relaxed);
                    println!("created resource");
            } else {
                return Err(PoolError::AtCapacity(AtCapacityError {capacity: self.capacity}));
            }
        }

        let result = callback(&mut resource);
        {
            let mut resources = self.resources.lock().unwrap();
            resources.push(resource);
            println!("pushed resource");
        }
        Ok(result)
    }

}

