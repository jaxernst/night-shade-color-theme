using System;
using System.Collections.Generic;
using System.Linq;

public class Book
{
    public int BookId { get; set; }
    public string Title { get; set; }
    public string Author { get; set; }
    public bool IsAvailable { get; set; } = true;
}

public class Patron
{
    public int PatronId { get; set; }
    public string Name { get; set; }
    public List<Book> BorrowedBooks { get; set; } = new List<Book>();
}

public class Library
{
    private List<Book> books = new List<Book>();
    private List<Patron> patrons = new List<Patron>();

    public void AddBook(Book book)
    {
        books.Add(book);
    }

    public void RegisterPatron(Patron patron)
    {
        patrons.Add(patron);
    }

    public void BorrowBook(int bookId, int patronId)
    {
        var book = books.FirstOrDefault(b => b.BookId == bookId);
        var patron = patrons.FirstOrDefault(p => p.PatronId == patronId);

        if (book == null || patron == null)
        {
            throw new ArgumentException("Book or Patron not found.");
        }

        if (!book.IsAvailable)
        {
            throw new InvalidOperationException("Book is currently unavailable.");
        }

        book.IsAvailable = false;
        patron.BorrowedBooks.Add(book);
    }

    public void ReturnBook(int bookId, int patronId)
    {
        var book = books.FirstOrDefault(b => b.BookId == bookId);
        var patron = patrons.FirstOrDefault(p => p.PatronId == patronId);

        if (book == null || patron == null)
        {
            throw new ArgumentException("Book or Patron not found.");
        }

        if (!patron.BorrowedBooks.Remove(book))
        {
            throw new InvalidOperationException("This book was not borrowed by this patron.");
        }

        book.IsAvailable = true;
    }

    public void PrintAvailableBooks()
    {
        var availableBooks = books.Where(b => b.IsAvailable).ToList();
        if (availableBooks.Any())
        {
            Console.WriteLine("Available Books:");
            foreach (var book in availableBooks)
            {
                Console.WriteLine($"ID: {book.BookId}, Title: {book.Title}, Author: {book.Author}");
            }
        }
        else
        {
            Console.WriteLine("No available books.");
        }
    }
}

class Program
{
    static void Main()
    {
        var library = new Library();
        library.AddBook(new Book { BookId = 1, Title = "1984", Author = "George Orwell" });
        library.AddBook(new Book { BookId = 2, Title = "Brave New World", Author = "Aldous Huxley" });

        var patron = new Patron { PatronId = 1, Name = "John Doe" };
        library.RegisterPatron(patron);

        try
        {
            library.BorrowBook(1, 1);
            library.PrintAvailableBooks();
            library.ReturnBook(1, 1);
        }
        catch (Exception e)
        {
            Console.WriteLine(e.Message);
        }

        library.PrintAvailableBooks();
    }
}
