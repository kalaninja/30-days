﻿using System;
using System.Collections.Generic;
using System.IO;

abstract class Book
{
    protected String title;
    protected String author;

    public Book(String t, String a)
    {
        title = t;
        author = a;
    }

    public abstract void display();
}

//Write MyBook class
class MyBook : Book
{
    private readonly int _price;

    public MyBook(string title, string author, int price) : base(title, author)
    {
        _price = price;
    }

    public override void display()
    {
        Console.WriteLine($"Title: {title}");
        Console.WriteLine($"Author: {author}");
        Console.WriteLine($"Price: {_price}");
    }
}


class Solution
{
    static void Main(String[] args)
    {
        String title = Console.ReadLine();
        String author = Console.ReadLine();
        int price = Int32.Parse(Console.ReadLine());
        Book new_novel = new MyBook(title, author, price);
        new_novel.display();
    }
}