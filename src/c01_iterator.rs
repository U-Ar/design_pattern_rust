

pub struct Book {
    pub name: String
}

pub struct BookShelf {
    books: Vec<Book>
}

pub struct BookShelfIterator<'a> {
    bookshelf: &'a BookShelf,
    index: usize
}

impl<'a> BookShelfIterator<'a> {
    pub fn new(bookshelf: &'a BookShelf) -> BookShelfIterator {
        BookShelfIterator {
            bookshelf,
            index: 0
        }
    }
}

impl BookShelf {
    pub fn new() -> BookShelf {
        BookShelf { books: vec![] }
    }

    pub fn get_book_at(&self, index: usize) -> &Book {
        &self.books[index]
    }

    pub fn append_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn get_length(&self) -> usize {
        self.books.len()
    }

}

impl<'a> Iterator for BookShelfIterator<'a> {
    type Item = &'a Book;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.bookshelf.get_length() {
            let item = &self.bookshelf.get_book_at(self.index);
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}


impl<'a> IntoIterator for &'a BookShelf {
    type Item = &'a Book;
    type IntoIter = BookShelfIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BookShelfIterator::new(&self)
    }
}

