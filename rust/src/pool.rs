
use std::sync::{Arc,Mutex};


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
}




pub struct Pool<T, F: Fn() -> T> {
    resources: Arc<Mutex<Vec<Resource<T>>>>,
    creator: Arc<F>
}

impl <T, F: Fn() -> T> Clone for Pool<T, F> {
    fn clone(&self) -> Self {
        Pool::<T, F> {
            resources: self.resources.clone(),
            creator: self.creator.clone()
        }
    }
}

impl <T, F: Fn() -> T> Pool<T, F> {
    pub fn new(capacity: usize, creator: F) -> Pool<T, F>{
        let mut resources: Vec<Resource<T>> = Vec::with_capacity(capacity);
        for _i in 0..capacity {
            resources.push(
                Resource {
                    value: (creator)(),
                    on_drop: Arc::new(|| {
                           println!("resource custom on_drop");
                    })
                }
            );
        }
        Pool {
            resources: Arc::new(Mutex::new(resources)),
            creator: Arc::new(creator)
        }
    }

    pub fn process<R, FF: FnMut(&Resource<T>) -> R>(&mut self, mut callback: FF) -> Option<R> {

        let resource: Option<Resource<T>>;

        {
            let mut resources = self.resources.lock().unwrap();
            resource = resources.pop();
            println!("popped resource");
        }


        match resource {
            Some(resource) => {
                let result = Some(callback(&resource));
                {
                    let mut resources = self.resources.lock().unwrap();
                    resources.push(resource);
                    println!("pushed resource");
                }
                result
            },
            None => None
        }
    }

}

