pub struct Book<T: Clone> {
    pub items: Vec<T>,
    pub itemcount: i32,
}

impl<T: Clone> Book<T> {
    pub fn new(items: Vec<T>) -> Self {
        Book { items: items.clone(), itemcount: items.len() as i32 }
    }

    pub fn get_page(&self, page: i32, page_size: i32) -> Vec<T> {
        let start = (page - 1) * page_size;
        let end = start + page_size;
        self.items[start as usize..end as usize].to_vec()
    }

    pub fn get_page_count(&self, page_size: i32) -> i32 {
        (self.itemcount as f64 / page_size as f64).ceil() as i32
    }

    pub fn change_page_count(&mut self, page_size: i32) {
        self.itemcount = self.get_page_count(page_size);
    }

    pub fn add_item(&mut self, item: T) {
        self.items.push(item);
        self.itemcount += 1;
    }

    pub fn page_limit(&mut self, pagecount: i32) -> i32 {
        ((self.itemcount as f64) / (pagecount as f64)).ceil() as i32
    }

    pub fn process_book(&mut self, page_size: i32) -> ProceesedBook<T> {
        let pages = self.get_page_count(page_size);
        ProceesedBook {
            items: self.items.chunks(page_size as usize).map(|x| x.to_vec()).collect(),
            pages,
            page_size,
            current_page: 0,
        }
    }
}

pub struct ProceesedBook<T> where T: Clone {
    pub items: Vec<Vec<T>>,
    pub pages: i32,
    pub page_size: i32,
    pub current_page: i32,
}

impl<T: Clone> Iterator for ProceesedBook<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_page < self.pages {
            let page = self.items[self.current_page as usize].clone();
            self.current_page += 1;
            Some(page)
        } else {
            None
        }
    }
}