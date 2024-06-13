import Foundation

enum LibraryError: Error {
    case bookNotFound
    case bookAlreadyBorrowed
    case bookNotBorrowed
}

class Book {
    var title: String
    var author: String
    var isBorrowed: Bool = false
    
    init(title: String, author: String) {
        self.title = title
        self.author = author
    }
}

class Library {
    private var books: [Book] = []
    
    func addBook(_ book: Book) {
        books.append(book)
    }
    
    func borrowBook(title: String) throws {
        guard let book = books.first(where: { $0.title == title }) else {
            throw LibraryError.bookNotFound
        }
        
        if book.isBorrowed {
            throw LibraryError.bookAlreadyBorrowed
        }
        
        book.isBorrowed = true
        print("Book borrowed: \(book.title)")
    }
    
    func returnBook(title: String) throws {
        guard let book = books.first(where: { $0.title == title }) else {
            throw LibraryError.bookNotFound
        }
        
        if !book.isBorrowed {
            throw LibraryError.bookNotBorrowed
        }
        
        book.isBorrowed = false
        print("Book returned: \(book.title)")
    }
    
    func listAvailableBooks() {
        let availableBooks = books.filter { !$0.isBorrowed }
        if availableBooks.isEmpty {
            print("No available books right now.")
        } else {
            availableBooks.forEach { print("\($0.title) by \($0.author)") }
        }
    }
}

// Usage
let library = Library()
library.addBook(Book(title: "1984", author: "George Orwell"))
library.addBook(Book(title: "Brave New World", author: "Aldous Huxley"))

do {
    try library.borrowBook(title: "1984")
    try library.returnBook(title: "1984")
    library.listAvailableBooks()
} catch LibraryError.bookNotFound {
    print("The book you requested does not exist.")
} catch LibraryError.bookAlreadyBorrowed {
    print("The book has already been borrowed.")
} catch LibraryError.bookNotBorrowed {
    print("The book was not borrowed.")
} catch {
    print("An unexpected error occurred.")
}
